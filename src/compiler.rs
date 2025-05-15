use crate::lexer::{Lexer, TokenType};
use crate::parser::{Expr, FunctionParam, Parser, Stmt};
use crate::error_reporting::LutError;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::rc::Rc;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue, InstructionValue};
use inkwell::types::IntType;
use inkwell::OptimizationLevel;
use inkwell::AddressSpace;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine
};
use inkwell::Either;

// Define an enum for variable types
#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Integer,
    Float,
    String,
    Boolean,
    Array,
    Array2D,
}

// String interning pool for efficient string management
struct StringPool<'ctx> {
    // Store unique strings with reference counting
    pool: HashSet<Rc<String>>,
    // Map of global string constants in LLVM IR
    global_strings: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> StringPool<'ctx> {
    fn new() -> Self {
        StringPool {
            pool: HashSet::new(),
            global_strings: HashMap::new(),
        }
    }
    
    fn intern(&mut self, s: String) -> Rc<String> {
        let rc_string = Rc::new(s);
        // If the string is already in the pool, return the existing reference
        if let Some(existing) = self.pool.get(&rc_string) {
            Rc::clone(existing)
        } else {
            // Otherwise add it to the pool and return a reference
            self.pool.insert(Rc::clone(&rc_string));
            rc_string
        }
    }
    
    // Get or create a global string constant in LLVM IR
    fn get_or_create_global_string(&mut self, 
                                   string_val: &str, 
                                   context: &'ctx Context, 
                                   module: &Module<'ctx>, 
                                   builder: &Builder<'ctx>) -> PointerValue<'ctx> {
        // Check if we already have this string in the global strings cache
        if let Some(&ptr) = self.global_strings.get(string_val) {
            return ptr;
        }
        
        // Create a new global string constant
        let i8_type = context.i8_type();
        let string_type = i8_type.array_type((string_val.len() + 1) as u32);
        
        // Create a unique name for the global string
        let global_name = format!("str_{}", module.get_globals().count());
        let global_string = module.add_global(string_type, None, &global_name);
        global_string.set_constant(true);
        global_string.set_linkage(inkwell::module::Linkage::Private);
        global_string.set_initializer(&context.const_string(string_val.as_bytes(), true));
        
        // Create a pointer to the string data
        let zero = context.i32_type().const_zero();
        let indices = [zero, zero];
        let ptr = unsafe {
            builder.build_gep(i8_type, global_string.as_pointer_value(), &indices, "str_ptr").unwrap()
        };
        
        // Cache and return the pointer
        self.global_strings.insert(string_val.to_string(), ptr);
        ptr
    }
}

// LLVM Code generator
pub struct LLVMCompiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
    variable_types: HashMap<String, VariableType>,
    printf_func: FunctionValue<'ctx>,
    i64_type: IntType<'ctx>,
    // String handling functions
    sprintf_func: FunctionValue<'ctx>,
    atoll_func: FunctionValue<'ctx>,
    malloc_func: FunctionValue<'ctx>,
    strlen_func: FunctionValue<'ctx>,
    // Loop control flow tracking
    current_loop_exit: Option<inkwell::basic_block::BasicBlock<'ctx>>,
    current_loop_continue: Option<inkwell::basic_block::BasicBlock<'ctx>>,
    // String interning pool
    string_pool: StringPool<'ctx>,
    // Pattern detection flags
    is_counting_loop: bool,
    // Silent mode for optimized output
    silent_mode: bool,
    // Function tracking
    functions: HashMap<String, FunctionValue<'ctx>>,
    // Current function for return statements
    current_function: Option<FunctionValue<'ctx>>,
    // Source file path (for special case handling)
    file_path: String,
}

impl<'ctx> LLVMCompiler<'ctx> {
    // Helper function to handle LLVM errors more gracefully
    fn handle_llvm_err<T, E>(&self, result: Result<T, E>, operation: &str) -> Result<T, LutError>
    where
        E: std::fmt::Display
    {
        result.map_err(|e| LutError::compiler_error(
            format!("LLVM error during {}: {}", operation, e),
            None
        ))
    }

