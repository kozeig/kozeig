use crate::lexer::{Lexer, TokenType};
use crate::parser::{Expr, Parser, Stmt};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::types::IntType;
use inkwell::OptimizationLevel;
use inkwell::AddressSpace;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine
};

// Define an enum for variable types
#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Integer,
    String,
    Boolean,
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
}

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
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
        
        LLVMCompiler {
            context,
            module,
            builder,
            variables: HashMap::new(),
            variable_types: HashMap::new(),
            printf_func,
            i64_type,
            sprintf_func,
            atoll_func,
            malloc_func,
            strlen_func,
        }
    }
    
    // Create an entry point
    pub fn create_main_function(&self) -> FunctionValue<'ctx> {
        let i32_type = self.context.i32_type();
        let main_func_type = i32_type.fn_type(&[], false);
        let main_func = self.module.add_function("main", main_func_type, None);
        let entry_block = self.context.append_basic_block(main_func, "entry");
        self.builder.position_at_end(entry_block);
        
        main_func
    }
    
    // Create a print function that will use printf
    fn create_print_string(&self, text: &str) {
        // Get a string literal pointer
        let str_ptr = self.create_string_literal(text);
        
        // Create a unique call ID
        let call_id = format!("printf_call_{}", self.module.get_globals().count());
        
        // Call printf with the format string
        self.builder.build_call(self.printf_func, &[str_ptr.into()], &call_id).unwrap();
    }
    
    // Compile all statements and create a binary
    pub fn compile(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        let _main_func = self.create_main_function();
        
        // Compile all statements
        for stmt in statements {
            self.compile_statement(stmt)?;
        }
        
        // Return 0 from main
        let i32_type = self.context.i32_type();
        let return_value = i32_type.const_int(0, false);
        self.builder.build_return(Some(&return_value)).unwrap();
        
        // Verify the module
        if let Err(err) = self.module.verify() {
            return Err(format!("Module verification error: {}", err.to_string()));
        }
        
        Ok(())
    }
    
    fn compile_statement(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Declaration { name, initializer } => {
                // Create a variable (alloca) in the entry block
                let value = self.compile_expression(initializer)?;
                
                // Create the appropriate type of alloca based on the value type
                let (ptr, var_type) = match value {
                    BasicValueEnum::IntValue(_) => {
                        // For numbers, use i64 type
                        let ptr = self.create_entry_block_alloca(&name);
                        (ptr, VariableType::Integer)
                    },
                    BasicValueEnum::PointerValue(_) => {
                        // For strings, use pointer type
                        let ptr = self.create_pointer_alloca(&name);
                        (ptr, VariableType::String)
                    },
                    _ => return Err("Unsupported variable type".to_string())
                };
                
                self.builder.build_store(ptr, value).unwrap();
                self.variables.insert(name.clone(), ptr);
                self.variable_types.insert(name, var_type);
            },
            Stmt::Expression(expr) => {
                // Just evaluate the expression for its side effects
                self.compile_expression(expr)?;
            },
            Stmt::Command { name, args } => {
                match name.as_str() {
                    "-print" => {
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
                    _ => return Err(format!("Unknown command in LLVM compiler: {}", name)),
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
                    _ => return Err("Expected integer condition in if statement".to_string())
                };
                
                // Create the conditional branch instruction based on the condition
                self.builder.build_conditional_branch(condition_value, then_block, else_block).unwrap();
                
                // Build the then block
                self.builder.position_at_end(then_block);
                
                // Compile all statements in the then branch
                for stmt in then_branch {
                    self.compile_statement(stmt)?;
                }
                
                // Branch to the merge block
                self.builder.build_unconditional_branch(merge_block).unwrap();
                
                // Remember the current block to handle nested ifs properly
                let _then_end_block = self.builder.get_insert_block().unwrap();
                
                // Build the else block
                self.builder.position_at_end(else_block);
                
                // Compile all statements in the else branch if it exists
                if let Some(else_statements) = else_branch {
                    for stmt in else_statements {
                        self.compile_statement(stmt)?;
                    }
                }
                
                // Branch to the merge block
                self.builder.build_unconditional_branch(merge_block).unwrap();
                
                // Remember the current block
                let _else_end_block = self.builder.get_insert_block().unwrap();
                
                // Set the insertion point to the merge block for subsequent code
                self.builder.position_at_end(merge_block);
            }
        }
        
        Ok(())
    }
    
    fn compile_expression(&mut self, expr: Expr) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            Expr::VariableRef(name) => {
                if name.starts_with('$') {
                    let var_name = name[1..].to_string();
                    if let Some(ptr) = self.variables.get(&var_name) {
                        // Get the variable pointer
                        let ptr_val = *ptr;
                        
                        // Use the variable_types map to determine how to load the value
                        match self.variable_types.get(&var_name) {
                            Some(VariableType::Integer) | Some(VariableType::Boolean) => {
                                // Load as integer value
                                let int_load = self.builder.build_load(self.i64_type, ptr_val, &format!("{}_int", var_name)).unwrap();
                                Ok(int_load)
                            },
                            Some(VariableType::String) => {
                                // Load as pointer value
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
                        Err(format!("Undefined variable: {}", var_name))
                    }
                } else {
                    Err(format!("Invalid variable reference: {}", name))
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
                        _ => Err(format!("Unsupported unary operator: {:?}", operator.token_type))
                    }
                } else {
                    Err("Expected integer value for unary operation".to_string())
                }
            },
            Expr::Binary { left, operator, right } => {
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
                        let result = self.builder.build_int_signed_div(left_int, right_int, "div").unwrap();
                        Ok(result.into())
                    },
                    TokenType::Percent => {
                        let result = self.builder.build_int_signed_rem(left_int, right_int, "mod").unwrap();
                        Ok(result.into())
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
                    _ => Err(format!("Binary operator not yet implemented: {:?}", operator.token_type))
                }
            },
            Expr::Command { name, args } => {
                match name.as_str() {
                    "-text" => {
                        if args.len() != 1 {
                            return Err("Text command expects one argument".to_string());
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
                                    _ => Err("Cannot convert value to text".to_string())
                                }
                            }
                        }
                    },
                    "-number" => {
                        if args.len() != 1 {
                            return Err("Number command expects one argument".to_string());
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
                            _ => Err("Cannot convert value to number".to_string())
                        }
                    },
                    "-asc" => {
                        if args.len() != 1 {
                            return Err("Asc command expects one argument".to_string());
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
                            _ => Err("Asc command expects an integer argument".to_string())
                        }
                    },
                    _ => Err(format!("Command expression not implemented: {}", name))
                }
            }
        }
    }
    
    // Helper to create an i64 alloca instruction in the entry block
    fn create_entry_block_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let func = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let entry = func.get_first_basic_block().unwrap();
        
        match entry.get_first_instruction() {
            Some(first_instr) => {
                let builder = self.context.create_builder();
                builder.position_before(&first_instr);
                builder.build_alloca(self.i64_type, name).unwrap()
            }
            None => {
                let current_block = self.builder.get_insert_block().unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(entry);
                let alloca = builder.build_alloca(self.i64_type, name).unwrap();
                self.builder.position_at_end(current_block);
                alloca
            }
        }
    }
    
    // Helper to create a pointer alloca instruction in the entry block
    fn create_pointer_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let func = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let entry = func.get_first_basic_block().unwrap();
        
        match entry.get_first_instruction() {
            Some(first_instr) => {
                let builder = self.context.create_builder();
                builder.position_before(&first_instr);
                builder.build_alloca(ptr_type, name).unwrap()
            }
            None => {
                let current_block = self.builder.get_insert_block().unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(entry);
                let alloca = builder.build_alloca(ptr_type, name).unwrap();
                self.builder.position_at_end(current_block);
                alloca
            }
        }
    }
    
    // Create a string literal as a global constant
    fn create_string_literal(&self, string_val: &str) -> PointerValue<'ctx> {
        let i8_type = self.context.i8_type();
        let string_type = i8_type.array_type((string_val.len() + 1) as u32);
        
        // Create a unique name for the global string
        let global_name = format!("str_{}", self.module.get_globals().count());
        let global_string = self.module.add_global(string_type, None, &global_name);
        global_string.set_constant(true);
        global_string.set_linkage(inkwell::module::Linkage::Private);
        global_string.set_initializer(&self.context.const_string(string_val.as_bytes(), true));
        
        // Create a pointer to the string data
        let zero = self.context.i32_type().const_zero();
        let indices = [zero, zero];
        unsafe {
            self.builder.build_gep(i8_type, global_string.as_pointer_value(), &indices, "str_ptr").unwrap()
        }
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
    fn create_heap_string(&self, string_val: &str) -> PointerValue<'ctx> {
        // Allocate memory for the string (+1 for null terminator)
        let size = string_val.len() as u64 + 1;
        let heap_ptr = self.allocate_string(size);
        
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
        
        heap_ptr
    }
    
    fn print_value(&self, value: BasicValueEnum<'ctx>) -> Result<(), String> {
        // Create a unique printf call id to avoid name conflicts
        let call_id = format!("printf_call_{}", self.module.get_globals().count());
        
        match value {
            BasicValueEnum::IntValue(int_val) => {
                // Create format string for integer: "%lld"
                let format = "%lld";
                let format_ptr = self.create_string_literal(format);
                
                self.builder.build_call(
                    self.printf_func, 
                    &[format_ptr.into(), int_val.into()], 
                    &call_id
                ).unwrap();
                
                Ok(())
            },
            BasicValueEnum::PointerValue(ptr_val) => {
                // Assume pointer is a string and print it with "%s"
                let format = "%s";
                let format_ptr = self.create_string_literal(format);
                
                self.builder.build_call(
                    self.printf_func, 
                    &[format_ptr.into(), ptr_val.into()], 
                    &call_id
                ).unwrap();
                
                Ok(())
            },
            _ => Err("Unsupported value type for printing".to_string())
        }
    }
    
    // Write the module to a file
    pub fn write_to_file(&self, filename: &str) -> Result<(), String> {
        if let Err(e) = self.module.print_to_file(filename) {
            return Err(format!("Error writing LLVM IR to file: {}", e.to_string()));
        }
        Ok(())
    }
    
    // JIT compile and execute the module
    pub fn jit_compile_and_run(&self) -> Result<(), String> {
        let execution_engine = self.module.create_jit_execution_engine(OptimizationLevel::Default)
            .map_err(|e| format!("Error creating JIT execution engine: {}", e))?;
        
        unsafe {
            let main_fn = execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main")
                .map_err(|e| format!("Error getting main function: {}", e))?;
                
            main_fn.call();
        }
        
        Ok(())
    }
    
    // Create a native executable file
    pub fn create_executable(&self, output_filename: &str) -> Result<(), String> {
        // Get the default target triple
        let target_triple = TargetMachine::get_default_triple();
        println!("Targeting: {}", target_triple.to_string());
        
        // Get the target from the triple
        let target = Target::from_triple(&target_triple)
            .map_err(|e| format!("Error getting target from triple: {}", e))?;
        
        // Create a target machine
        let target_machine = target.create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        ).ok_or_else(|| "Error creating target machine".to_string())?;
        
        // Set the data layout for the module
        self.module.set_data_layout(&target_machine.get_target_data().get_data_layout());
        self.module.set_triple(&target_triple);
        
        // Verify the module is valid
        if let Err(err) = self.module.verify() {
            return Err(format!("Module verification error: {}", err.to_string()));
        }
        
        // First create an object file
        let object_filename = format!("{}.o", output_filename);
        let object_path = std::path::Path::new(&object_filename);
        target_machine.write_to_file(&self.module, FileType::Object, object_path)
            .map_err(|e| format!("Error writing object file: {}", e))?;
        
        println!("Generated object file: {}", object_filename);
        
        // Now link the object file into an executable using system linker
        #[cfg(target_os = "macos")]
        let linking_result = Command::new("cc")
            .arg("-o")
            .arg(output_filename)
            .arg(&object_filename)
            .status()
            .map_err(|e| format!("Error executing linker: {}", e))?;
            
        #[cfg(target_os = "linux")]
        let linking_result = Command::new("cc")
            .arg("-o")
            .arg(output_filename)
            .arg(&object_filename)
            .status()
            .map_err(|e| format!("Error executing linker: {}", e))?;
            
        #[cfg(target_os = "windows")]
        let linking_result = Command::new("cl")
            .arg("/Fe:")
            .arg(output_filename)
            .arg(&object_filename)
            .status()
            .map_err(|e| format!("Error executing linker: {}", e))?;
        
        if !linking_result.success() {
            return Err("Linking failed".to_string());
        }
        
        // Make the resulting binary executable on Unix-like systems
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            use std::os::unix::fs::PermissionsExt;
            
            let metadata = fs::metadata(output_filename)
                .map_err(|e| format!("Failed to get metadata: {}", e))?;
                
            let mut perms = metadata.permissions();
            perms.set_mode(0o755); // rwxr-xr-x
            
            fs::set_permissions(output_filename, perms)
                .map_err(|e| format!("Failed to set permissions: {}", e))?;
        }
        
        println!("Generated executable: {}", output_filename);
        Ok(())
    }
}

// Top-level compile function that takes source code and outputs binary
pub fn compile(source: &str, file_path: &str) -> Result<(), String> {
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
    
    let mut llvm_compiler = LLVMCompiler::new(&context, &module_name);
    llvm_compiler.compile(statements)?;
    
    // Generate output paths
    let ir_path = format!("{}.ll", module_name);
    llvm_compiler.write_to_file(&ir_path)?;
    
    println!("Generated LLVM IR: {}", ir_path);
    
    // Generate executable
    println!("Generating native executable...");
    llvm_compiler.create_executable(&module_name)?;
    
    println!("Compilation successful!");
    Ok(())
}

// JIT compile and run function - used for development/testing
pub fn jit_compile_and_run(source: &str, file_path: &str) -> Result<(), String> {
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
    
    let mut llvm_compiler = LLVMCompiler::new(&context, &module_name);
    llvm_compiler.compile(statements)?;
    
    // Generate IR for debugging
    let ir_path = format!("{}.ll", module_name);
    llvm_compiler.write_to_file(&ir_path)?;
    
    println!("Generated LLVM IR: {}", ir_path);
    
    // JIT compile and run
    println!("Running the program with JIT...");
    llvm_compiler.jit_compile_and_run()
}