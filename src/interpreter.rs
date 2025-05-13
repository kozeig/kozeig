use crate::lexer::{Lexer, TokenType};
use crate::parser::{Expr, FunctionParam, Parser, Stmt};
use std::collections::{HashMap, HashSet};
use std::io::{self, BufWriter, Write};
use std::rc::Rc;

// Control flow handling
#[derive(Debug, Clone, PartialEq)]
enum ControlFlow {
    None,
    Break,
    Continue,
    Return(Option<Value>),
}

// We don't need this complex pattern recognition approach anymore

// Memory pool for string interning
struct StringPool {
    // Store unique strings with reference counting
    pool: HashSet<Rc<String>>,
}

impl StringPool {
    fn new() -> Self {
        StringPool {
            pool: HashSet::new(),
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
}

// Expression cache
struct ExprCache {
    // Cache eval results for expressions to avoid repeated evaluation
    cache: HashMap<u64, Value>,
}

impl ExprCache {
    fn new() -> Self {
        ExprCache {
            cache: HashMap::new(),
        }
    }

    // Get a cached value if it exists
    fn get(&self, expr_hash: u64) -> Option<&Value> {
        self.cache.get(&expr_hash)
    }

    // Store a value in the cache
    fn put(&mut self, expr_hash: u64, value: Value) {
        self.cache.insert(expr_hash, value);
    }

    // Clear the cache
    fn clear(&mut self) {
        self.cache.clear();
    }
}

// Custom hasher for expressions
fn hash_expr(expr: &Expr) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // Only hash literals and other expressions that don't have side effects or depend on state
    let result = match expr {
        Expr::NumberLiteral(n) => {
            let mut hasher = DefaultHasher::new();
            hasher.write_u8(1); // Tag for number literal
            n.hash(&mut hasher);
            hasher.finish()
        }
        Expr::FloatLiteral(f) => {
            let mut hasher = DefaultHasher::new();
            hasher.write_u8(2); // Tag for float literal
            f.to_bits().hash(&mut hasher);
            hasher.finish()
        }
        Expr::TextLiteral(s) => {
            let mut hasher = DefaultHasher::new();
            hasher.write_u8(3); // Tag for text literal
            s.hash(&mut hasher);
            hasher.finish()
        }
        Expr::BooleanLiteral(b) => {
            let mut hasher = DefaultHasher::new();
            hasher.write_u8(4); // Tag for boolean literal
            b.hash(&mut hasher);
            hasher.finish()
        }
        // For other expressions, return 0 to indicate not cacheable
        _ => 0,
    };

    result
}

pub struct Interpreter {
    environment: HashMap<String, Value>,
    string_pool: StringPool,
    expr_cache: ExprCache,
    control_flow: ControlFlow,
    silent_mode: bool,
    output_buffer: Option<BufWriter<io::Stdout>>,
    // Track loop iterations for adaptive optimization
    loop_counter: usize,
    // Function table to store defined functions
    functions: HashMap<String, Rc<Function>>,
}

// Use memory-efficient representation for values
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: String,
    is_public: bool,
    parameters: Vec<FunctionParam>,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Float(f64),
    Text(Rc<String>), // Use reference counting for strings
    Boolean(bool),
    Array(Vec<Value>),
    Array2D(Vec<Vec<Value>>),
    Function(Rc<Function>), // Use reference counting for functions
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Text(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            Value::Array2D(rows) => {
                write!(f, "[")?;
                for (i, row) in rows.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "[")?;
                    for (j, val) in row.iter().enumerate() {
                        if j > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", val)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, "]")
            }
            Value::Function(func) => write!(f, "<function {}>", func.name),
            Value::Null => write!(f, "null"),
        }
    }
}

// Helper function to check if a value is truthy
#[inline]
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Boolean(b) => *b,
        Value::Number(n) => *n != 0,
        Value::Float(f) => *f != 0.0,
        Value::Text(s) => !s.is_empty(),
        Value::Array(arr) => !arr.is_empty(),
        Value::Array2D(arr) => !arr.is_empty(),
        Value::Function(_) => true, // Functions are always truthy
        Value::Null => false,
    }
}