    pub fn new(context: &'ctx Context, module_name: &str, file_path: &str) -> Self {
        // Initialize LLVM targets
        Target::initialize_all(&InitializationConfig::default());
        
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let i64_type = context.i64_type();
        
        // Create external functions
        let i8_ptr_type = context.ptr_type(AddressSpace::default());
        let i32_type = context.i32_type();
        
        // printf function declaration
        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
        let printf_func = module.add_function("printf", printf_type, None);
        
        // sprintf function declaration (char* dest, char* format, ...)
        let sprintf_type = i32_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], true);
        let sprintf_func = module.add_function("sprintf", sprintf_type, None);
        
        // atoll function declaration (char* str)
        let atoll_type = i64_type.fn_type(&[i8_ptr_type.into()], false);
        let atoll_func = module.add_function("atoll", atoll_type, None);
        
        // malloc function declaration (size_t size)
        let malloc_type = i8_ptr_type.fn_type(&[i64_type.into()], false);
        let malloc_func = module.add_function("malloc", malloc_type, None);
        
        // strlen function declaration (char* str)
        let strlen_type = i64_type.fn_type(&[i8_ptr_type.into()], false);
        let strlen_func = module.add_function("strlen", strlen_type, None);
        
        // Create string pool
        let string_pool = StringPool::new();
        
        LLVMCompiler {
            context,
            module,
            builder,
            variables: HashMap::with_capacity(128), // Pre-allocate space for variables
            variable_types: HashMap::with_capacity(128),
            printf_func,
            i64_type,
            sprintf_func,
            atoll_func,
            malloc_func,
            strlen_func,
            current_loop_exit: None,
            current_loop_continue: None,
            string_pool,
            is_counting_loop: false,
            silent_mode: false,
            functions: HashMap::with_capacity(32), // Pre-allocate space for functions
            current_function: None,
            file_path: file_path.to_string(),
        }
    }
    
    // Set silent mode for optimized output
    pub fn with_silent_mode(mut self, silent: bool) -> Self {
        self.silent_mode = silent;
        self
    }
    
    // Create an entry point
    pub fn create_main_function(&mut self) -> FunctionValue<'ctx> {
        // First, we need to check if there's a user main function
        let has_user_main = self.functions.contains_key("main");
        
        // If there's already a user-defined main function, we need to handle it specially
        let user_main_func = if has_user_main {
            // Get the existing main function
            let user_main = self.functions.remove("main").unwrap();
            
            // Add it back with a different name (user_main)
            self.functions.insert("user_main".to_string(), user_main);
            Some(user_main)
        } else {
            None
        };
        
        // Create the system main function (i64 return type for consistency)
        let main_type = self.i64_type.fn_type(&[], false);
        let main_func = self.module.add_function("main", main_type, None);
        
        // Create a single entry block
        let entry = self.context.append_basic_block(main_func, "entry");
        self.builder.position_at_end(entry);
        
        // Create a dummy variable to avoid empty blocks (required by LLVM)
        self.builder.build_alloca(self.i64_type, "dummy").unwrap();
        
        // If there's a user main function, call it and return its result
        if let Some(user_main) = user_main_func {
            // Add a call to the user_main function
            let call = self.builder.build_call(user_main, &[], "user_main_call").unwrap();
            
            // Extract the return value
            let return_value = match call.try_as_basic_value() {
                Either::Left(value) => value.into_int_value(),
                Either::Right(_) => self.i64_type.const_int(0, false)
            };
            
            // Return the result
            self.builder.build_return(Some(&return_value)).unwrap();
        } else {
            // No user main, just return 0
            let return_value = self.i64_type.const_int(0, false);
            self.builder.build_return(Some(&return_value)).unwrap();
        }
        
        main_func
    }
    
    // Create a print function that will use printf
    fn create_print_string(&mut self, text: &str) {
        // Get a string literal pointer
        let str_ptr = self.create_string_literal(text);
        
        // Create a unique call ID
        let call_id = format!("printf_call_{}", self.module.get_globals().count());
        
        // Call printf with the format string
        self.builder.build_call(self.printf_func, &[str_ptr.into()], &call_id).unwrap();
    }
    
    // Compile all statements and create a binary
    pub fn compile(&mut self, statements: Vec<Stmt>) -> Result<(), LutError> {
        // First pass: register all function declarations (including main)
        for stmt in statements.iter() {
            if let Stmt::Function { name, is_public, parameters, .. } = stmt {
                // Create parameter types
                let mut param_types = Vec::with_capacity(parameters.len());
                for _ in parameters {
                    // All parameters are i64 for now
                    param_types.push(self.i64_type.into());
                }
                
                // Create function type
                let function_type = self.i64_type.fn_type(&param_types, false);
                
                // Create the function without body
                let linkage = if *is_public {
                    inkwell::module::Linkage::External
                } else {
                    inkwell::module::Linkage::Private
                };
                
                // Just declare the function in the first pass
                let function = self.module.add_function(name, function_type, Some(linkage));
                
                // Register the function so it can be referenced before definition
                self.functions.insert(name.clone(), function);
            }
        }
        
        // Create system main after user functions are registered
        let main_func = self.create_main_function();
        
        // Set insertion point to main function entry block
        let entry_block = main_func.get_first_basic_block().unwrap();
        self.builder.position_at_end(entry_block);
        
        // Check if user defined a main function
        let has_user_main = self.functions.contains_key("main");
        
        // Second pass: compile all functions
        for stmt in statements {
            self.compile_statement(stmt)?;
        }
        
        // First, let's manually find the main function in our LLVM module
        // We need to ensure the main function itself is properly terminated
        if let Some(main_func) = self.module.get_function("main") {
            let blocks = main_func.get_basic_blocks();
            for block in blocks {
                // If this block doesn't have a terminator, add one
                if !block.get_terminator().is_some() {
                    self.builder.position_at_end(block);
                    
                    // If this is the entry block, add a return 0
                    if block.get_name().to_str().unwrap_or("") == "entry" {
                        // Add a return statement to the entry block
                        let return_value = self.i64_type.const_int(0, false);
                        self.builder.build_return(Some(&return_value)).unwrap();
                    } else {
                        // For other blocks, find some block to branch to or add a return
                        let entry = main_func.get_first_basic_block().unwrap();
                        self.builder.build_unconditional_branch(entry).unwrap();
                    }
                }
            }
        }
        
        // Verify all other functions are correctly formed
        for (name, func) in self.functions.clone() {
            // Check and fix unterminated blocks
            let blocks = func.get_basic_blocks();
            for block in blocks {
                if !block.get_terminator().is_some() {
                    // This block needs a terminator - position at the end and add a return
                    self.builder.position_at_end(block);
                    let return_value = self.i64_type.const_int(0, false);
                    self.builder.build_return(Some(&return_value)).unwrap();
                }
            }
        }
        
        // Before verifying, let's clean up any unused function declarations
        // Remove main.1 and main.2 declarations if they exist
        for func_name in ["main.1", "main.2"] {
            if let Some(decl) = self.module.get_function(func_name) {
                // Check if it's just a declaration (no body)
                if decl.get_basic_blocks().is_empty() {
                    // It's just a declaration, we can safely remove it
                    unsafe {
                        decl.delete();
                    }
                }
            }
        }
        
        // Make sure the main function calls user_main if needed
        if let Some(main_func) = self.module.get_function("main") {
            // Get or create the entry block
            let entry_block = match main_func.get_first_basic_block() {
                Some(block) => block,
                None => {
                    // Create an entry block if it doesn't exist
                    self.context.append_basic_block(main_func, "entry")
                }
            };
            
            // Position at the end of the entry block
            self.builder.position_at_end(entry_block);
            
            // Check if the entry block already has a terminator
            if entry_block.get_terminator().is_none() {
                // Create a dummy variable if needed
                if main_func.get_params().is_empty() && main_func.get_basic_blocks().len() <= 1 {
                    self.builder.build_alloca(self.i64_type, "dummy").unwrap();
                }
                
                // Check for user_main function
                if let Some(user_main) = self.functions.get("user_main") {
                    // Call the user_main function and return its result
                    let call = self.builder.build_call(*user_main, &[], "main_call").unwrap();
                    
                    // Extract return value
                    let return_value = match call.try_as_basic_value() {
                        Either::Left(value) => value.into_int_value(),
                        Either::Right(_) => self.i64_type.const_int(0, false)
                    };
                    
                    // Return the result
                    self.builder.build_return(Some(&return_value)).unwrap();
                } else {
                    // No user_main, just return 0
                    let return_value = self.i64_type.const_int(0, false);
                    self.builder.build_return(Some(&return_value)).unwrap();
                }
            }
        }
        
        // Always verify modules to ensure correctness
        if let Err(err) = self.module.verify() {
            // Print the generated IR to help debug the verification error
            if let Some(filename) = std::path::Path::new(&self.file_path).file_name() {
                if filename.to_string_lossy().contains("benchmark") || 
                   filename.to_string_lossy().contains("factorial") {
                    println!("Module verification failed for {}", self.file_path);
                    println!("Error: {}", err);
                    
                    // Save the problematic IR for debugging
                    let debug_file = format!("{}.debug.ll", self.file_path);
                    let _ = self.module.print_to_file(&debug_file);
                    println!("Saved debug IR to {}", debug_file);
                }
            }
            
            return Err(LutError::compiler_error(
                format!("Module verification error: {}. This may indicate a type mismatch or malformed IR.", 
                        err.to_string()),
                None
            ));
        }
        
        Ok(())
    }
    
    // Compile a function definition
    fn compile_function(&mut self, name: String, is_public: bool, parameters: Vec<FunctionParam>, body: Vec<Stmt>) -> Result<FunctionValue<'ctx>, LutError> {
        // Special case for main function
        let is_main_function = name == "main";
        
        // For the main function, we want to use "user_main" instead
        let function_name = if is_main_function {
            "user_main".to_string()
        } else {
            name.clone()
        };
        
        // Get the function if it's already declared (from first pass)
        let function = if let Some(func) = self.functions.get(&function_name) {
            *func
        } else {
            // If function wasn't pre-registered in the first pass, create it now
            // Create parameter types
            let mut param_types = Vec::with_capacity(parameters.len());
            for _ in &parameters {
                // All parameters are i64 for now (we can add more types later)
                param_types.push(self.i64_type.into());
            }
            
            // Create function type
            let function_type = self.i64_type.fn_type(&param_types, false);
            
            // Create the function
            let linkage = if is_public {
                inkwell::module::Linkage::External
            } else {
                inkwell::module::Linkage::Private
            };
            
            // Create the function with the appropriate name
            let func = self.module.add_function(&function_name, function_type, Some(linkage));
            
            // Add to functions map using the appropriate name
            self.functions.insert(function_name.clone(), func);
            func
        };
        
        // Create entry block
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
        
        // Save current function
        let old_function = self.current_function;
        self.current_function = Some(function);
        
        // Store parameters in local variables
        let mut old_variables = HashMap::new();
        for (i, param) in parameters.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            
            // Create a local variable for the parameter
            let alloca = self.create_entry_block_alloca(&param.name);
            
            // Store parameter - let LLVM handle alignment automatically
            // Critical: We need consistent alignment for all store operations
            let _store_inst = self.builder.build_store(alloca, param_value).unwrap();
            
            // Save old variable with the same name if it exists
            if let Some(old_ptr) = self.variables.get(&param.name) {
                old_variables.insert(param.name.clone(), *old_ptr);
            }
            
            // Add the parameter to our variables
            self.variables.insert(param.name.clone(), alloca);
            
            // Add the parameter type
            self.variable_types.insert(param.name.clone(), VariableType::Integer);
        }
        
        // Compile function body
        let mut return_value = None;
        for (i, stmt) in body.iter().enumerate() {
            // Check if this is the last statement and if it's an expression
            if i == body.len() - 1 {
                if let Stmt::Expression(expr) = stmt {
                    // Special case: handle 'ok' as a return value (equivalent to 0)
                    if let Expr::TextLiteral(s) = &expr {
                        if s == "ok" {
                            // 'ok' is equivalent to returning 0
                            let zero_value = self.i64_type.const_int(0, false).into();
                            return_value = Some(zero_value);
                        } else {
                            // For other text literals, compile as normal
                            let value = self.compile_expression(expr.clone())?;
                            return_value = Some(value);
                        }
                    } else {
                        // Not a text literal, compile as normal
                        let value = self.compile_expression(expr.clone())?;
                        return_value = Some(value);
                    }
                } else {
                    self.compile_statement(stmt.clone())?;
                }
            } else {
                self.compile_statement(stmt.clone())?;
            }
        }
        
        // Ensure the current basic block has a terminator
        let current_block = self.builder.get_insert_block().unwrap();
        if !current_block.get_terminator().is_some() {
            // If we have a return value from the last expression, return it
            if let Some(val) = return_value {
                // Convert to the appropriate type for return
                match val {
                    BasicValueEnum::IntValue(int_val) => {
                        self.builder.build_return(Some(&int_val)).unwrap();
                    },
                    _ => {
                        // Default to returning 0 for other types
                        let default_return = self.i64_type.const_int(0, false);
                        self.builder.build_return(Some(&default_return)).unwrap();
                    }
                }
            } else {
                // Add a default return of 0
                let default_return = self.i64_type.const_int(0, false);
                self.builder.build_return(Some(&default_return)).unwrap();
            }
        }
        
        // Check all basic blocks have terminators
        for block in function.get_basic_blocks() {
            if !block.get_terminator().is_some() {
                // Position at the end of the unterminated block
                self.builder.position_at_end(block);
                // Add a branch to the next block or a return if it's the last block
                let next_block = block.get_next_basic_block();
                if let Some(next) = next_block {
                    self.builder.build_unconditional_branch(next).unwrap();
                } else {
                    // This is the last block, add a return
                    let return_value = self.i64_type.const_int(0, false);
                    self.builder.build_return(Some(&return_value)).unwrap();
                }
            }
        }
        
        // Verify the function
        if function.verify(true) {
            // Function is already in the table from either first pass or early in this function
            
            // Restore saved variables
            for (name, ptr) in old_variables {
                self.variables.insert(name, ptr);
            }
            
            // Restore old function
            self.current_function = old_function;
            
            Ok(function)
        } else {
            // Remove the function if verification fails
            unsafe {
                function.delete();
            }
            Err(LutError::compiler_error("Function verification failed", None))
        }
    }
    
    fn compile_statement(&mut self, stmt: Stmt) -> Result<(), LutError> {
        match stmt {
            Stmt::Function { name, is_public, parameters, body } => {
                self.compile_function(name, is_public, parameters, body)?;
            },
            Stmt::Declaration { name, initializer } => {
                // Check if this is a variable update (name already exists)
                if self.variables.contains_key(&name) {
                    // This is a variable update, not a declaration
                    let value = self.compile_expression(initializer)?;
                    if let Some(ptr) = self.variables.get(&name) {
                        // Store the new value in the existing variable
                        self.builder.build_store(*ptr, value).unwrap();
                    } else {
                        return Err(LutError::compiler_error(
                            format!("Variable '{}' referenced before declaration", name),
                            None
                        ));
                    }
                } else {
                    // This is a new variable declaration
                    // Check the initializer type before moving it
                    let is_boolean_expr = match &initializer {
                        Expr::BooleanLiteral(_) => true,
                        Expr::Binary { operator, .. } => {
                            // Check if this is a comparison operator
                            matches!(
                                operator.token_type,
                                TokenType::Equal | TokenType::NotEqual | TokenType::Less |
                                TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual |
                                TokenType::And | TokenType::Or
                            )
                        },
                        _ => false
                    };

                    // Create a variable (alloca) in the entry block
                    let value = self.compile_expression(initializer)?;

                    // Create the appropriate type of alloca based on the value type
                    let (ptr, var_type) = match value {
                        BasicValueEnum::IntValue(_) => {
                            // For both integers and booleans, use i64 type
                            let ptr = self.create_entry_block_alloca(&name);

                            if is_boolean_expr {
                                (ptr, VariableType::Boolean)
                            } else {
                                (ptr, VariableType::Integer)
                            }
                        },
                        BasicValueEnum::PointerValue(_) => {
                            // For strings, use pointer type
                            let ptr = self.create_pointer_alloca(&name);
                            (ptr, VariableType::String)
                        },
                        _ => return Err(LutError::compiler_error("Unsupported variable type", None))
                    };

                    let _store_inst = self.builder.build_store(ptr, value).unwrap(); // Ignoring the result
                    // Let LLVM handle alignment automatically
                    self.variables.insert(name.clone(), ptr);
                    self.variable_types.insert(name, var_type);
                }
            },
            Stmt::Expression(expr) => {
                // Just evaluate the expression for its side effects
                self.compile_expression(expr)?;
            },
            Stmt::Command { name, args } => {
                match name.as_str() {
                    "print" | "-print" => {
                        for (i, arg) in args.iter().enumerate() {
                            let value = self.compile_expression(arg.clone())?;
                            self.print_value(value)?;
                            
                            // Print a space between arguments (but not after the last one)
                            if i < args.len() - 1 {
                                // Do nothing in this simple version
                            }
                        }
                        // Print newline
                        self.create_print_string("\\n");
                    },
                    _ => return Err(LutError::compiler_error(
                        format!("Unknown command in LLVM compiler: {}", name),
                        None
                    )),
                }
            },
            Stmt::Print(exprs) => {
                for (i, expr) in exprs.iter().enumerate() {
                    let value = self.compile_expression(expr.clone())?;
                    self.print_value(value)?;
                    
                    // Print a space between arguments (but not after the last one)
                    if i < exprs.len() - 1 {
                        // Do nothing in this simple version
                    }
                }
                // Print newline
                self.create_print_string("\\n");
            },
            Stmt::Comment(_) => {
                // Comments are ignored in the compiled output
            },
            Stmt::If { condition, then_branch, else_branch } => {
                // Get the current function
                let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();

                // Create blocks for the then, else, and merge parts
                let then_block = self.context.append_basic_block(current_function, "then");
                let else_block = self.context.append_basic_block(current_function, "else");
                let merge_block = self.context.append_basic_block(current_function, "ifcont");

                // Compile the condition
                let condition_value = self.compile_expression(condition)?;

                // Convert the condition to a boolean value (0 or 1)
                let condition_value = match condition_value {
                    BasicValueEnum::IntValue(int_val) => {
                        // Compare with 0 to get a boolean value (0 = false, anything else = true)
                        let zero = self.i64_type.const_int(0, false);
                        self.builder.build_int_compare(
                            inkwell::IntPredicate::NE,
                            int_val,
                            zero,
                            "ifcond"
                        ).unwrap()
                    },
                    _ => return Err(LutError::compiler_error("Expected integer condition in if statement", None))
                };

                // Create the conditional branch instruction based on the condition
                self.builder.build_conditional_branch(condition_value, then_block, else_block).unwrap();

                // Build the then block
                self.builder.position_at_end(then_block);

                // Compile all statements in the then branch
                let mut then_result = None;
                for (i, stmt) in then_branch.iter().enumerate() {
                    // If this is the last statement in an expression context, treat it as return value
                    if i == then_branch.len() - 1 {
                        if let Stmt::Expression(expr) = stmt {
                            // This is a terminating expression that should be treated as the return value
                            let value = self.compile_expression(expr.clone())?;
                            then_result = Some(value);
                        } else {
                            self.compile_statement(stmt.clone())?;
                        }
                    } else {
                        self.compile_statement(stmt.clone())?;
                    }
                }

                // Branch to the merge block
                self.builder.build_unconditional_branch(merge_block).unwrap();

                // Remember the current block to handle nested ifs properly
                let then_end_block = self.builder.get_insert_block().unwrap();

                // Build the else block
                self.builder.position_at_end(else_block);

                // Compile all statements in the else branch if it exists
                let mut else_result = None;
                if let Some(else_statements) = else_branch {
                    for (i, stmt) in else_statements.iter().enumerate() {
                        // If this is the last statement in an expression context, treat it as return value
                        if i == else_statements.len() - 1 {
                            if let Stmt::Expression(expr) = stmt {
                                // This is a terminating expression that should be treated as the return value
                                let value = self.compile_expression(expr.clone())?;
                                else_result = Some(value);
                            } else {
                                self.compile_statement(stmt.clone())?;
                            }
                        } else {
                            self.compile_statement(stmt.clone())?;
                        }
                    }
                }

                // Branch to the merge block
                self.builder.build_unconditional_branch(merge_block).unwrap();

                // Remember the current block
                let else_end_block = self.builder.get_insert_block().unwrap();

                // Set the insertion point to the merge block for subsequent code
                self.builder.position_at_end(merge_block);
                
                // If we have result values from either branch, create a PHI node
                let phi_result = if then_result.is_some() || else_result.is_some() {
                    if let (Some(then_val), Some(else_val)) = (then_result, else_result) {
                        if let (BasicValueEnum::IntValue(then_int), BasicValueEnum::IntValue(else_int)) = (then_val, else_val) {
                            // Create a PHI node for the result
                            let phi = self.builder.build_phi(self.i64_type, "ifresult").unwrap();
                            
                            // Add the incoming values from both branches
                            phi.add_incoming(&[
                                (&then_int, then_end_block),
                                (&else_int, else_end_block)
                            ]);
                            
                            // Return the PHI result as a value
                            Some(phi.as_basic_value())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };
                
                // If we're in an expression context, this PHI node value should be returned
                if let Some(current_func) = self.current_function {
                    if let Some(phi_val) = phi_result {
                        // We're in a function and have a result value, so return it
                        let current_block = self.builder.get_insert_block().unwrap();
                        if !current_block.get_terminator().is_some() {
                            match phi_val {
                                BasicValueEnum::IntValue(int_val) => {
                                    self.builder.build_return(Some(&int_val)).unwrap();
                                },
                                _ => {
                                    // Default to returning 0 for other types
                                    let default_return = self.i64_type.const_int(0, false);
                                    self.builder.build_return(Some(&default_return)).unwrap();
                                }
                            }
                        }
                    }
                }
            },
            Stmt::While { condition, body } => {
                // Get the current function
                let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();

                // Check for counting loop pattern - this is similar to what we did in the interpreter
                let mut is_counting_loop = false;
                let mut counter_var_name = String::new();
                let mut counter_limit = 0;
                let mut counting_up = true; // true = count up, false = count down
                
                // Look for the pattern "$count < 1000000" or similar
                if let Expr::Binary { left, operator, right } = &condition {
                    let op_type = &operator.token_type;
                    if let (Expr::VariableRef(var_name), Expr::NumberLiteral(limit)) = (&**left, &**right) {
                        if var_name.starts_with('$') &&
                           (*op_type == TokenType::Less || *op_type == TokenType::LessEqual) {
                            // Potential counting up loop
                            counter_var_name = var_name[1..].to_string();
                            counter_limit = *limit;
                            counting_up = true;
                            
                            // Now check if body matches pattern
                            if body.len() == 2 {
                                // Check for increment pattern: count : $count + 1
                                if let Stmt::Declaration { name, initializer } = &body[0] {
                                    if name == &counter_var_name {
                                        if let Expr::Binary { left: l, operator: op, right: r } = initializer {
                                            if let (Expr::VariableRef(vname), Expr::NumberLiteral(inc)) = (&**l, &**r) {
                                                if vname == var_name && op.token_type == TokenType::Plus && *inc == 1 {
                                                    // Check for print statement
                                                    if matches!(&body[1], Stmt::Print(exprs) if exprs.len() == 1 &&
                                                            matches!(&exprs[0], Expr::VariableRef(vname) if vname == var_name)) ||
                                                       matches!(&body[1], Stmt::Command { name, args } if
                                                            name == "print" && args.len() == 1 &&
                                                            matches!(&args[0], Expr::VariableRef(vname) if vname == var_name)) {
                                                        is_counting_loop = true;
                                                        self.is_counting_loop = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                if is_counting_loop && !self.silent_mode {
                    // Generate optimized counting loop
                    println!("Generating optimized counting loop...");
                    
                    // Get counter variable
                    let counter_ptr = *self.variables.get(&counter_var_name).unwrap();
                    
                    // Get the initial counter value
                    let counter_val = self.builder.build_load(self.i64_type, counter_ptr, "counter").unwrap()
                        .into_int_value();
                        
                    // Create the end condition block, loop body, and exit blocks
                    let header_block = self.context.append_basic_block(current_function, "count_header");
                    let body_block = self.context.append_basic_block(current_function, "count_body");
                    let exit_block = self.context.append_basic_block(current_function, "count_exit");
                    
                    // Branch to the header block
                    self.builder.build_unconditional_branch(header_block).unwrap();
                    self.builder.position_at_end(header_block);
                    
                    // Create the loop counter PHI node
                    let counter_phi = self.builder.build_phi(self.i64_type, "count_iter").unwrap();
                    // Add the initial value
                    counter_phi.add_incoming(&[(&counter_val, self.builder.get_insert_block().unwrap())]);
                    
                    // Compare with the limit
                    let limit_val = self.i64_type.const_int(counter_limit as u64, false);
                    let cmp_result = if counting_up {
                        self.builder.build_int_compare(
                            inkwell::IntPredicate::SLT, // <
                            counter_phi.as_basic_value().into_int_value(),
                            limit_val,
                            "count_cmp"
                        ).unwrap()
                    } else {
                        self.builder.build_int_compare(
                            inkwell::IntPredicate::SGT, // >
                            counter_phi.as_basic_value().into_int_value(),
                            limit_val,
                            "count_cmp"
                        ).unwrap()
                    };
                    
                    // Conditional branch to either body or exit
                    self.builder.build_conditional_branch(cmp_result, body_block, exit_block).unwrap();
                    
                    // Build the loop body block
                    self.builder.position_at_end(body_block);
                    
                    // Save the old loop exit and continue blocks (for nested loops)
                    let old_loop_exit = self.current_loop_exit;
                    let old_loop_continue = self.current_loop_continue;
                    
                    // Update the current loop exit and continue blocks
                    self.current_loop_exit = Some(exit_block);
                    self.current_loop_continue = Some(header_block); // Continue goes back to header
                    
                    // Print the current counter value
                    let counter_val = counter_phi.as_basic_value();
                    self.print_value(counter_val)?;
                    
                    // Increment the counter
                    let increment = self.i64_type.const_int(1, false);
                    let next_counter = if counting_up {
                        self.builder.build_int_add(
                            counter_phi.as_basic_value().into_int_value(),
                            increment,
                            "next_counter"
                        ).unwrap()
                    } else {
                        self.builder.build_int_sub(
                            counter_phi.as_basic_value().into_int_value(),
                            increment,
                            "next_counter"
                        ).unwrap()
                    };
                    
                    // Add the incremented counter to the PHI node
                    counter_phi.add_incoming(&[(&next_counter, self.builder.get_insert_block().unwrap())]);
                    
                    // Branch back to the header
                    self.builder.build_unconditional_branch(header_block).unwrap();
                    
                    // Store the final counter value in the variable
                    self.builder.position_at_end(exit_block);
                    // Store counter in the variable
                    let final_counter_val = if counting_up {
                        // At exit, counter equals limit
                        limit_val
                    } else {
                        // At exit for counting down, counter equals limit - 1
                        self.builder.build_int_sub(
                            limit_val,
                            self.i64_type.const_int(1, false),
                            "final_counter"
                        ).unwrap()
                    };
                    self.builder.build_store(counter_ptr, final_counter_val).unwrap();
                    
                    // Restore the old loop exit and continue blocks
                    self.current_loop_exit = old_loop_exit;
                    self.current_loop_continue = old_loop_continue;
                } else {
                    // Use the standard loop implementation
                    // Create basic blocks for the loop
                    let condition_block = self.context.append_basic_block(current_function, "while_cond");
                    let body_block = self.context.append_basic_block(current_function, "while_body");
                    let exit_block = self.context.append_basic_block(current_function, "while_exit");

                    // Branch to the condition block
                    self.builder.build_unconditional_branch(condition_block).unwrap();

                    // Start with the condition block
                    self.builder.position_at_end(condition_block);

                    // Compile the condition
                    let condition_value = self.compile_expression(condition.clone())?;

                    // Convert to boolean (0 or 1)
                    let condition_bool = match condition_value {
                        BasicValueEnum::IntValue(int_val) => {
                            // Compare with 0 to get a boolean value (0 = false, anything else = true)
                            let zero = self.i64_type.const_int(0, false);
                            self.builder.build_int_compare(
                                inkwell::IntPredicate::NE,
                                int_val,
                                zero,
                                "while_cond"
                            ).unwrap()
                        },
                        _ => return Err(LutError::compiler_error("Expected integer condition in while loop", None))
                    };

                    // Conditional branch: if condition is true, go to body, otherwise exit
                    self.builder.build_conditional_branch(condition_bool, body_block, exit_block).unwrap();

                    // Set up the loop body block
                    self.builder.position_at_end(body_block);

                    // Save the old loop exit and continue blocks (for nested loops)
                    let old_loop_exit = self.current_loop_exit;
                    let old_loop_continue = self.current_loop_continue;

                    // Update the current loop exit and continue blocks
                    self.current_loop_exit = Some(exit_block);
                    self.current_loop_continue = Some(condition_block); // Continue goes back to condition

                    // Compile the loop body
                    for stmt in body {
                        self.compile_statement(stmt)?;
                    }

                    // Restore the old loop exit and continue blocks
                    self.current_loop_exit = old_loop_exit;
                    self.current_loop_continue = old_loop_continue;

                    // Unconditionally branch back to the condition block
                    self.builder.build_unconditional_branch(condition_block).unwrap();

                    // Position at the exit block for subsequent code
                    self.builder.position_at_end(exit_block);
                }
            },
            Stmt::For { initializer, update, condition, body } => {
                // Get the current function
                let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();

                // Create the initialization block
                let init_block = self.context.append_basic_block(current_function, "for_init");

                // Create basic blocks for the loop
                let condition_block = self.context.append_basic_block(current_function, "for_cond");
                let body_block = self.context.append_basic_block(current_function, "for_body");
                let update_block = self.context.append_basic_block(current_function, "for_update");
                let exit_block = self.context.append_basic_block(current_function, "for_exit");

                // Branch to the initialization block
                self.builder.build_unconditional_branch(init_block).unwrap();

                // Set up the initialization block
                self.builder.position_at_end(init_block);

                // Special handling for initializer to support variable declarations
                match &initializer {
                    Expr::Binary { left, operator, right } => {
                        // Check if this looks like a declaration (i : 0)
                        if operator.token_type == TokenType::Colon {
                            if let Expr::VariableRef(name) = &**left {
                                // This is a variable declaration - evaluate right side and create variable
                                let value = self.compile_expression(*right.clone())?;

                                // Create a variable (alloca) in the entry block
                                // Infer the type from the right side expression
                                let (ptr, var_type) = match value {
                                    BasicValueEnum::IntValue(_) => {
                                        // Regular integer value
                                        let ptr = self.create_entry_block_alloca(name);
                                        (ptr, VariableType::Integer)
                                    },
                                    BasicValueEnum::PointerValue(_) => {
                                        // For strings, use pointer type
                                        let ptr = self.create_pointer_alloca(name);
                                        (ptr, VariableType::String)
                                    },
                                    _ => return Err(LutError::compiler_error("Unsupported variable type", None))
                                };

                                // Store the value in the variable
                                let _store_inst = self.builder.build_store(ptr, value).unwrap(); // Ignoring the result
                    // Let LLVM handle alignment automatically

                                // Add the variable to our variable maps
                                self.variables.insert(name.clone(), ptr);
                                self.variable_types.insert(name.clone(), var_type);
                            } else {
                                // Just evaluate it normally
                                self.compile_expression(initializer.clone())?;
                            }
                        } else {
                            // Just evaluate it normally
                            self.compile_expression(initializer.clone())?;
                        }
                    }
                    _ => {
                        // Just evaluate it normally
                        self.compile_expression(initializer.clone())?;
                    }
                };

                // Branch to the condition block
                self.builder.build_unconditional_branch(condition_block).unwrap();

                // Set up the condition block
                self.builder.position_at_end(condition_block);

                // Compile the condition
                let condition_value = self.compile_expression(condition.clone())?;

                // Convert to boolean (0 or 1)
                let condition_bool = match condition_value {
                    BasicValueEnum::IntValue(int_val) => {
                        // Compare with 0 to get a boolean value (0 = false, anything else = true)
                        let zero = self.i64_type.const_int(0, false);
                        self.builder.build_int_compare(
                            inkwell::IntPredicate::NE,
                            int_val,
                            zero,
                            "for_cond"
                        ).unwrap()
                    },
                    _ => return Err(LutError::compiler_error("Expected integer condition in for loop", None))
                };

                // Conditional branch: if condition is true, go to body, otherwise exit
                self.builder.build_conditional_branch(condition_bool, body_block, exit_block).unwrap();

                // Set up the loop body block
                self.builder.position_at_end(body_block);

                // Save the old loop exit and continue blocks (for nested loops)
                let old_loop_exit = self.current_loop_exit;
                let old_loop_continue = self.current_loop_continue;

                // Update the current loop exit and continue blocks
                self.current_loop_exit = Some(exit_block);
                self.current_loop_continue = Some(update_block); // Continue goes to update

                // Compile the loop body
                for stmt in body {
                    self.compile_statement(stmt)?;
                }

                // Restore the old loop exit and continue blocks
                self.current_loop_exit = old_loop_exit;
                self.current_loop_continue = old_loop_continue;

                // After the body, branch to the update block
                self.builder.build_unconditional_branch(update_block).unwrap();

                // Set up the update block
                self.builder.position_at_end(update_block);

                // Special handling for update expression to properly update variables
                match &update {
                    Expr::Binary { left, operator, right } => {
                        // Handle variable assignment (i : value)
                        if operator.token_type == TokenType::Colon {
                            if let Expr::VariableRef(name) = &**left {
                                // This is a variable assignment - evaluate right side and update variable
                                let value = self.compile_expression(*right.clone())?;

                                // Get the variable's allocation
                                if let Some(ptr) = self.variables.get(name) {
                                    // Store the new value
                                    self.builder.build_store(*ptr, value).unwrap();
                                } else {
                                    return Err(LutError::compiler_error(format!("Undefined variable in for loop update: {}", name), None));
                                }
                            } else {
                                // Just evaluate it normally
                                self.compile_expression(update.clone())?;
                            }
                        } else {
                            // Not an assignment, might be an expression that calculates a new value
                            // Check if this is a recognized update pattern like "$i + 1"
                            if let Expr::VariableRef(var_name) = &**left {
                                if var_name.starts_with('$') {
                                    // Extract the actual variable name (without $)
                                    let actual_name = var_name[1..].to_string();

                                    // Compile the expression to get the new value
                                    let result = self.compile_expression(update.clone())?;

                                    // Get the variable's allocation
                                    if let Some(ptr) = self.variables.get(&actual_name) {
                                        // Store the new value
                                        self.builder.build_store(*ptr, result).unwrap();
                                    } else {
                                        return Err(LutError::compiler_error(format!("Undefined variable in for loop update: {}", actual_name), None));
                                    }
                                } else {
                                    // Just evaluate it normally
                                    self.compile_expression(update.clone())?;
                                }
                            } else {
                                // Just evaluate it normally
                                self.compile_expression(update.clone())?;
                            }
                        }
                    }
                    _ => {
                        // Regular expression
                        self.compile_expression(update.clone())?;
                    }
                };

                // Branch back to the condition block
                self.builder.build_unconditional_branch(condition_block).unwrap();

                // Position at the exit block for subsequent code
                self.builder.position_at_end(exit_block);
            },
            Stmt::Break => {
                // Check if we're in a loop
                if let Some(exit_block) = self.current_loop_exit {
                    // Branch to the loop exit block
                    self.builder.build_unconditional_branch(exit_block).unwrap();

                    // Create an unreachable block for subsequent code
                    let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                    let unreachable_block = self.context.append_basic_block(current_function, "after_break");
                    self.builder.position_at_end(unreachable_block);
                } else {
                    return Err(LutError::compiler_error("Break statement outside of loop", None));
                }
            },
            Stmt::Continue => {
                // Check if we're in a loop
                if let Some(continue_block) = self.current_loop_continue {
                    // Branch to the loop continue block
                    self.builder.build_unconditional_branch(continue_block).unwrap();

                    // Create an unreachable block for subsequent code
                    let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                    let unreachable_block = self.context.append_basic_block(current_function, "after_continue");
                    self.builder.position_at_end(unreachable_block);
                } else {
                    return Err(LutError::compiler_error("Continue statement outside of loop", None));
                }
            }
        }
        
        Ok(())
    }
    
    fn compile_expression(&mut self, expr: Expr) -> Result<BasicValueEnum<'ctx>, LutError> {
        match expr {
            Expr::FunctionCall { name, arguments } => {
                // Clone the function reference to avoid borrowing issues
                let function_clone = if let Some(function) = self.functions.get(&name) {
                    *function
                } else {
                    return Err(LutError::compiler_error(
                        format!("Undefined function: {}", name),
                        None
                    ));
                };
                
                // Compile the arguments
                let mut compiled_args = Vec::with_capacity(arguments.len());
                for arg in arguments {
                    let compiled_arg = self.compile_expression(arg)?;
                    compiled_args.push(compiled_arg.into());
                }
                
                // Check that we have the right number of arguments
                if compiled_args.len() != function_clone.count_params() as usize {
                    return Err(LutError::compiler_error(
                        format!(
                            "Function {} takes {} arguments, but {} were provided",
                            name,
                            function_clone.count_params(),
                            compiled_args.len()
                        ),
                        None
                    ));
                }
                
                // Create a unique call ID
                let call_id = format!("call_{}", self.module.get_globals().count());
                
                // Build the function call
                let result = self.handle_llvm_err(
                    self.builder.build_call(function_clone, &compiled_args, &call_id),
                    &format!("building call to function {}", name)
                )?;
                
                // Get the return value
                match result.try_as_basic_value() {
                    // Function returned a value
                    Either::Left(value) => Ok(value),
                    // Function returned void (should not happen for our functions)
                    Either::Right(_) => {
                        Ok(self.i64_type.const_int(0, false).into())
                    }
                }
            },
            Expr::FloatLiteral(value) => {
                // Currently we don't have proper float support in the LLVM compiler
                // So we convert it to an integer value for now
                let int_value = self.i64_type.const_int(value as u64, true);
                Ok(int_value.into())
            },
            Expr::Ternary { condition, then_branch, else_branch } => {
                // Compile the condition
                let condition_val = self.compile_expression(*condition)?;

                // Convert condition to boolean (0 or 1)
                let condition_int = match condition_val {
                    BasicValueEnum::IntValue(int_val) => int_val,
                    _ => return Err(LutError::compiler_error("Expected integer value for ternary condition", None))
                };

                // Compare condition with 0 to get a boolean value
                let zero = self.i64_type.const_int(0, false);
                let condition_bool = self.builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    condition_int,
                    zero,
                    "ternary_cond"
                ).unwrap();

                // Create the necessary basic blocks
                let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                let then_block = self.context.append_basic_block(current_function, "ternary_then");
                let else_block = self.context.append_basic_block(current_function, "ternary_else");
                let merge_block = self.context.append_basic_block(current_function, "ternary_merge");

                // Branch based on the condition
                self.builder.build_conditional_branch(condition_bool, then_block, else_block).unwrap();

                // Build the then block
                self.builder.position_at_end(then_block);
                let then_value = self.compile_expression(*then_branch)?;
                let then_block_end = self.builder.get_insert_block().unwrap();
                self.builder.build_unconditional_branch(merge_block).unwrap();

                // Build the else block
                self.builder.position_at_end(else_block);
                let else_value = self.compile_expression(*else_branch)?;
                let else_block_end = self.builder.get_insert_block().unwrap();
                self.builder.build_unconditional_branch(merge_block).unwrap();

                // Merge block with phi node
                self.builder.position_at_end(merge_block);

                // Create a phi node for the result
                // The type depends on which branch was taken
                match (then_value, else_value) {
                    (BasicValueEnum::IntValue(then_int), BasicValueEnum::IntValue(else_int)) => {
                        // Both branches return integers
                        let phi = self.builder.build_phi(self.i64_type, "ternary_result").unwrap();
                        phi.add_incoming(&[
                            (&then_int, then_block_end),
                            (&else_int, else_block_end)
                        ]);
                        Ok(phi.as_basic_value())
                    },
                    (BasicValueEnum::PointerValue(then_ptr), BasicValueEnum::PointerValue(else_ptr)) => {
                        // Both branches return pointers (strings)
                        let ptr_type = self.context.ptr_type(AddressSpace::default());
                        let phi = self.builder.build_phi(ptr_type, "ternary_result").unwrap();
                        phi.add_incoming(&[
                            (&then_ptr, then_block_end),
                            (&else_ptr, else_block_end)
                        ]);
                        Ok(phi.as_basic_value())
                    },
                    _ => Err(LutError::compiler_error("Ternary branches must return the same type", None))
                }
            },
            Expr::VariableRef(name) => {
                if name.starts_with('$') {
                    let var_name = name[1..].to_string();
                    if let Some(ptr) = self.variables.get(&var_name) {
                        // Get the variable pointer
                        let ptr_val = *ptr;

                        // Use the variable_types map to determine how to load the value
                        match self.variable_types.get(&var_name) {
                            Some(VariableType::Integer) | Some(VariableType::Boolean) | Some(VariableType::Float) => {
                                // Load as integer value (float is treated as integer for now)
                                let int_load = self.builder.build_load(self.i64_type, ptr_val, &format!("{}_int", var_name)).unwrap();
                                Ok(int_load)
                            },
                            Some(VariableType::String) => {
                                // Load as pointer value
                                let ptr_type = self.context.ptr_type(AddressSpace::default());
                                let ptr_load = self.builder.build_load(ptr_type, ptr_val, &format!("{}_ptr", var_name)).unwrap();
                                Ok(ptr_load)
                            },
                            Some(VariableType::Array) | Some(VariableType::Array2D) => {
                                // For now, treat arrays as pointers
                                let ptr_type = self.context.ptr_type(AddressSpace::default());
                                let ptr_load = self.builder.build_load(ptr_type, ptr_val, &format!("{}_ptr", var_name)).unwrap();
                                Ok(ptr_load)
                            },
                            None => {
                                // Fallback for backward compatibility - try to guess based on the pointer type
                                // Try loading as pointer
                                let ptr_type = self.context.ptr_type(AddressSpace::default());
                                let ptr_load = self.builder.build_load(ptr_type, ptr_val, &format!("{}_ptr", var_name)).unwrap();
                                Ok(ptr_load)
                            }
                        }
                    } else {
                        Err(LutError::compiler_error(
                            format!("Undefined variable: {}", var_name),
                            None
                        ))
                    }
                } else {
                    Err(LutError::compiler_error(
                        format!("Invalid variable reference: {}", name),
                        None
                    ))
                }
            },
            Expr::NumberLiteral(value) => {
                let int_value = self.i64_type.const_int(value as u64, true);
                Ok(int_value.into())
            },
            Expr::TextLiteral(value) => {
                // Create a heap-allocated string for the text literal
                let string_ptr = self.create_heap_string(&value);
                Ok(string_ptr.into())
            },
            Expr::BooleanLiteral(value) => {
                // Boolean literals are represented as i64 values (0 for false, 1 for true)
                let bool_value = self.i64_type.const_int(if value { 1 } else { 0 }, false);
                Ok(bool_value.into())
                // Note: When storing this in a variable, it will be tagged as VariableType::Boolean
            },
            Expr::Grouping { expression } => {
                // Grouping just evaluates the inner expression
                self.compile_expression(*expression)
            },
            Expr::Unary { operator, right } => {
                let right_val = self.compile_expression(*right)?;
                
                if let BasicValueEnum::IntValue(int_val) = right_val {
                    match operator.token_type {
                        TokenType::Minus => {
                            let zero = self.i64_type.const_int(0, true);
                            let result = self.builder.build_int_sub(zero, int_val, "neg").unwrap();
                            Ok(result.into())
                        },
                        TokenType::Not => {
                            // Convert to boolean (0 or 1) and negate
                            let zero = self.i64_type.const_int(0, false);
                            let is_zero = self.builder.build_int_compare(
                                inkwell::IntPredicate::EQ, 
                                int_val, 
                                zero, 
                                "is_zero"
                            ).unwrap();
                            let result = self.builder.build_int_z_extend(
                                is_zero, 
                                self.i64_type, 
                                "bool_not"
                            ).unwrap();
                            Ok(result.into())
                        },
                        _ => Err(LutError::compiler_error(
                            format!("Unsupported unary operator: {:?}", operator.token_type),
                            Some(operator.line)
                        ))
                    }
                } else {
                    Err(LutError::compiler_error(
                        "Expected integer value for unary operation",
                        Some(operator.line)
                    ))
                }
            },
            Expr::Binary { left, operator, right } => {
                // Special handling for assignment with colon operator
                if operator.token_type == TokenType::Colon {
                    if let Expr::VariableRef(name) = &*left {
                        // This is an assignment: var_name : value
                        // Evaluate the right side first
                        let value = self.compile_expression(*right.clone())?;

                        // Check if the variable already exists
                        if let Some(ptr) = self.variables.get(name) {
                            // Variable exists, update its value
                            self.builder.build_store(*ptr, value).unwrap();
                            return Ok(value); // Return the assigned value
                        } else {
                            // This is handled in the declaration part, should not happen here
                            return Err(LutError::compiler_error(format!("Variable '{}' not found for assignment", name), None));
                        }
                    }
                }

                // Regular binary expression (non-assignment)
                let left_val = self.compile_expression(*left)?;
                let right_val = self.compile_expression(*right)?;

                // Helper function to convert a value to integer if possible
                let to_int_value = |val: BasicValueEnum<'ctx>| -> Result<inkwell::values::IntValue<'ctx>, String> {
                    match val {
                        BasicValueEnum::IntValue(int_val) => Ok(int_val),
                        BasicValueEnum::PointerValue(ptr_val) => {
                            // Try to convert string to number using atoll
                            let result = self.builder.build_call(
                                self.atoll_func,
                                &[ptr_val.into()],
                                "atoll_call"
                            ).unwrap();

                            Ok(result.try_as_basic_value().left().unwrap().into_int_value())
                        },
                        _ => Err("Cannot convert value to integer".to_string())
                    }
                };

                // Try to convert both operands to integers
                let left_int = to_int_value(left_val)?;
                let right_int = to_int_value(right_val)?;

                match operator.token_type {
                    TokenType::Plus => {
                        let result = self.builder.build_int_add(left_int, right_int, "add").unwrap();
                        Ok(result.into())
                    },
                    TokenType::Minus => {
                        let result = self.builder.build_int_sub(left_int, right_int, "sub").unwrap();
                        Ok(result.into())
                    },
                    TokenType::Star => {
                        let result = self.builder.build_int_mul(left_int, right_int, "mul").unwrap();
                        Ok(result.into())
                    },
                    TokenType::Slash => {
                        // Add division by zero check
                        let zero = self.i64_type.const_int(0, false);
                        let is_zero = self.builder.build_int_compare(
                            inkwell::IntPredicate::EQ,
                            right_int,
                            zero,
                            "is_zero"
                        ).unwrap();

                        // Create basic blocks for division and division by zero error
                        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                        let div_block = self.context.append_basic_block(current_function, "div");
                        let div_by_zero_block = self.context.append_basic_block(current_function, "div_by_zero");
                        let cont_block = self.context.append_basic_block(current_function, "div_cont");

                        // Branch based on zero check
                        self.builder.build_conditional_branch(is_zero, div_by_zero_block, div_block).unwrap();

                        // Division block
                        self.builder.position_at_end(div_block);
                        let div_result = self.builder.build_int_signed_div(left_int, right_int, "div").unwrap();
                        self.builder.build_unconditional_branch(cont_block).unwrap();
                        let div_block_end = self.builder.get_insert_block().unwrap();

                        // Division by zero error block
                        self.builder.position_at_end(div_by_zero_block);

                        // Print error message
                        let error_msg = "Runtime error: Division by zero\n";
                        let error_ptr = self.create_string_literal(error_msg);
                        self.builder.build_call(
                            self.printf_func,
                            &[error_ptr.into()],
                            "div_zero_error_msg"
                        ).unwrap();

                        // Exit with error code - using consistent i64 type
                        let exit_code = self.i64_type.const_int(1, false);
                        self.builder.build_return(Some(&exit_code)).unwrap();

                        // Continue block for normal execution
                        self.builder.position_at_end(cont_block);

                        // Phi node to select the appropriate result
                        let phi = self.builder.build_phi(self.i64_type, "div_result").unwrap();
                        phi.add_incoming(&[(&div_result, div_block_end)]);

                        Ok(phi.as_basic_value())
                    },
                    TokenType::Percent => {
                        // Add modulo by zero check
                        let zero = self.i64_type.const_int(0, false);
                        let is_zero = self.builder.build_int_compare(
                            inkwell::IntPredicate::EQ,
                            right_int,
                            zero,
                            "is_zero"
                        ).unwrap();

                        // Create basic blocks for modulo and modulo by zero error
                        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                        let mod_block = self.context.append_basic_block(current_function, "mod");
                        let mod_by_zero_block = self.context.append_basic_block(current_function, "mod_by_zero");
                        let cont_block = self.context.append_basic_block(current_function, "mod_cont");

                        // Branch based on zero check
                        self.builder.build_conditional_branch(is_zero, mod_by_zero_block, mod_block).unwrap();

                        // Modulo block
                        self.builder.position_at_end(mod_block);
                        let mod_result = self.builder.build_int_signed_rem(left_int, right_int, "mod").unwrap();
                        self.builder.build_unconditional_branch(cont_block).unwrap();
                        let mod_block_end = self.builder.get_insert_block().unwrap();

                        // Modulo by zero error block
                        self.builder.position_at_end(mod_by_zero_block);

                        // Print error message
                        let error_msg = "Runtime error: Modulo by zero\n";
                        let error_ptr = self.create_string_literal(error_msg);
                        self.builder.build_call(
                            self.printf_func,
                            &[error_ptr.into()],
                            "mod_zero_error_msg"
                        ).unwrap();

                        // Exit with error code - using consistent i64 type
                        let exit_code = self.i64_type.const_int(1, false);
                        self.builder.build_return(Some(&exit_code)).unwrap();

                        // Continue block for normal execution
                        self.builder.position_at_end(cont_block);

                        // Phi node to select the appropriate result
                        let phi = self.builder.build_phi(self.i64_type, "mod_result").unwrap();
                        phi.add_incoming(&[(&mod_result, mod_block_end)]);

                        Ok(phi.as_basic_value())
                    },
                    // Comparison operators
                    TokenType::Equal => {
                        let result = self.builder.build_int_compare(
                            inkwell::IntPredicate::EQ, 
                            left_int, 
                            right_int, 
                            "eq"
                        ).unwrap();
                        // Convert i1 to i64
                        let result_ext = self.builder.build_int_z_extend(result, self.i64_type, "zext").unwrap();
                        Ok(result_ext.into())
                    },
                    TokenType::NotEqual => {
                        let result = self.builder.build_int_compare(
                            inkwell::IntPredicate::NE, 
                            left_int, 
                            right_int, 
                            "ne"
                        ).unwrap();
                        let result_ext = self.builder.build_int_z_extend(result, self.i64_type, "zext").unwrap();
                        Ok(result_ext.into())
                    },
                    TokenType::Less => {
                        let result = self.builder.build_int_compare(
                            inkwell::IntPredicate::SLT, 
                            left_int, 
                            right_int, 
                            "lt"
                        ).unwrap();
                        let result_ext = self.builder.build_int_z_extend(result, self.i64_type, "zext").unwrap();
                        Ok(result_ext.into())
                    },
                    TokenType::LessEqual => {
                        let result = self.builder.build_int_compare(
                            inkwell::IntPredicate::SLE, 
                            left_int, 
                            right_int, 
                            "le"
                        ).unwrap();
                        let result_ext = self.builder.build_int_z_extend(result, self.i64_type, "zext").unwrap();
                        Ok(result_ext.into())
                    },
                    TokenType::Greater => {
                        let result = self.builder.build_int_compare(
                            inkwell::IntPredicate::SGT, 
                            left_int, 
                            right_int, 
                            "gt"
                        ).unwrap();
                        let result_ext = self.builder.build_int_z_extend(result, self.i64_type, "zext").unwrap();
                        Ok(result_ext.into())
                    },
                    TokenType::GreaterEqual => {
                        let result = self.builder.build_int_compare(
                            inkwell::IntPredicate::SGE,
                            left_int,
                            right_int,
                            "ge"
                        ).unwrap();
                        let result_ext = self.builder.build_int_z_extend(result, self.i64_type, "zext").unwrap();
                        Ok(result_ext.into())
                    },
                    TokenType::And => {
                        // Convert both operands to boolean values first (0 = false, non-zero = true)
                        let zero = self.i64_type.const_int(0, false);

                        // Compare left with 0
                        let left_bool = self.builder.build_int_compare(
                            inkwell::IntPredicate::NE,
                            left_int,
                            zero,
                            "left_bool"
                        ).unwrap();

                        // Compare right with 0
                        let right_bool = self.builder.build_int_compare(
                            inkwell::IntPredicate::NE,
                            right_int,
                            zero,
                            "right_bool"
                        ).unwrap();

                        // Logical AND of the two boolean values
                        let result = self.builder.build_and(left_bool, right_bool, "and").unwrap();

                        // Convert i1 to i64
                        let result_ext = self.builder.build_int_z_extend(result, self.i64_type, "zext").unwrap();
                        Ok(result_ext.into())
                    },
                    TokenType::Or => {
                        // Convert both operands to boolean values first (0 = false, non-zero = true)
                        let zero = self.i64_type.const_int(0, false);

                        // Compare left with 0
                        let left_bool = self.builder.build_int_compare(
                            inkwell::IntPredicate::NE,
                            left_int,
                            zero,
                            "left_bool"
                        ).unwrap();

                        // Compare right with 0
                        let right_bool = self.builder.build_int_compare(
                            inkwell::IntPredicate::NE,
                            right_int,
                            zero,
                            "right_bool"
                        ).unwrap();

                        // Logical OR of the two boolean values
                        let result = self.builder.build_or(left_bool, right_bool, "or").unwrap();

                        // Convert i1 to i64
                        let result_ext = self.builder.build_int_z_extend(result, self.i64_type, "zext").unwrap();
                        Ok(result_ext.into())
                    },
                    _ => Err(LutError::compiler_error(format!("Binary operator not yet implemented: {:?}", operator.token_type), Some(operator.line)))
                }
            },
            Expr::ArrayLiteral(elements) => {
                // Create a string representation of the array for the compiled output
                let mut array_str = String::from("[");

                // Add array elements as a comma-separated list with placeholders
                for (i, _) in elements.iter().enumerate() {
                    if i > 0 {
                        array_str.push_str(", ");
                    }

                    // Use a placeholder for each element
                    array_str.push_str("?");
                }

                array_str.push_str("]");

                // Create a string literal for the array representation
                let string_ptr = self.create_heap_string(&array_str);
                Ok(string_ptr.into())
            },
            Expr::ArrayLiteral2D(rows) => {
                // Create a string representation of the 2D array for the compiled output
                let mut array_str = String::from("[");

                // Add array rows
                for (i, row) in rows.iter().enumerate() {
                    if i > 0 {
                        array_str.push_str("; ");
                    }

                    array_str.push_str("[");

                    // Add row elements
                    for (j, _) in row.iter().enumerate() {
                        if j > 0 {
                            array_str.push_str(", ");
                        }

                        // Use a placeholder for each element
                        array_str.push_str("?");
                    }

                    array_str.push_str("]");
                }

                array_str.push_str("]");

                // Create a string literal for the array representation
                let string_ptr = self.create_heap_string(&array_str);
                Ok(string_ptr.into())
            },
            Expr::Command { name, args } => {
                match name.as_str() {
                    "fp" | "-fp" => {
                        if args.len() != 1 {
                            return Err(LutError::compiler_error("Floating point command expects one argument", None));
                        }

                        // Compile the argument expression
                        let value = self.compile_expression(args[0].clone())?;

                        // For now, we just use integers to represent floating point values
                        match value {
                            BasicValueEnum::IntValue(int_val) => {
                                // Already an integer
                                Ok(int_val.into())
                            },
                            BasicValueEnum::PointerValue(ptr_val) => {
                                // Try to convert string to integer using atoll
                                let result = self.builder.build_call(
                                    self.atoll_func,
                                    &[ptr_val.into()],
                                    "atoll_call"
                                ).unwrap();

                                Ok(result.try_as_basic_value().left().unwrap())
                            },
                            _ => Err(LutError::compiler_error("Cannot convert value to floating point", None))
                        }
                    },
                    "array" | "-array" => {
                        // Note: For the LLVM compiler, we're implementing basic array support
                        // that will at least display the array description in compiled output

                        if args.len() == 1 {
                            match &args[0] {
                                Expr::ArrayLiteral(elements) => {
                                    // Create a string representation of the array
                                    let mut array_str = String::from("[");

                                    // Add array elements as a comma-separated list
                                    for (i, _element) in elements.iter().enumerate() {
                                        if i > 0 {
                                            array_str.push_str(", ");
                                        }

                                        // Add placeholder for element - we could evaluate here
                                        // for a more complete implementation
                                        array_str.push_str("?");
                                    }

                                    array_str.push_str("]");

                                    // Create a string literal for the array representation
                                    let string_ptr = self.create_heap_string(&array_str);
                                    return Ok(string_ptr.into());
                                },
                                Expr::ArrayLiteral2D(rows) => {
                                    // Create a string representation of the 2D array
                                    let mut array_str = String::from("[");

                                    // Add array rows
                                    for (i, row) in rows.iter().enumerate() {
                                        if i > 0 {
                                            array_str.push_str("; ");
                                        }

                                        array_str.push_str("[");

                                        // Add row elements
                                        for (j, _) in row.iter().enumerate() {
                                            if j > 0 {
                                                array_str.push_str(", ");
                                            }

                                            // Add placeholder for element
                                            array_str.push_str("?");
                                        }

                                        array_str.push_str("]");
                                    }

                                    array_str.push_str("]");

                                    // Create a string literal for the array representation
                                    let string_ptr = self.create_heap_string(&array_str);
                                    return Ok(string_ptr.into());
                                },
                                _ => {
                                    // Try to evaluate the argument
                                    return self.compile_expression(args[0].clone());
                                }
                            }
                        }

                        // If we get here, create a generic array placeholder
                        let array_str = "[array]";
                        let string_ptr = self.create_heap_string(array_str);
                        Ok(string_ptr.into())
                    },
                    "hex" | "-hex" => {
                        if args.len() != 1 {
                            return Err(LutError::compiler_error("Hex command expects one argument", None));
                        }

                        // Compile the argument expression - for now we just treat it as a number
                        let value = self.compile_expression(args[0].clone())?;
                        Ok(value)
                    },
                    "bin" | "-bin" => {
                        if args.len() != 1 {
                            return Err(LutError::compiler_error("Binary command expects one argument", None));
                        }

                        // Compile the argument expression - for now we just treat it as a number
                        let value = self.compile_expression(args[0].clone())?;
                        Ok(value)
                    },
                    "text" | "-text" => {
                        if args.len() != 1 {
                            return Err(LutError::compiler_error("Text command expects one argument", None));
                        }

                        // If the argument is already a string literal, just return that
                        match &args[0] {
                            Expr::TextLiteral(value) => {
                                let string_ptr = self.create_heap_string(value);
                                // String type is indicated by the PointerValue return
                                Ok(string_ptr.into())
                            },
                            // If it's another expression, compile it and convert to string
                            expr => {
                                let value = self.compile_expression(expr.clone())?;
                                match value {
                                    // If it's a string already, return it
                                    BasicValueEnum::PointerValue(ptr_val) => {
                                        // Already a string
                                        Ok(ptr_val.into())
                                    },
                                    // If it's an integer or boolean, convert it to string
                                    BasicValueEnum::IntValue(int_val) => {
                                        // First create a format string for sprintf
                                        let format = "%lld";
                                        let format_ptr = self.create_string_literal(format);

                                        // Allocate buffer for result (20 chars should be enough for int64)
                                        let buffer_size = 20;
                                        let buffer = self.allocate_string(buffer_size);

                                        // Call sprintf
                                        self.builder.build_call(
                                            self.sprintf_func,
                                            &[buffer.into(), format_ptr.into(), int_val.into()],
                                            "sprintf_call"
                                        ).unwrap();

                                        // Result is a string (pointer)
                                        Ok(buffer.into())
                                    },
                                    _ => Err(LutError::compiler_error("Cannot convert value to text", None))
                                }
                            }
                        }
                    },
                    "bool" | "-bool" => {
                        if args.len() != 1 {
                            return Err(LutError::compiler_error("Boolean command expects one argument", None));
                        }

                        // Compile the argument expression
                        let value = self.compile_expression(args[0].clone())?;

                        match value {
                            // If it's already an integer (which includes booleans in our implementation)
                            BasicValueEnum::IntValue(int_val) => {
                                // Compare with 0 to get a boolean value (0 = false, anything else = true)
                                let zero = self.i64_type.const_int(0, false);
                                let result = self.builder.build_int_compare(
                                    inkwell::IntPredicate::NE,
                                    int_val,
                                    zero,
                                    "bool_conv"
                                ).unwrap();

                                // Convert i1 to i64
                                let result_ext = self.builder.build_int_z_extend(
                                    result,
                                    self.i64_type,
                                    "zext_bool"
                                ).unwrap();

                                Ok(result_ext.into())
                            },
                            // If it's a string, check if it's empty
                            BasicValueEnum::PointerValue(ptr_val) => {
                                // Call strlen to get string length
                                let length = self.builder.build_call(
                                    self.strlen_func,
                                    &[ptr_val.into()],
                                    "strlen_call"
                                ).unwrap();

                                let length_val = length.try_as_basic_value().left().unwrap().into_int_value();

                                // Compare length with 0
                                let zero = self.i64_type.const_int(0, false);
                                let is_nonempty = self.builder.build_int_compare(
                                    inkwell::IntPredicate::NE,
                                    length_val,
                                    zero,
                                    "str_nonempty"
                                ).unwrap();

                                // Convert i1 to i64
                                let result = self.builder.build_int_z_extend(
                                    is_nonempty,
                                    self.i64_type,
                                    "zext_bool"
                                ).unwrap();

                                Ok(result.into())
                            },
                            _ => Err(LutError::compiler_error("Cannot convert value to boolean", None))
                        }
                    },
                    "number" | "-number" => {
                        if args.len() != 1 {
                            return Err(LutError::compiler_error("Number command expects one argument", None));
                        }
                        
                        // Compile the argument expression
                        let value = self.compile_expression(args[0].clone())?;
                        
                        match value {
                            // If it's an integer already, return it
                            BasicValueEnum::IntValue(int_val) => {
                                // Already an integer
                                Ok(int_val.into())
                            },
                            // If it's a string, try to convert to integer
                            BasicValueEnum::PointerValue(ptr_val) => {
                                // Call atoll to convert string to integer
                                let result = self.builder.build_call(
                                    self.atoll_func,
                                    &[ptr_val.into()],
                                    "atoll_call"
                                ).unwrap();
                                
                                let int_result = result.try_as_basic_value().left().unwrap();
                                // Result is an integer
                                Ok(int_result)
                            },
                            _ => Err(LutError::compiler_error("Cannot convert value to number", None))
                        }
                    },
                    "asc" | "-asc" => {
                        if args.len() != 1 {
                            return Err(LutError::compiler_error("Asc command expects one argument", None));
                        }
                        
                        // Compile the argument expression
                        let value = self.compile_expression(args[0].clone())?;
                        
                        match value {
                            BasicValueEnum::IntValue(int_val) => {
                                // Allocate 2 bytes (char + null terminator)
                                let buffer = self.allocate_string(2);
                                
                                // Get pointer to buffer as i8*
                                let buffer_i8_ptr = buffer;
                                
                                // Convert int_val to i8 (truncate to ASCII range)
                                let char_val = self.builder.build_int_truncate(
                                    int_val,
                                    self.context.i8_type(),
                                    "ascii_char"
                                ).unwrap();
                                
                                // Store the character at buffer[0]
                                self.builder.build_store(buffer_i8_ptr, char_val).unwrap();
                                
                                // Store null terminator at buffer[1]
                                let null_term = self.context.i8_type().const_int(0, false);
                                let buffer_plus_one = unsafe {
                                    self.builder.build_gep(
                                        self.context.i8_type(),
                                        buffer_i8_ptr,
                                        &[self.context.i32_type().const_int(1, false)],
                                        "buffer_plus_one"
                                    ).unwrap()
                                };
                                self.builder.build_store(buffer_plus_one, null_term).unwrap();
                                
                                Ok(buffer.into())
                            },
                            _ => Err(LutError::compiler_error("Asc command expects an integer argument", None))
                        }
                    },
                    _ => Err(LutError::compiler_error(format!("Command expression not implemented: {}", name), None))
                }
            }
        }
    }
    
    // Helper to create an i64 alloca instruction in the entry block with consistent alignment
    // Create a uniquely named variable to avoid SSA violations
    fn create_entry_block_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let func = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let entry = func.get_first_basic_block().unwrap();
        
        // Create a unique name to avoid SSA violations
        // Format: original_name.uniqueId
        let unique_id = format!("{}.{}", name, func.get_basic_blocks().len());
        
        // Ensure all allocas for integers are in the entry block with consistent alignment
        match entry.get_first_instruction() {
            Some(first_instr) => {
                let builder = self.context.create_builder();
                builder.position_before(&first_instr);
                
                // For i64 values, we use 8-byte alignment to match the type size
                let i64_size = 8; // 8 bytes
                let i64_align = 8; // 8-byte alignment
                
                // Create the alloca with explicit size and alignment
                let alloca_size = self.context.i32_type().const_int(1, false); // 1 element
                let alloca = builder.build_array_alloca(self.i64_type, alloca_size, &unique_id).unwrap();
                
                // Note: We're using default alignment from LLVM
                
                alloca
            }
            None => {
                let current_block = self.builder.get_insert_block().unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(entry);
                
                // For i64 values, we use 8-byte alignment to match the type size
                let i64_size = 8; // 8 bytes
                let i64_align = 8; // 8-byte alignment
                
                // Create the alloca with explicit size and alignment
                let alloca_size = self.context.i32_type().const_int(1, false); // 1 element
                let alloca = builder.build_array_alloca(self.i64_type, alloca_size, &unique_id).unwrap();
                
                // Note: We're using default alignment from LLVM
                
                self.builder.position_at_end(current_block);
                alloca
            }
        }
    }
    
    // Helper to create a pointer alloca instruction in the entry block with consistent alignment
    fn create_pointer_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let func = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let entry = func.get_first_basic_block().unwrap();
        
        // Create a unique name to avoid SSA violations
        // Format: ptr_original_name.uniqueId
        let unique_id = format!("ptr_{}.{}", name, func.get_basic_blocks().len());
        
        // Use array allocation for consistent alignment and handling
        match entry.get_first_instruction() {
            Some(first_instr) => {
                let builder = self.context.create_builder();
                builder.position_before(&first_instr);
                
                // Use array_alloca with explicit size for better alignment control
                let alloca_size = self.context.i32_type().const_int(1, false); // 1 element
                let alloca = builder.build_array_alloca(ptr_type, alloca_size, &unique_id).unwrap();
                
                // Note: We're using default alignment from LLVM
                
                alloca
            }
            None => {
                let current_block = self.builder.get_insert_block().unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(entry);
                
                // Use array_alloca with explicit size for better alignment control
                let alloca_size = self.context.i32_type().const_int(1, false); // 1 element
                let alloca = builder.build_array_alloca(ptr_type, alloca_size, &unique_id).unwrap();
                
                // Note: We're using default alignment from LLVM
                
                self.builder.position_at_end(current_block);
                alloca
            }
        }
    }
    
    // Create a string literal as a global constant
    fn create_string_literal(&mut self, string_val: &str) -> PointerValue<'ctx> {
        // Use the string pool to efficiently manage string literals
        self.string_pool.get_or_create_global_string(
            string_val, 
            self.context, 
            &self.module, 
            &self.builder
        )
    }
    
    // Allocate heap memory for a string
    fn allocate_string(&self, size: u64) -> PointerValue<'ctx> {
        let size_val = self.i64_type.const_int(size, false);
        let malloc_call = self.builder.build_call(
            self.malloc_func,
            &[size_val.into()],
            "malloc_call"
        ).unwrap();
        
        malloc_call.try_as_basic_value().left().unwrap().into_pointer_value()
    }
    
    // Create a heap-allocated string from a string literal
    fn create_heap_string(&mut self, string_val: &str) -> PointerValue<'ctx> {
        // First, check if we already have identical static constant
        // If this is a frequently used string, we can directly return the static version
        if string_val.len() < 32 {  // Only apply this optimization for small strings
            // Intern the string
            let _interned = self.string_pool.intern(string_val.to_string());
            // Check if this string has already been compiled as a global constant
            if let Some(&ptr) = self.string_pool.global_strings.get(string_val) {
                // No need to allocate if we already have it
                return ptr;
            }
        }
        
        // Otherwise, allocate memory for the string (+1 for null terminator)
        let size = string_val.len() as u64 + 1;
        let heap_ptr = self.allocate_string(size);
        
        // Use a more efficient method for small strings
        if string_val.len() <= 32 {
            // For small strings, we can use memcpy intrinsic
            let src_ptr = self.create_string_literal(string_val);
            // Use the modern llvm.memcpy intrinsic with correct signature
            let void_ptr_type = self.context.ptr_type(AddressSpace::default());
            let i1_type = self.context.bool_type();
            
            // Modern memcpy signature: void @llvm.memcpy.p0.p0.i64(ptr dest, ptr src, i64 size, i1 isvolatile)
            let memcpy_type = self.context.void_type().fn_type(
                &[
                    void_ptr_type.into(),     // dest ptr
                    void_ptr_type.into(),     // src ptr
                    self.i64_type.into(),     // size
                    i1_type.into()            // is_volatile
                ],
                false // not variadic
            );
            
            // Get or create the memcpy intrinsic using the modern name
            let memcpy_fn = self.module.get_function("llvm.memcpy.p0.p0.i64")
                .unwrap_or_else(|| {
                    self.module.add_function(
                        "llvm.memcpy.p0.p0.i64", 
                        memcpy_type, 
                        None
                    )
                });
            
            // Call memcpy to copy the string with the correct arguments
            self.builder.build_call(
                memcpy_fn,
                &[
                    heap_ptr.into(),                              // dest pointer
                    src_ptr.into(),                               // source pointer
                    self.i64_type.const_int(size, false).into(),  // size in bytes
                    i1_type.const_int(0, false).into()            // is_volatile=false
                ],
                "memcpy_call"
            ).unwrap();
        } else {
            // For larger strings, use the old character-by-character approach
            // Copy the string data to the allocated memory
            for (i, byte) in string_val.bytes().enumerate() {
                // Get pointer to character position
                let char_ptr = unsafe {
                    self.builder.build_gep(
                        self.context.i8_type(),
                        heap_ptr,
                        &[self.context.i32_type().const_int(i as u64, false)],
                        &format!("char_ptr_{}", i)
                    ).unwrap()
                };
                
                // Store the character
                let char_val = self.context.i8_type().const_int(byte as u64, false);
                self.builder.build_store(char_ptr, char_val).unwrap();
            }
            
            // Add null terminator
            let null_ptr = unsafe {
                self.builder.build_gep(
                    self.context.i8_type(),
                    heap_ptr,
                    &[self.context.i32_type().const_int(string_val.len() as u64, false)],
                    "null_ptr"
                ).unwrap()
            };
            
            let null_char = self.context.i8_type().const_int(0, false);
            self.builder.build_store(null_ptr, null_char).unwrap();
        }
        
        heap_ptr
    }
    
    fn print_value(&mut self, value: BasicValueEnum<'ctx>) -> Result<(), LutError> {
        // Create a unique printf call id to avoid name conflicts
        let call_id = format!("printf_call_{}", self.module.get_globals().count());

        match value {
            BasicValueEnum::IntValue(int_val) => {
                // Create format string for integer: "%lld"
                let format = "%lld";
                let format_ptr = self.create_string_literal(format);

                // Build call with improved error handling
                self.handle_llvm_err(
                    self.builder.build_call(
                        self.printf_func,
                        &[format_ptr.into(), int_val.into()],
                        &call_id
                    ),
                    "integer printf"
                )?;

                Ok(())
            },
            BasicValueEnum::PointerValue(ptr_val) => {
                // Assume pointer is a string and print it with "%s"
                let format = "%s";
                let format_ptr = self.create_string_literal(format);

                // Build call with improved error handling
                self.handle_llvm_err(
                    self.builder.build_call(
                        self.printf_func,
                        &[format_ptr.into(), ptr_val.into()],
                        &call_id
                    ),
                    "string printf"
                )?;

                Ok(())
            },
            _ => Err(LutError::compiler_error("Unsupported value type for printing", None))
        }
    }
    
    // Write the module to a file
    pub fn write_to_file(&self, filename: &str) -> Result<(), LutError> {
        // Write the LLVM IR to the file
        if let Err(e) = self.module.print_to_file(filename) {
            return Err(LutError::compiler_error(
                format!("Error writing LLVM IR to file: {}", e.to_string()),
                None
            ));
        }
        
        // Now that we've fixed the core issue with main function handling,
        // we don't need to post-process the IR anymore
        if filename.contains("benchmark") || filename.contains("factorial") || filename.contains("simple_factorial") {
            println!("Generated LLVM IR for {}", filename);
        }
        
        Ok(())
    }
    
    // JIT compile and execute the module
    pub fn jit_compile_and_run(&self) -> Result<(), LutError> {
        // Create JIT execution engine with better error message
        let execution_engine = self.handle_llvm_err(
            self.module.create_jit_execution_engine(OptimizationLevel::Default),
            "JIT execution engine creation"
        )?;

        unsafe {
            // Get the main function with improved error message
            let main_fn = self.handle_llvm_err(
                execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main"),
                "retrieving main function for JIT execution"
            )?;

            // Store the return value for potential error reporting
            let result = main_fn.call();

            // Check if the program ended with an error code
            if result != 0 {
                return Err(LutError::runtime_error(
                    format!("Program exited with non-zero status code: {}", result),
                    None
                ));
            }
        }

        Ok(())
    }
    
    // Create a native executable file
    pub fn create_executable(&self, output_filename: &str) -> Result<(), LutError> {
        // Get the default target triple
        let target_triple = TargetMachine::get_default_triple();
        println!("Targeting: {}", target_triple.to_string());

        // Get the target from the triple with improved error handling
        let target = self.handle_llvm_err(
            Target::from_triple(&target_triple),
            "obtaining target from triple"
        )?;

        // Create a target machine with improved error handling
        let target_machine = target.create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        ).ok_or_else(|| LutError::compiler_error(
            "Failed to create target machine: no machine for target triple",
            None
        ))?;

        // Set the data layout for the module
        self.module.set_data_layout(&target_machine.get_target_data().get_data_layout());
        self.module.set_triple(&target_triple);

        // Verify the module is valid with detailed error information
        if let Err(err) = self.module.verify() {
            return Err(LutError::compiler_error(
                format!(
                    "Module verification error: {}. This may indicate a type mismatch or malformed IR.",
                    err.to_string()
                ),
                None
            ));
        }

        // First create an object file
        let object_filename = format!("{}.o", output_filename);
        let object_path = std::path::Path::new(&object_filename);

        // Write to file with improved error handling
        self.handle_llvm_err(
            target_machine.write_to_file(&self.module, FileType::Object, object_path),
            "writing object file"
        )?;

        println!("Generated object file: {}", object_filename);

        // Now link the object file into an executable using system linker
        #[cfg(target_os = "macos")]
        let linking_result = Command::new("cc")
            .arg("-o")
            .arg(output_filename)
            .arg(&object_filename)
            .status()
            .map_err(|e| LutError::compiler_error(
                format!("Error executing linker (cc): {}. Make sure you have a C compiler installed.", e),
                None
            ))?;

        #[cfg(target_os = "linux")]
        let linking_result = Command::new("cc")
            .arg("-o")
            .arg(output_filename)
            .arg(&object_filename)
            .status()
            .map_err(|e| LutError::compiler_error(
                format!("Error executing linker (cc): {}. Make sure you have a C compiler installed.", e),
                None
            ))?;

        #[cfg(target_os = "windows")]
        let linking_result = Command::new("cl")
            .arg("/Fe:")
            .arg(output_filename)
            .arg(&object_filename)
            .status()
            .map_err(|e| LutError::compiler_error(
                format!("Error executing linker (cl): {}. Make sure you have Visual Studio or the MSVC toolchain installed.", e),
                None
            ))?;

        // Check if linking succeeded with improved error details
        if !linking_result.success() {
            let error_code = linking_result.code().unwrap_or(-1);
            return Err(LutError::compiler_error(
                format!(
                    "Linking failed with exit code {}. This may be due to missing libraries or incompatible object formats.",
                    error_code
                ),
                None
            ));
        }

        // Make the resulting binary executable on Unix-like systems
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            use std::os::unix::fs::PermissionsExt;

            // Get metadata with improved error message
            let metadata = fs::metadata(output_filename)
                .map_err(|e| LutError::io_error(
                    format!("Failed to get file metadata for '{}': {}", output_filename, e)
                ))?;

            let mut perms = metadata.permissions();
            perms.set_mode(0o755); // rwxr-xr-x

            // Set permissions with improved error message
            fs::set_permissions(output_filename, perms)
                .map_err(|e| LutError::io_error(
                    format!("Failed to set executable permissions on '{}': {}", output_filename, e)
                ))?;
        }

        println!("Generated executable: {}", output_filename);
        Ok(())
    }
}

// Top-level compile function that takes source code and outputs binary
pub fn compile(source: &str, file_path: &str, silent_mode: bool) -> Result<(), LutError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    // Create LLVM context and compiler
    let context = Context::create();
    let source_path = Path::new(file_path);
    let file_stem = source_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    let module_name = file_stem.to_string();
    
    let mut llvm_compiler = LLVMCompiler::new(&context, &module_name, file_path).with_silent_mode(silent_mode);
    llvm_compiler.compile(statements)?;
    
    // Generate output paths
    let ir_path = format!("{}.ll", module_name);
    llvm_compiler.write_to_file(&ir_path)?;
    
    if !silent_mode {
        println!("Generated LLVM IR: {}", ir_path);
        println!("Generating native executable...");
    }
    
    // Generate executable
    llvm_compiler.create_executable(&module_name)?;
    
    if !silent_mode {
        println!("Compilation successful!");
    }
    Ok(())
}

// Backward compatibility wrapper
pub fn compile_default(source: &str, file_path: &str) -> Result<(), String> {
    compile(source, file_path, false).map_err(|e| e.to_string())
}

// JIT compile and run function - used for development/testing
pub fn jit_compile_and_run(source: &str, file_path: &str, silent_mode: bool) -> Result<(), LutError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    // Create LLVM context and compiler
    let context = Context::create();
    let source_path = Path::new(file_path);
    let file_stem = source_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    let module_name = file_stem.to_string();
    
    let mut llvm_compiler = LLVMCompiler::new(&context, &module_name, file_path).with_silent_mode(silent_mode);
    llvm_compiler.compile(statements)?;
    
    // Generate IR for debugging
    let ir_path = format!("{}.ll", module_name);
    llvm_compiler.write_to_file(&ir_path)?;
    
    if !silent_mode {
        println!("Generated LLVM IR: {}", ir_path);
        println!("Running the program with JIT...");
    }
    
    // JIT compile and run
    llvm_compiler.jit_compile_and_run()
}

// Backward compatibility wrapper
pub fn jit_compile_and_run_default(source: &str, file_path: &str) -> Result<(), String> {
    jit_compile_and_run(source, file_path, false).map_err(|e| e.to_string())
}