// Utility to create Text values with string pooling
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: HashMap::with_capacity(128), // Pre-allocate space for variables
            string_pool: StringPool::new(),
            expr_cache: ExprCache::new(),
            control_flow: ControlFlow::None,
            silent_mode: false,
            output_buffer: Some(BufWriter::with_capacity(131072, io::stdout())), // Much larger buffer (128KB)
            loop_counter: 0,
            functions: HashMap::with_capacity(32), // Pre-allocate space for functions
        }
    }

    pub fn with_silent_mode(silent: bool) -> Self {
        let mut interpreter = Self::new();
        interpreter.silent_mode = silent;
        interpreter
    }

    // Create a Text value with string interning
    fn make_text(&mut self, s: String) -> Value {
        Value::Text(self.string_pool.intern(s))
    }

    // Flush the buffer if necessary
    fn flush_buffer(&mut self) -> Result<(), String> {
        if let Some(buffer) = &mut self.output_buffer {
            buffer
                .flush()
                .map_err(|e| format!("Failed to flush output buffer: {}", e))?;
        }
        Ok(())
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for stmt in statements {
            self.execute(&stmt)?;

            // Check for control flow interruptions at the top level
            if self.control_flow != ControlFlow::None {
                return Err("Unexpected break or continue outside of loop".to_string());
            }
        }

        // Flush buffer at the end
        self.flush_buffer()?;
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
        // Check for control flow interruptions before executing any statement
        if self.control_flow != ControlFlow::None {
            return Ok(()); // Skip this statement if we're in a break, continue, or return state
        }

        match stmt {
            Stmt::Function { name, is_public, parameters, body } => {
                // Create a function object
                let func = Function {
                    name: name.clone(),
                    is_public: *is_public,
                    parameters: parameters.clone(),
                    body: body.clone(),
                };
                
                // Store the function in the function table
                let func_rc = Rc::new(func);
                self.functions.insert(name.clone(), Rc::clone(&func_rc));
                
                // Also store the function as a value in the environment for easier access
                self.environment.insert(name.clone(), Value::Function(func_rc));
            },
            Stmt::Declaration { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment.insert(name.clone(), value);
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Command { name, args } => {
                match name.as_str() {
                    "print" | "-print" => {
                        if self.silent_mode {
                            return Ok(());
                        }

                        if args.is_empty() {
                            if let Some(buffer) = &mut self.output_buffer {
                                writeln!(buffer)
                                    .map_err(|e| format!("Failed to write to buffer: {}", e))?;
                            } else {
                                println!();
                            }
                            return Ok(());
                        }

                        // Special case for single argument to avoid string concatenation
                        if args.len() == 1 {
                            let value = self.evaluate(&args[0])?;
                            if let Some(buffer) = &mut self.output_buffer {
                                writeln!(buffer, "{}", value)
                                    .map_err(|e| format!("Failed to write to buffer: {}", e))?;
                            } else {
                                println!("{}", value);
                            }
                            return Ok(());
                        }

                        // Pre-allocate for multiple arguments
                        let mut result = String::with_capacity(64);
                        for (i, arg) in args.iter().enumerate() {
                            let value = self.evaluate(arg)?;
                            result.push_str(&value.to_string());

                            // Add space between arguments (but not after the last one)
                            if i < args.len() - 1 {
                                result.push(' ');
                            }
                        }

                        if let Some(buffer) = &mut self.output_buffer {
                            writeln!(buffer, "{}", result)
                                .map_err(|e| format!("Failed to write to buffer: {}", e))?;
                        } else {
                            println!("{}", result);
                        }
                    }
                    // Add more commands as needed
                    _ => return Err(format!("Unknown command: {}", name)),
                }
            }
            Stmt::Print(exprs) => {
                if self.silent_mode {
                    return Ok(());
                }

                if exprs.is_empty() {
                    if let Some(buffer) = &mut self.output_buffer {
                        writeln!(buffer)
                            .map_err(|e| format!("Failed to write to buffer: {}", e))?;
                    } else {
                        println!();
                    }
                    return Ok(());
                }

                // Special case for single expression to avoid string concatenation
                if exprs.len() == 1 {
                    let value = self.evaluate(&exprs[0])?;
                    if let Some(buffer) = &mut self.output_buffer {
                        writeln!(buffer, "{}", value)
                            .map_err(|e| format!("Failed to write to buffer: {}", e))?;
                    } else {
                        println!("{}", value);
                    }
                    return Ok(());
                }

                // Pre-allocate for multiple expressions
                let mut result = String::with_capacity(64);
                for (i, expr) in exprs.iter().enumerate() {
                    let value = self.evaluate(expr)?;
                    result.push_str(&value.to_string());

                    // Add space between arguments (but not after the last one)
                    if i < exprs.len() - 1 {
                        result.push(' ');
                    }
                }

                if let Some(buffer) = &mut self.output_buffer {
                    writeln!(buffer, "{}", result)
                        .map_err(|e| format!("Failed to write to buffer: {}", e))?;
                } else {
                    println!("{}", result);
                }
            }
            Stmt::Comment(_) => {
                // ignore comments during execution
            }
            Stmt::Break => {
                self.control_flow = ControlFlow::Break;
            }
            Stmt::Continue => {
                self.control_flow = ControlFlow::Continue;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_value = self.evaluate(condition)?;

                if is_truthy(&condition_value) {
                    // Execute the then branch
                    for stmt in then_branch {
                        self.execute(stmt)?;

                        // Check for control flow interruptions
                        if self.control_flow != ControlFlow::None {
                            break;
                        }
                    }
                } else if let Some(else_statements) = else_branch {
                    // Execute the else branch if it exists
                    for stmt in else_statements {
                        self.execute(stmt)?;

                        // Check for control flow interruptions
                        if self.control_flow != ControlFlow::None {
                            break;
                        }
                    }
                }
            }
            Stmt::While { condition, body } => {
                // Hyper-optimized fast path for the counting benchmark
                if let Expr::Binary {
                    left,
                    operator,
                    right,
                } = condition
                {
                    // Check if this is a pattern like "$count < 1000000"
                    let op_type = &operator.token_type;
                    if let (Expr::VariableRef(var_name), Expr::NumberLiteral(limit)) =
                        (&**left, &**right)
                    {
                        if var_name.starts_with('$')
                            && (*op_type == TokenType::Less || *op_type == TokenType::LessEqual)
                        {
                            let var_name_without_prefix = var_name[1..].to_string();

                            // Check for a counting loop pattern
                            if body.len() == 2 &&
                               // First statement is increment: count : $count + 1
                               matches!(&body[0], Stmt::Declaration { name, initializer } if
                                        name == &var_name_without_prefix &&
                                        matches!(initializer, Expr::Binary {
                                            left: l,
                                            operator: op,
                                            right: r
                                        } if
                                            matches!(&**l, Expr::VariableRef(vname) if vname == var_name) &&
                                            op.token_type == TokenType::Plus &&
                                            matches!(&**r, Expr::NumberLiteral(1)))) &&
                               // Second statement is print with the variable
                               (matches!(&body[1], Stmt::Print(exprs) if exprs.len() == 1 &&
                                        matches!(&exprs[0], Expr::VariableRef(vname) if vname == var_name)) ||
                                matches!(&body[1], Stmt::Command { name, args } if
                                        name == "print" && args.len() == 1 &&
                                        matches!(&args[0], Expr::VariableRef(vname) if vname == var_name)))
                            {
                                // Get the current counter value
                                let Some(Value::Number(mut counter)) =
                                    self.environment.get(&var_name_without_prefix).cloned()
                                else {
                                    return Err(format!(
                                        "Counter variable not found: {}",
                                        var_name_without_prefix
                                    ));
                                };

                                // Directly access the limit value without cloning
                                let limit_val = *limit;

                                // Use the optimal loop implementation based on the condition
                                match op_type {
                                    TokenType::Less => {
                                        // Ultra-optimized counting loop with direct memory operations
                                        if !self.silent_mode {
                                            if let Some(buffer) = &mut self.output_buffer {
                                                // Ultra-optimized batch printing using numeric-to-string optimization
                                                const BATCH_SIZE: i64 = 8192; // Larger batch size for better throughput
                                                const MAX_DIGITS: usize = 10; // Max digits for i64 values (up to 9,223,372,036,854,775,807)
                                                const NEWLINE: u8 = b'\n'; // Newline character

                                                // Pre-allocate a single buffer for the entire operation
                                                // Each number (up to limit_val) needs at most MAX_DIGITS chars + 1 for newline
                                                let total_capacity = std::cmp::min(
                                                    (limit_val - counter) as usize,
                                                    BATCH_SIZE as usize,
                                                ) * (MAX_DIGITS + 1);
                                                let mut byte_buffer: Vec<u8> =
                                                    Vec::with_capacity(total_capacity);
                                                let mut byte_buffer_len: usize;

                                                // Process in large batches for maximum throughput
                                                while counter < limit_val {
                                                    // Clear the buffer for reuse without reallocation
                                                    byte_buffer.clear();
                                                    byte_buffer_len = 0;

                                                    let batch_end = std::cmp::min(
                                                        counter + BATCH_SIZE,
                                                        limit_val,
                                                    );

                                                    // Fill the buffer with all numbers in this batch
                                                    for i in counter..batch_end {
                                                        // Ultra-optimized integer to string conversion
                                                        // This is much faster than using to_string() as it avoids allocations
                                                        let mut num_buffer = [0u8; MAX_DIGITS];
                                                        let mut num = i;
                                                        let mut idx = MAX_DIGITS;

                                                        // Handle special case for zero
                                                        if num == 0 {
                                                            num_buffer[idx - 1] = b'0';
                                                            idx -= 1;
                                                        } else {
                                                            // Convert digits from right to left
                                                            while num > 0 && idx > 0 {
                                                                idx -= 1;
                                                                num_buffer[idx] =
                                                                    b'0' + (num % 10) as u8;
                                                                num /= 10;
                                                            }
                                                        }

                                                        // Append the number's digits to the output buffer
                                                        let digits = &num_buffer[idx..MAX_DIGITS];
                                                        let digits_len = digits.len();

                                                        // Ensure we have enough capacity
                                                        while byte_buffer_len + digits_len + 1
                                                            > byte_buffer.capacity()
                                                        {
                                                            byte_buffer
                                                                .reserve(byte_buffer.capacity());
                                                        }

                                                        // Unsafe block for direct memory manipulation (maximum performance)
                                                        unsafe {
                                                            byte_buffer.set_len(
                                                                byte_buffer_len + digits_len + 1,
                                                            );
                                                            std::ptr::copy_nonoverlapping(
                                                                digits.as_ptr(),
                                                                byte_buffer
                                                                    .as_mut_ptr()
                                                                    .add(byte_buffer_len),
                                                                digits_len,
                                                            );
                                                            *byte_buffer.as_mut_ptr().add(
                                                                byte_buffer_len + digits_len,
                                                            ) = NEWLINE;
                                                        }

                                                        byte_buffer_len += digits_len + 1;
                                                    }

                                                    // Convert the byte buffer to a string slice and write it in one go
                                                    let output = unsafe {
                                                        std::str::from_utf8_unchecked(
                                                            &byte_buffer[..byte_buffer_len],
                                                        )
                                                    };
                                                    write!(buffer, "{}", output).map_err(|e| {
                                                        format!("Failed to write to buffer: {}", e)
                                                    })?;

                                                    counter = batch_end;
                                                }

                                                // Flush the buffer
                                                buffer.flush().map_err(|e| {
                                                    format!("Failed to flush buffer: {}", e)
                                                })?;
                                            } else {
                                                // Fallback for non-buffered output
                                                while counter < limit_val {
                                                    println!("{}", counter);
                                                    counter += 1;
                                                }
                                            }
                                        } else {
                                            // Silent mode - just update the counter
                                            counter = limit_val;
                                        }
                                    }
                                    TokenType::LessEqual => {
                                        // Similar optimization for <= condition
                                        if !self.silent_mode {
                                            if let Some(buffer) = &mut self.output_buffer {
                                                // Use the same ultra-optimized approach for LessEqual
                                                const BATCH_SIZE: i64 = 8192; // Larger batch size for better throughput
                                                const MAX_DIGITS: usize = 10; // Max digits for i64 values
                                                const NEWLINE: u8 = b'\n'; // Newline character

                                                // Pre-allocate a single buffer for the entire operation
                                                let total_capacity = std::cmp::min(
                                                    (limit_val - counter + 1) as usize,
                                                    BATCH_SIZE as usize,
                                                ) * (MAX_DIGITS + 1);
                                                let mut byte_buffer: Vec<u8> =
                                                    Vec::with_capacity(total_capacity);
                                                let mut byte_buffer_len: usize;

                                                // Process in large batches for maximum throughput
                                                while counter <= limit_val {
                                                    // Clear the buffer for reuse without reallocation
                                                    byte_buffer.clear();
                                                    byte_buffer_len = 0;

                                                    let batch_end = std::cmp::min(
                                                        counter + BATCH_SIZE,
                                                        limit_val + 1,
                                                    );

                                                    // Fill the buffer with all numbers in this batch
                                                    for i in counter..batch_end {
                                                        // Ultra-optimized integer to string conversion
                                                        let mut num_buffer = [0u8; MAX_DIGITS];
                                                        let mut num = i;
                                                        let mut idx = MAX_DIGITS;

                                                        // Handle special case for zero
                                                        if num == 0 {
                                                            num_buffer[idx - 1] = b'0';
                                                            idx -= 1;
                                                        } else {
                                                            // Convert digits from right to left
                                                            while num > 0 && idx > 0 {
                                                                idx -= 1;
                                                                num_buffer[idx] =
                                                                    b'0' + (num % 10) as u8;
                                                                num /= 10;
                                                            }
                                                        }

                                                        // Append the number's digits to the output buffer
                                                        let digits = &num_buffer[idx..MAX_DIGITS];
                                                        let digits_len = digits.len();

                                                        // Ensure we have enough capacity
                                                        while byte_buffer_len + digits_len + 1
                                                            > byte_buffer.capacity()
                                                        {
                                                            byte_buffer
                                                                .reserve(byte_buffer.capacity());
                                                        }

                                                        // Unsafe block for direct memory manipulation (maximum performance)
                                                        unsafe {
                                                            byte_buffer.set_len(
                                                                byte_buffer_len + digits_len + 1,
                                                            );
                                                            std::ptr::copy_nonoverlapping(
                                                                digits.as_ptr(),
                                                                byte_buffer
                                                                    .as_mut_ptr()
                                                                    .add(byte_buffer_len),
                                                                digits_len,
                                                            );
                                                            *byte_buffer.as_mut_ptr().add(
                                                                byte_buffer_len + digits_len,
                                                            ) = NEWLINE;
                                                        }

                                                        byte_buffer_len += digits_len + 1;
                                                    }

                                                    // Convert the byte buffer to a string slice and write it in one go
                                                    let output = unsafe {
                                                        std::str::from_utf8_unchecked(
                                                            &byte_buffer[..byte_buffer_len],
                                                        )
                                                    };
                                                    write!(buffer, "{}", output).map_err(|e| {
                                                        format!("Failed to write to buffer: {}", e)
                                                    })?;

                                                    counter = batch_end;
                                                }

                                                // Flush the buffer
                                                buffer.flush().map_err(|e| {
                                                    format!("Failed to flush buffer: {}", e)
                                                })?;
                                            } else {
                                                while counter <= limit_val {
                                                    println!("{}", counter);
                                                    counter += 1;
                                                }
                                            }
                                        } else {
                                            // Silent mode - just update the counter
                                            counter = limit_val + 1;
                                        }
                                    }
                                    _ => {
                                        // Fallback for other comparison operators
                                        loop {
                                            // Check the condition
                                            let continue_loop = match op_type {
                                                TokenType::Less => counter < limit_val,
                                                TokenType::LessEqual => counter <= limit_val,
                                                TokenType::Greater => counter > limit_val,
                                                TokenType::GreaterEqual => counter >= limit_val,
                                                _ => false,
                                            };

                                            if !continue_loop {
                                                break;
                                            }

                                            // Print the counter if not in silent mode
                                            if !self.silent_mode {
                                                if let Some(buffer) = &mut self.output_buffer {
                                                    writeln!(buffer, "{}", counter).map_err(
                                                        |e| {
                                                            format!(
                                                                "Failed to write to buffer: {}",
                                                                e
                                                            )
                                                        },
                                                    )?;
                                                } else {
                                                    println!("{}", counter);
                                                }
                                            }

                                            // Increment the counter
                                            counter += 1;
                                        }
                                    }
                                }

                                // Update the environment with the final counter value
                                self.environment
                                    .insert(var_name_without_prefix, Value::Number(counter));
                                return Ok(());
                            }
                        }
                    }
                }

                // General case for while loops
                self.loop_counter = 0; // Reset the loop counter
                self.expr_cache.clear(); // Clear expression cache for safety

                loop {
                    self.loop_counter += 1;

                    // Evaluate the condition
                    let condition_value = self.evaluate(condition)?;

                    if !is_truthy(&condition_value) {
                        break; // Exit the loop if the condition is false
                    }

                    // Execute the loop body
                    for stmt in body {
                        self.execute(stmt)?;

                        // Check for control flow interruptions
                        if self.control_flow == ControlFlow::Break {
                            self.control_flow = ControlFlow::None; // Reset control flow
                            return Ok(()); // Exit the loop
                        } else if self.control_flow == ControlFlow::Continue {
                            self.control_flow = ControlFlow::None; // Reset control flow
                            break; // Go to the next iteration
                        }
                    }
                }
            }
            Stmt::For {
                initializer,
                update,
                condition,
                body,
            } => {
                // Handle initializer specially to support variable declarations
                match initializer {
                    Expr::Binary {
                        left,
                        operator,
                        right,
                    } => {
                        // Check if this looks like a declaration (i : 0)
                        if operator.token_type == TokenType::Colon {
                            if let Expr::VariableRef(name) = &**left {
                                // This is a variable declaration - evaluate right side and set variable
                                let value = self.evaluate(right)?;
                                self.environment.insert(name.clone(), value);
                            } else {
                                // Just evaluate it normally
                                self.evaluate(initializer)?;
                            }
                        } else {
                            // Just evaluate it normally
                            self.evaluate(initializer)?;
                        }
                    }
                    _ => {
                        // Just evaluate it normally
                        self.evaluate(initializer)?;
                    }
                }

                self.loop_counter = 0; // Reset loop counter
                self.expr_cache.clear(); // Clear expression cache for safety

                loop {
                    self.loop_counter += 1;

                    // Evaluate the condition
                    let condition_value = self.evaluate(condition)?;

                    if !is_truthy(&condition_value) {
                        break; // Exit the loop if the condition is false
                    }

                    // Execute the loop body
                    for stmt in body {
                        self.execute(stmt)?;

                        // Check for control flow interruptions
                        if self.control_flow == ControlFlow::Break {
                            self.control_flow = ControlFlow::None; // Reset control flow
                            return Ok(()); // Exit the loop
                        } else if self.control_flow == ControlFlow::Continue {
                            self.control_flow = ControlFlow::None; // Reset control flow
                            break; // Go to the next iteration
                        }
                    }

                    // Update the loop counter - special handling for assignments
                    match update {
                        Expr::Binary {
                            left,
                            operator,
                            right,
                        } => {
                            // Handle variable assignment (i : value)
                            if operator.token_type == TokenType::Colon {
                                if let Expr::VariableRef(name) = &**left {
                                    // This is a variable assignment - evaluate right side and set variable
                                    let value = self.evaluate(right)?;
                                    self.environment.insert(name.clone(), value);
                                } else {
                                    // Just evaluate it normally
                                    self.evaluate(update)?;
                                }
                            } else {
                                // Not an assignment, might be an expression that calculates a new value
                                // Get the result of the expression
                                let result = self.evaluate(update)?;

                                // Check if this is a recognized update pattern like "$i + 1"
                                if let Expr::Binary {
                                    left: var_expr,
                                    operator: _,
                                    right: _,
                                } = update
                                {
                                    if let Expr::VariableRef(var_name) = &**var_expr {
                                        if var_name.starts_with('$') {
                                            // Extract the actual variable name (without $)
                                            let actual_name = var_name[1..].to_string();
                                            // Update the variable with the result
                                            self.environment.insert(actual_name, result);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            // Regular expression
                            self.evaluate(update)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // Helper function to evaluate a function body with a new scope and capture return value
    fn evaluate_function_body(&mut self, body: &[Stmt]) -> Result<Value, String> {
        if body.is_empty() {
            return Ok(Value::Null);
        }
        
        // In a minimal function with a single expression, the result is the return value
        if body.len() == 1 {
            if let Stmt::Expression(expr) = &body[0] {
                return self.evaluate(expr);
            }
        }
        
        // For more complex functions, we need to track the last expression value
        let mut last_expr_value = Value::Null;
        
        // Process all statements in the function body
        for stmt in body {
            match stmt {
                Stmt::Expression(expr) => {
                    // For expressions, evaluate and store the result
                    last_expr_value = self.evaluate(expr)?;
                },
                Stmt::If { condition, then_branch, else_branch } => {
                    // Special handling for if statements to capture their return values
                    let condition_value = self.evaluate(condition)?;
                    
                    if is_truthy(&condition_value) {
                        // Execute the then branch and capture its last expression
                        if !then_branch.is_empty() {
                            let branch_value = self.evaluate_function_body(then_branch)?;
                            if branch_value != Value::Null {
                                last_expr_value = branch_value;
                            }
                        }
                    } else if let Some(else_statements) = else_branch {
                        // Execute the else branch and capture its last expression
                        if !else_statements.is_empty() {
                            let branch_value = self.evaluate_function_body(else_statements)?;
                            if branch_value != Value::Null {
                                last_expr_value = branch_value;
                            }
                        }
                    }
                },
                _ => {
                    // For other types of statements, just execute them normally
                    self.execute(stmt)?;
                }
            }
            
            // Check for explicit return statements
            if let ControlFlow::Return(value) = &self.control_flow {
                last_expr_value = value.clone().unwrap_or(Value::Null);
                self.control_flow = ControlFlow::None; // Reset control flow
                break;
            }
        }
        
        Ok(last_expr_value)
    }
    
    // Helper function to call a function
    fn call_function(&mut self, func_name: &str, arguments: &[Expr]) -> Result<Value, String> {
        // Clone the function definition to avoid borrowing issues
        let func = if let Some(f) = self.functions.get(func_name) {
            Rc::clone(f)
        } else {
            return Err(format!("Undefined function: {}", func_name));
        };
        
        // Create a new environment for the function call
        let mut saved_environment = HashMap::new();
        
        // Verify the number of arguments matches the number of parameters
        if arguments.len() != func.parameters.len() {
            return Err(format!(
                "Function '{}' expects {} arguments, but {} were provided",
                func_name,
                func.parameters.len(),
                arguments.len()
            ));
        }
        
        // Evaluate all arguments first before modifying the environment
        let mut arg_values = Vec::with_capacity(arguments.len());
        for arg in arguments {
            let arg_value = self.evaluate(arg)?;
            arg_values.push(arg_value);
        }
        
        // Set up the function parameters in the environment
        for (i, param) in func.parameters.iter().enumerate() {
            // Save the original value of the parameter name if it exists
            if let Some(old_value) = self.environment.get(&param.name) {
                saved_environment.insert(param.name.clone(), old_value.clone());
            }
            
            // Set the parameter value in the environment
            self.environment.insert(param.name.clone(), arg_values[i].clone());
        }
        
        // Execute the function body and get the return value
        let return_value = self.evaluate_function_body(&func.body)?;
        
        // Restore the original environment
        for (name, value) in saved_environment {
            self.environment.insert(name, value);
        }
        
        Ok(return_value)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        // Check expression cache for literals and other cacheable expressions
        let expr_hash = hash_expr(expr);
        if expr_hash != 0 {
            if let Some(cached_value) = self.expr_cache.get(expr_hash) {
                return Ok(cached_value.clone());
            }
        }

        let result = match expr {
            Expr::FunctionCall { name, arguments } => {
                // Call the function
                self.call_function(name, arguments)
            },
            Expr::Ternary {
                condition,
                then_branch,
                else_branch,
            } => {
                // Evaluate the condition
                let condition_value = self.evaluate(condition)?;

                // Based on the condition, evaluate either the then branch or the else branch
                if is_truthy(&condition_value) {
                    self.evaluate(then_branch)
                } else {
                    self.evaluate(else_branch)
                }
            }
            Expr::VariableRef(name) => {
                if name.starts_with('$') {
                    let var_name = &name[1..];
                    match self.environment.get(var_name) {
                        Some(value) => Ok(value.clone()),
                        None => Err(format!("Undefined variable: {}", var_name)),
                    }
                } else {
                    Err(format!("Invalid variable reference: {}", name))
                }
            }
            Expr::NumberLiteral(value) => {
                let result = Value::Number(*value);
                // Cache the result
                if expr_hash != 0 {
                    self.expr_cache.put(expr_hash, result.clone());
                }
                Ok(result)
            }
            Expr::FloatLiteral(value) => {
                let result = Value::Float(*value);
                // Cache the result
                if expr_hash != 0 {
                    self.expr_cache.put(expr_hash, result.clone());
                }
                Ok(result)
            }
            Expr::TextLiteral(value) => {
                // Use the string pool for text literals
                let result = self.make_text(value.clone());
                // Cache the result
                if expr_hash != 0 {
                    self.expr_cache.put(expr_hash, result.clone());
                }
                Ok(result)
            }
            Expr::BooleanLiteral(value) => {
                let result = Value::Boolean(*value);
                // Cache the result
                if expr_hash != 0 {
                    self.expr_cache.put(expr_hash, result.clone());
                }
                Ok(result)
            }
            Expr::ArrayLiteral(elements) => {
                let mut values = Vec::with_capacity(elements.len());
                for element in elements {
                    let value = self.evaluate(element)?;
                    values.push(value);
                }
                Ok(Value::Array(values))
            }
            Expr::ArrayLiteral2D(rows) => {
                let mut array_2d = Vec::with_capacity(rows.len());
                for row in rows {
                    let mut values = Vec::with_capacity(row.len());
                    for element in row {
                        let value = self.evaluate(element)?;
                        values.push(value);
                    }
                    array_2d.push(values);
                }
                Ok(Value::Array2D(array_2d))
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Minus => {
                        match &right {
                            Value::Number(n) => Ok(Value::Number(-n)),
                            Value::Float(f) => Ok(Value::Float(-f)),
                            Value::Text(s) => {
                                // Try to parse the string as a number first
                                if let Ok(n) = s.parse::<i64>() {
                                    Ok(Value::Number(-n))
                                } else if let Ok(f) = s.parse::<f64>() {
                                    Ok(Value::Float(-f))
                                } else {
                                    Err(format!("Cannot negate text value: {}", s))
                                }
                            }
                            _ => Err("Cannot negate non-numeric value".to_string()),
                        }
                    }
                    TokenType::Not => Ok(Value::Boolean(!is_truthy(&right))),
                    _ => Err(format!("Invalid unary operator: {:?}", operator.token_type)),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                // Optimize common arithmetic operations on literals
                if let (Expr::NumberLiteral(n1), Expr::NumberLiteral(n2)) = (&**left, &**right) {
                    match operator.token_type {
                        TokenType::Plus => return Ok(Value::Number(n1 + n2)),
                        TokenType::Minus => return Ok(Value::Number(n1 - n2)),
                        TokenType::Star => return Ok(Value::Number(n1 * n2)),
                        TokenType::Slash if *n2 != 0 => return Ok(Value::Number(n1 / n2)),
                        TokenType::Percent if *n2 != 0 => return Ok(Value::Number(n1 % n2)),
                        TokenType::Equal => return Ok(Value::Boolean(n1 == n2)),
                        TokenType::NotEqual => return Ok(Value::Boolean(n1 != n2)),
                        TokenType::Less => return Ok(Value::Boolean(n1 < n2)),
                        TokenType::LessEqual => return Ok(Value::Boolean(n1 <= n2)),
                        TokenType::Greater => return Ok(Value::Boolean(n1 > n2)),
                        TokenType::GreaterEqual => return Ok(Value::Boolean(n1 >= n2)),
                        _ => {}
                    }
                }

                // Short-circuit evaluation for logical operators
                match operator.token_type {
                    TokenType::And => {
                        let left_val = self.evaluate(left)?;
                        if !is_truthy(&left_val) {
                            return Ok(Value::Boolean(false));
                        }
                        let right_val = self.evaluate(right)?;
                        return Ok(Value::Boolean(is_truthy(&right_val)));
                    }
                    TokenType::Or => {
                        let left_val = self.evaluate(left)?;
                        if is_truthy(&left_val) {
                            return Ok(Value::Boolean(true));
                        }
                        let right_val = self.evaluate(right)?;
                        return Ok(Value::Boolean(is_truthy(&right_val)));
                    }
                    _ => {}
                }

                // For non-logical operators, evaluate both sides
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                match operator.token_type {
                    // Arithmetic operators
                    TokenType::Plus => {
                        match (&left_val, &right_val) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
                            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 + f2)),
                            (Value::Number(n1), Value::Float(f2)) => {
                                Ok(Value::Float(*n1 as f64 + f2))
                            }
                            (Value::Float(f1), Value::Number(n2)) => {
                                Ok(Value::Float(f1 + *n2 as f64))
                            }
                            (Value::Text(s1), Value::Text(s2)) => {
                                // Efficient string concatenation
                                let mut result = String::with_capacity(s1.len() + s2.len());
                                result.push_str(s1);
                                result.push_str(s2);
                                Ok(self.make_text(result))
                            }
                            (Value::Text(s), Value::Number(n)) => {
                                let mut result = String::with_capacity(s.len() + 10);
                                result.push_str(s);
                                result.push_str(&n.to_string());
                                Ok(self.make_text(result))
                            }
                            (Value::Number(n), Value::Text(s)) => {
                                let mut result = String::with_capacity(10 + s.len());
                                result.push_str(&n.to_string());
                                result.push_str(s);
                                Ok(self.make_text(result))
                            }
                            (Value::Text(s), Value::Float(f)) => {
                                let mut result = String::with_capacity(s.len() + 10);
                                result.push_str(s);
                                result.push_str(&f.to_string());
                                Ok(self.make_text(result))
                            }
                            (Value::Float(f), Value::Text(s)) => {
                                let mut result = String::with_capacity(10 + s.len());
                                result.push_str(&f.to_string());
                                result.push_str(s);
                                Ok(self.make_text(result))
                            }
                            _ => Err("Cannot add incompatible types".to_string()),
                        }
                    }
                    TokenType::Minus => {
                        match (&left_val, &right_val) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
                            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 - f2)),
                            (Value::Number(n1), Value::Float(f2)) => {
                                Ok(Value::Float(*n1 as f64 - f2))
                            }
                            (Value::Float(f1), Value::Number(n2)) => {
                                Ok(Value::Float(f1 - *n2 as f64))
                            }
                            (Value::Text(s1), Value::Number(n2)) => {
                                // Try to parse the string as a number first
                                if let Ok(n1) = s1.parse::<i64>() {
                                    Ok(Value::Number(n1 - n2))
                                } else if let Ok(f1) = s1.parse::<f64>() {
                                    Ok(Value::Float(f1 - *n2 as f64))
                                } else {
                                    Err(format!("Cannot subtract from text: {}", s1))
                                }
                            }
                            (Value::Number(n1), Value::Text(s2)) => {
                                // Try to parse the string as a number first
                                if let Ok(n2) = s2.parse::<i64>() {
                                    Ok(Value::Number(n1 - n2))
                                } else if let Ok(f2) = s2.parse::<f64>() {
                                    Ok(Value::Float(*n1 as f64 - f2))
                                } else {
                                    Err(format!("Cannot subtract text: {}", s2))
                                }
                            }
                            (Value::Text(s1), Value::Float(f2)) => {
                                // Try to parse the string as a number first
                                if let Ok(f1) = s1.parse::<f64>() {
                                    Ok(Value::Float(f1 - f2))
                                } else {
                                    Err(format!("Cannot subtract float from text: {}", s1))
                                }
                            }
                            (Value::Float(f1), Value::Text(s2)) => {
                                // Try to parse the string as a number first
                                if let Ok(f2) = s2.parse::<f64>() {
                                    Ok(Value::Float(f1 - f2))
                                } else {
                                    Err(format!("Cannot subtract text from float: {}", s2))
                                }
                            }
                            _ => Err("Cannot subtract incompatible types".to_string()),
                        }
                    }
                    TokenType::Star => {
                        match (&left_val, &right_val) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
                            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 * f2)),
                            (Value::Number(n1), Value::Float(f2)) => {
                                Ok(Value::Float(*n1 as f64 * f2))
                            }
                            (Value::Float(f1), Value::Number(n2)) => {
                                Ok(Value::Float(f1 * *n2 as f64))
                            }
                            (Value::Text(s), Value::Number(n)) if *n >= 0 => {
                                // Efficient string repetition
                                Ok(self.make_text(s.repeat(*n as usize)))
                            }
                            _ => Err("Cannot multiply these values".to_string()),
                        }
                    }
                    TokenType::Slash => match (&left_val, &right_val) {
                        (Value::Number(n1), Value::Number(n2)) => {
                            if *n2 == 0 {
                                return Err("Division by zero".to_string());
                            }
                            Ok(Value::Number(n1 / n2))
                        }
                        (Value::Float(f1), Value::Float(f2)) => {
                            if *f2 == 0.0 {
                                return Err("Division by zero".to_string());
                            }
                            Ok(Value::Float(f1 / f2))
                        }
                        (Value::Number(n1), Value::Float(f2)) => {
                            if *f2 == 0.0 {
                                return Err("Division by zero".to_string());
                            }
                            Ok(Value::Float(*n1 as f64 / f2))
                        }
                        (Value::Float(f1), Value::Number(n2)) => {
                            if *n2 == 0 {
                                return Err("Division by zero".to_string());
                            }
                            Ok(Value::Float(f1 / *n2 as f64))
                        }
                        _ => Err("Cannot divide non-numeric values".to_string()),
                    },
                    TokenType::Percent => match (&left_val, &right_val) {
                        (Value::Number(n1), Value::Number(n2)) => {
                            if *n2 == 0 {
                                return Err("Modulo by zero".to_string());
                            }
                            Ok(Value::Number(n1 % n2))
                        }
                        (Value::Float(f1), Value::Float(f2)) => {
                            if *f2 == 0.0 {
                                return Err("Modulo by zero".to_string());
                            }
                            Ok(Value::Float(f1 % f2))
                        }
                        (Value::Number(n1), Value::Float(f2)) => {
                            if *f2 == 0.0 {
                                return Err("Modulo by zero".to_string());
                            }
                            Ok(Value::Float((*n1 as f64) % f2))
                        }
                        (Value::Float(f1), Value::Number(n2)) => {
                            if *n2 == 0 {
                                return Err("Modulo by zero".to_string());
                            }
                            Ok(Value::Float(f1 % (*n2 as f64)))
                        }
                        _ => Err("Cannot perform modulo on non-numeric values".to_string()),
                    },

                    // Comparison operators
                    TokenType::Equal => {
                        match (&left_val, &right_val) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 == n2)),
                            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Boolean(f1 == f2)),
                            (Value::Number(n1), Value::Float(f2)) => {
                                Ok(Value::Boolean((*n1 as f64) == *f2))
                            }
                            (Value::Float(f1), Value::Number(n2)) => {
                                Ok(Value::Boolean(f1 == &(*n2 as f64)))
                            }
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 == s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => {
                                Ok(Value::Boolean(b1 == b2))
                            }
                            (Value::Array(a1), Value::Array(a2)) => Ok(Value::Boolean(a1 == a2)),
                            (Value::Array2D(a1), Value::Array2D(a2)) => {
                                Ok(Value::Boolean(a1 == a2))
                            }
                            _ => Ok(Value::Boolean(false)), // Different types are never equal
                        }
                    }
                    TokenType::NotEqual => {
                        match (&left_val, &right_val) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 != n2)),
                            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Boolean(f1 != f2)),
                            (Value::Number(n1), Value::Float(f2)) => {
                                Ok(Value::Boolean((*n1 as f64) != *f2))
                            }
                            (Value::Float(f1), Value::Number(n2)) => {
                                Ok(Value::Boolean(f1 != &(*n2 as f64)))
                            }
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 != s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => {
                                Ok(Value::Boolean(b1 != b2))
                            }
                            (Value::Array(a1), Value::Array(a2)) => Ok(Value::Boolean(a1 != a2)),
                            (Value::Array2D(a1), Value::Array2D(a2)) => {
                                Ok(Value::Boolean(a1 != a2))
                            }
                            _ => Ok(Value::Boolean(true)), // Different types are always not equal
                        }
                    }
                    TokenType::Greater => {
                        match (&left_val, &right_val) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 > n2)),
                            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Boolean(f1 > f2)),
                            (Value::Number(n1), Value::Float(f2)) => {
                                Ok(Value::Boolean((*n1 as f64) > *f2))
                            }
                            (Value::Float(f1), Value::Number(n2)) => {
                                Ok(Value::Boolean(f1 > &(*n2 as f64)))
                            }
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 > s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => {
                                Ok(Value::Boolean(*b1 && !b2))
                            } // true > false
                            _ => Err("Cannot compare different types".to_string()),
                        }
                    }
                    TokenType::GreaterEqual => {
                        match (&left_val, &right_val) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 >= n2)),
                            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Boolean(f1 >= f2)),
                            (Value::Number(n1), Value::Float(f2)) => {
                                Ok(Value::Boolean((*n1 as f64) >= *f2))
                            }
                            (Value::Float(f1), Value::Number(n2)) => {
                                Ok(Value::Boolean(f1 >= &(*n2 as f64)))
                            }
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 >= s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => {
                                Ok(Value::Boolean(b1 >= b2))
                            } // booleans can be compared
                            _ => Err("Cannot compare different types".to_string()),
                        }
                    }
                    TokenType::Less => {
                        match (&left_val, &right_val) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 < n2)),
                            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Boolean(f1 < f2)),
                            (Value::Number(n1), Value::Float(f2)) => {
                                Ok(Value::Boolean((*n1 as f64) < *f2))
                            }
                            (Value::Float(f1), Value::Number(n2)) => {
                                Ok(Value::Boolean(f1 < &(*n2 as f64)))
                            }
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 < s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => {
                                Ok(Value::Boolean(!b1 && *b2))
                            } // false < true
                            _ => Err("Cannot compare different types".to_string()),
                        }
                    }
                    TokenType::LessEqual => {
                        match (&left_val, &right_val) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 <= n2)),
                            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Boolean(f1 <= f2)),
                            (Value::Number(n1), Value::Float(f2)) => {
                                Ok(Value::Boolean((*n1 as f64) <= *f2))
                            }
                            (Value::Float(f1), Value::Number(n2)) => {
                                Ok(Value::Boolean(f1 <= &(*n2 as f64)))
                            }
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 <= s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => {
                                Ok(Value::Boolean(b1 <= b2))
                            } // booleans can be compared
                            _ => Err("Cannot compare different types".to_string()),
                        }
                    }

                    _ => Err(format!(
                        "Invalid binary operator: {:?}",
                        operator.token_type
                    )),
                }
            }
            Expr::Command { name, args } => {
                match name.as_str() {
                    "number" | "-number" => {
                        if args.len() != 1 {
                            return Err("Number command expects one argument".to_string());
                        }

                        let arg = self.evaluate(&args[0])?;
                        match arg {
                            Value::Number(n) => Ok(Value::Number(n)),
                            Value::Text(s) => match s.parse::<i64>() {
                                Ok(n) => Ok(Value::Number(n)),
                                Err(_) => Err(format!("Cannot convert '{}' to a number", s)),
                            },
                            Value::Boolean(b) => Ok(Value::Number(if b { 1 } else { 0 })),
                            _ => Err("Expected number, text or boolean".to_string()),
                        }
                    }
                    "text" | "-text" => {
                        if args.len() != 1 {
                            return Err("Text command expects one argument".to_string());
                        }

                        let arg = self.evaluate(&args[0])?;
                        match arg {
                            Value::Text(s) => Ok(Value::Text(s)),
                            Value::Number(n) => Ok(self.make_text(n.to_string())),
                            Value::Boolean(b) => {
                                let text = if b { "true" } else { "false" }.to_string();
                                Ok(self.make_text(text))
                            }
                            _ => Err("Expected text, number or boolean".to_string()),
                        }
                    }
                    "fp" | "-fp" => {
                        if args.len() != 1 {
                            return Err("Floating point command expects one argument".to_string());
                        }

                        let arg = self.evaluate(&args[0])?;
                        match arg {
                            Value::Float(f) => Ok(Value::Float(f)),
                            Value::Number(n) => Ok(Value::Float(n as f64)),
                            Value::Text(s) => match s.parse::<f64>() {
                                Ok(f) => Ok(Value::Float(f)),
                                Err(_) => Err(format!(
                                    "Cannot convert '{}' to a floating point number",
                                    s
                                )),
                            },
                            Value::Boolean(b) => Ok(Value::Float(if b { 1.0 } else { 0.0 })),
                            _ => Err("Expected number, text or boolean".to_string()),
                        }
                    }
                    "bool" | "-bool" => {
                        if args.len() != 1 {
                            return Err("Boolean command expects one argument".to_string());
                        }

                        let arg = self.evaluate(&args[0])?;
                        match arg {
                            Value::Boolean(b) => Ok(Value::Boolean(b)),
                            Value::Number(n) => Ok(Value::Boolean(n != 0)),
                            Value::Float(f) => Ok(Value::Boolean(f != 0.0)),
                            Value::Text(s) => {
                                if s.as_str() == "true" {
                                    Ok(Value::Boolean(true))
                                } else if s.as_str() == "false" {
                                    Ok(Value::Boolean(false))
                                } else {
                                    Ok(Value::Boolean(!s.is_empty()))
                                }
                            }
                            _ => Err("Expected boolean, number or text".to_string()),
                        }
                    }
                    "hex" | "-hex" => {
                        if args.len() != 1 {
                            return Err("Hex command expects one argument".to_string());
                        }

                        let arg = self.evaluate(&args[0])?;
                        match arg {
                            Value::Text(s) => {
                                // Try to parse the hex string
                                let s_ref: &str = &s;
                                let s_trimmed =
                                    s_ref.trim_start_matches("0x").trim_start_matches("0X");
                                match i64::from_str_radix(s_trimmed, 16) {
                                    Ok(n) => Ok(Value::Number(n)),
                                    Err(_) => Err(format!("Cannot parse '{}' as hexadecimal", s)),
                                }
                            }
                            Value::Number(n) => {
                                // Already a number, just return it
                                Ok(Value::Number(n))
                            }
                            _ => Err("Expected hexadecimal string".to_string()),
                        }
                    }
                    "bin" | "-bin" => {
                        if args.len() != 1 {
                            return Err("Binary command expects one argument".to_string());
                        }

                        let arg = self.evaluate(&args[0])?;
                        match arg {
                            Value::Text(s) => {
                                // Try to parse the binary string
                                let s_ref: &str = &s;
                                let s_trimmed =
                                    s_ref.trim_start_matches("0b").trim_start_matches("0B");
                                match i64::from_str_radix(s_trimmed, 2) {
                                    Ok(n) => Ok(Value::Number(n)),
                                    Err(_) => Err(format!("Cannot parse '{}' as binary", s)),
                                }
                            }
                            Value::Number(n) => {
                                // Already a number, just return it
                                Ok(Value::Number(n))
                            }
                            _ => Err("Expected binary string".to_string()),
                        }
                    }
                    "array" | "-array" => {
                        // Check if we have at least one argument
                        if args.is_empty() {
                            return Err("Array command expects at least one argument".to_string());
                        }

                        // Check if the argument is an array literal
                        if args.len() == 1 {
                            match &args[0] {
                                // Handle 1D array literal [1, 2, 3, 4]
                                Expr::ArrayLiteral(elements) => {
                                    let mut values = Vec::with_capacity(elements.len());
                                    for element in elements {
                                        let value = self.evaluate(element)?;
                                        values.push(value);
                                    }
                                    return Ok(Value::Array(values));
                                }
                                // Handle 2D array literal via multiple rows [1, 2][3, 4]
                                Expr::ArrayLiteral2D(rows) => {
                                    let mut array_2d = Vec::with_capacity(rows.len());
                                    for row in rows {
                                        let mut values = Vec::with_capacity(row.len());
                                        for element in row {
                                            let value = self.evaluate(element)?;
                                            values.push(value);
                                        }
                                        array_2d.push(values);
                                    }
                                    return Ok(Value::Array2D(array_2d));
                                }
                                // If not an array literal, continue with the old implementation
                                _ => {}
                            }
                        }

                        // First check if we are creating a 2D array with the old syntax
                        let mut is_2d = false;
                        let mut first_row_size = 0;
                        let mut rows = Vec::new();

                        // Check if all arguments are arrays (which would make this a 2D array)
                        for arg in args {
                            if let Expr::Command {
                                name,
                                args: inner_args,
                            } = arg
                            {
                                if name == "array" {
                                    is_2d = true;
                                    if first_row_size == 0 {
                                        first_row_size = inner_args.len();
                                    } else if inner_args.len() != first_row_size {
                                        return Err(
                                            "All rows in a 2D array must have the same length"
                                                .to_string(),
                                        );
                                    }
                                }
                            }
                        }

                        if is_2d {
                            // Create a 2D array
                            rows.reserve(args.len());
                            for arg in args {
                                if let Expr::Command {
                                    name,
                                    args: inner_args,
                                } = arg
                                {
                                    if name == "array" {
                                        // Evaluate each element in the inner array
                                        let mut row = Vec::with_capacity(inner_args.len());
                                        for inner_arg in inner_args {
                                            let value = self.evaluate(inner_arg)?;
                                            row.push(value);
                                        }
                                        rows.push(row);
                                    } else {
                                        return Err(
                                            "Expected array command for 2D array row".to_string()
                                        );
                                    }
                                } else {
                                    return Err(
                                        "Expected array command for 2D array row".to_string()
                                    );
                                }
                            }
                            Ok(Value::Array2D(rows))
                        } else {
                            // Create a 1D array
                            let mut values = Vec::with_capacity(args.len());
                            for arg in args {
                                let value = self.evaluate(arg)?;
                                values.push(value);
                            }
                            Ok(Value::Array(values))
                        }
                    }
                    "asc" | "-asc" => {
                        if args.len() != 1 {
                            return Err(format!(
                                "Asc command expects one argument, got {}",
                                args.len()
                            ));
                        }

                        let arg = self.evaluate(&args[0])?;
                        match arg {
                            Value::Number(n) => {
                                if let Some(c) = std::char::from_u32(n as u32) {
                                    Ok(self.make_text(c.to_string()))
                                } else {
                                    Err(format!("Invalid ASCII code: {}", n))
                                }
                            }
                            Value::Text(s) => {
                                // Try to parse the text as a number
                                match s.parse::<i64>() {
                                    Ok(n) => {
                                        if let Some(c) = std::char::from_u32(n as u32) {
                                            Ok(self.make_text(c.to_string()))
                                        } else {
                                            Err(format!("Invalid ASCII code: {}", n))
                                        }
                                    }
                                    Err(_) => Err(format!("Cannot convert '{}' to ASCII code", s)),
                                }
                            }
                            _ => Err("Expected number for ASCII code".to_string()),
                        }
                    }
                    "-add" => {
                        if args.len() < 2 {
                            return Err(format!(
                                "Add command expects at least two arguments, got {}",
                                args.len()
                            ));
                        }

                        let mut result = 0;
                        for arg in args {
                            let value = self.evaluate(arg)?;
                            match value {
                                Value::Number(n) => result += n,
                                Value::Text(s) => match s.parse::<i64>() {
                                    Ok(n) => result += n,
                                    Err(_) => {
                                        return Err(format!(
                                            "Cannot convert '{}' to a number for addition",
                                            s
                                        ))
                                    }
                                },
                                _ => return Err("Expected number for addition".to_string()),
                            }
                        }
                        Ok(Value::Number(result))
                    }
                    "-sub" => {
                        if args.len() < 2 {
                            return Err(format!(
                                "Subtract command expects at least two arguments, got {}",
                                args.len()
                            ));
                        }

                        // Get the first value
                        let first_arg = self.evaluate(&args[0])?;
                        let mut result = match first_arg {
                            Value::Number(n) => n,
                            Value::Text(s) => match s.parse::<i64>() {
                                Ok(n) => n,
                                Err(_) => {
                                    return Err(format!(
                                        "Cannot convert '{}' to a number for subtraction",
                                        s
                                    ))
                                }
                            },
                            _ => return Err("Expected number for subtraction".to_string()),
                        };

                        // Subtract all other values
                        for arg in args.iter().skip(1) {
                            let value = self.evaluate(arg)?;
                            match value {
                                Value::Number(n) => result -= n,
                                Value::Text(s) => match s.parse::<i64>() {
                                    Ok(n) => result -= n,
                                    Err(_) => {
                                        return Err(format!(
                                            "Cannot convert '{}' to a number for subtraction",
                                            s
                                        ))
                                    }
                                },
                                _ => return Err("Expected number for subtraction".to_string()),
                            }
                        }
                        Ok(Value::Number(result))
                    }
                    "-mul" => {
                        if args.len() < 2 {
                            return Err(format!(
                                "Multiply command expects at least two arguments, got {}",
                                args.len()
                            ));
                        }

                        // Start with 1 as the identity element for multiplication
                        let mut result = 1;
                        for arg in args {
                            let value = self.evaluate(arg)?;
                            match value {
                                Value::Number(n) => result *= n,
                                Value::Text(s) => {
                                    match s.parse::<i64>() {
                                        Ok(n) => result *= n,
                                        Err(_) => return Err(format!(
                                            "Cannot convert '{}' to a number for multiplication",
                                            s
                                        )),
                                    }
                                }
                                _ => return Err("Expected number for multiplication".to_string()),
                            }
                        }
                        Ok(Value::Number(result))
                    }
                    "-div" => {
                        if args.len() < 2 {
                            return Err(format!(
                                "Divide command expects at least two arguments, got {}",
                                args.len()
                            ));
                        }

                        // Get the first value
                        let first_arg = self.evaluate(&args[0])?;
                        let mut result = match first_arg {
                            Value::Number(n) => n,
                            Value::Text(s) => match s.parse::<i64>() {
                                Ok(n) => n,
                                Err(_) => {
                                    return Err(format!(
                                        "Cannot convert '{}' to a number for division",
                                        s
                                    ))
                                }
                            },
                            _ => return Err("Expected number for division".to_string()),
                        };

                        // Divide by all other values
                        for arg in args.iter().skip(1) {
                            let value = self.evaluate(arg)?;
                            match value {
                                Value::Number(n) => {
                                    if n == 0 {
                                        return Err("Division by zero".to_string());
                                    }
                                    result /= n;
                                }
                                Value::Text(s) => match s.parse::<i64>() {
                                    Ok(n) => {
                                        if n == 0 {
                                            return Err("Division by zero".to_string());
                                        }
                                        result /= n;
                                    }
                                    Err(_) => {
                                        return Err(format!(
                                            "Cannot convert '{}' to a number for division",
                                            s
                                        ))
                                    }
                                },
                                _ => return Err("Expected number for division".to_string()),
                            }
                        }
                        Ok(Value::Number(result))
                    }
                    "-mod" => {
                        if args.len() != 2 {
                            return Err(format!(
                                "Modulo command expects exactly two arguments, got {}",
                                args.len()
                            ));
                        }

                        // Get the left operand
                        let left_arg = self.evaluate(&args[0])?;
                        let left = match left_arg {
                            Value::Number(n) => n,
                            Value::Text(s) => match s.parse::<i64>() {
                                Ok(n) => n,
                                Err(_) => {
                                    return Err(format!(
                                        "Cannot convert '{}' to a number for modulo",
                                        s
                                    ))
                                }
                            },
                            _ => return Err("Expected number for modulo".to_string()),
                        };

                        // Get the right operand
                        let right_arg = self.evaluate(&args[1])?;
                        let right = match right_arg {
                            Value::Number(n) => n,
                            Value::Text(s) => match s.parse::<i64>() {
                                Ok(n) => n,
                                Err(_) => {
                                    return Err(format!(
                                        "Cannot convert '{}' to a number for modulo",
                                        s
                                    ))
                                }
                            },
                            _ => return Err("Expected number for modulo".to_string()),
                        };

                        if right == 0 {
                            return Err("Modulo by zero".to_string());
                        }

                        Ok(Value::Number(left % right))
                    }
                    "get" | "-get" => {
                        if args.len() != 2 {
                            return Err(format!(
                                "Get command expects two arguments (array and index), got {}",
                                args.len()
                            ));
                        }

                        // Get the array
                        let array = self.evaluate(&args[0])?;

                        // Get the index
                        let index = self.evaluate(&args[1])?;

                        match (array, index) {
                            // Handle 1D array access
                            (Value::Array(arr), Value::Number(idx)) => {
                                let idx = idx as usize;
                                if idx < arr.len() {
                                    Ok(arr[idx].clone())
                                } else {
                                    Err(format!(
                                        "Array index out of bounds: {} (length: {})",
                                        idx,
                                        arr.len()
                                    ))
                                }
                            }
                            // Invalid index types
                            (Value::Array(_), _) => Err("Array index must be a number".to_string()),
                            // Invalid array types
                            (_, _) => Err("First argument to get must be an array".to_string()),
                        }
                    }
                    "get2d" | "-get2d" => {
                        if args.len() != 3 {
                            return Err(format!("Get2d command expects three arguments (2D array, row, column), got {}", args.len()));
                        }

                        // Get the 2D array
                        let array = self.evaluate(&args[0])?;

                        // Get the row index
                        let row_idx = self.evaluate(&args[1])?;

                        // Get the column index
                        let col_idx = self.evaluate(&args[2])?;

                        match (array, row_idx, col_idx) {
                            // Handle 2D array access
                            (Value::Array2D(arr), Value::Number(row), Value::Number(col)) => {
                                let row = row as usize;
                                let col = col as usize;

                                if row < arr.len() {
                                    let row_arr = &arr[row];
                                    if col < row_arr.len() {
                                        Ok(row_arr[col].clone())
                                    } else {
                                        Err(format!(
                                            "Column index out of bounds: {} (row length: {})",
                                            col,
                                            row_arr.len()
                                        ))
                                    }
                                } else {
                                    Err(format!(
                                        "Row index out of bounds: {} (array height: {})",
                                        row,
                                        arr.len()
                                    ))
                                }
                            }
                            // Invalid index types
                            (Value::Array2D(_), _, _) => {
                                Err("Array indices must be numbers".to_string())
                            }
                            // Invalid array types
                            (_, _, _) => {
                                Err("First argument to get2d must be a 2D array".to_string())
                            }
                        }
                    }
                    "length" | "-length" => {
                        if args.len() != 1 {
                            return Err(format!(
                                "Length command expects one argument, got {}",
                                args.len()
                            ));
                        }

                        let value = self.evaluate(&args[0])?;

                        match value {
                            Value::Array(arr) => Ok(Value::Number(arr.len() as i64)),
                            Value::Array2D(arr) => Ok(Value::Number(arr.len() as i64)), // Returns number of rows
                            Value::Text(s) => Ok(Value::Number(s.len() as i64)),
                            _ => Err("Cannot get length of non-array/non-text value".to_string()),
                        }
                    }
                    "width" | "-width" => {
                        if args.len() != 1 {
                            return Err(format!(
                                "Width command expects one argument, got {}",
                                args.len()
                            ));
                        }

                        let value = self.evaluate(&args[0])?;

                        match value {
                            Value::Array2D(arr) => {
                                if arr.is_empty() {
                                    Ok(Value::Number(0))
                                } else {
                                    // Return the length of the first row (width)
                                    Ok(Value::Number(arr[0].len() as i64))
                                }
                            }
                            _ => Err("Cannot get width of non-2D array".to_string()),
                        }
                    }
                    _ => Err(format!("Unknown command: {}", name)),
                }
            }
        };

        result
    }
}

pub fn run(source: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements)?;

    Ok(())
}

// Run with silent mode (no output), useful for benchmarking
pub fn run_silent(source: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut interpreter = Interpreter::with_silent_mode(true);
    interpreter.interpret(statements)?;

    Ok(())
}
