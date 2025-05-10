use std::collections::HashMap;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{Parser, Stmt, Expr};

// Control flow handling
#[derive(Debug, Clone, PartialEq)]
enum ControlFlow {
    None,
    Break,
    Continue,
}

pub struct Interpreter {
    environment: HashMap<String, Value>,
    control_flow: ControlFlow,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Text(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Text(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: HashMap::new(),
            control_flow: ControlFlow::None,
        }
    }
    
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for stmt in statements {
            self.execute(stmt)?;

            // Check for control flow interruptions at the top level
            if self.control_flow != ControlFlow::None {
                return Err("Unexpected break or continue outside of loop".to_string());
            }
        }

        Ok(())
    }

    fn execute(&mut self, stmt: Stmt) -> Result<(), String> {
        // Check for control flow interruptions before executing any statement
        if self.control_flow != ControlFlow::None {
            return Ok(());  // Skip this statement if we're in a break or continue state
        }

        match stmt {
            Stmt::Declaration { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment.insert(name, value);
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Command { name, args } => {
                match name.as_str() {
                    "print" | "-print" => {
                        if args.is_empty() {
                            println!();
                            return Ok(());
                        }

                        let mut result = String::new();
                        for (i, arg) in args.iter().enumerate() {
                            let value = self.evaluate(arg.clone())?;
                            result.push_str(&value.to_string());

                            // Add space between arguments (but not after the last one)
                            if i < args.len() - 1 {
                                result.push(' ');
                            }
                        }
                        println!("{}", result);
                    }
                    // Add more commands as needed
                    _ => return Err(format!("Unknown command: {}", name)),
                }
            }
            Stmt::Print(exprs) => {
                if exprs.is_empty() {
                    println!();
                    return Ok(());
                }

                let mut result = String::new();
                for (i, expr) in exprs.iter().enumerate() {
                    let value = self.evaluate(expr.clone())?;
                    result.push_str(&value.to_string());

                    // Add space between arguments (but not after the last one)
                    if i < exprs.len() - 1 {
                        result.push(' ');
                    }
                }
                println!("{}", result);
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
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate(condition)?;

                // Determine if the condition is "truthy"
                let is_truthy = match condition_value {
                    Value::Boolean(b) => b,
                    Value::Number(n) => n != 0,
                    Value::Text(s) => !s.is_empty(),
                    Value::Null => false,
                };

                if is_truthy {
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
                loop {
                    // Evaluate the condition
                    let condition_value = self.evaluate(condition.clone())?;

                    // Determine if the condition is "truthy"
                    let is_truthy = match condition_value {
                        Value::Boolean(b) => b,
                        Value::Number(n) => n != 0,
                        Value::Text(s) => !s.is_empty(),
                        Value::Null => false,
                    };

                    if !is_truthy {
                        break;  // Exit the loop if the condition is false
                    }

                    // Execute the loop body
                    for stmt in body.clone() {
                        self.execute(stmt)?;

                        // Check for control flow interruptions
                        if self.control_flow == ControlFlow::Break {
                            self.control_flow = ControlFlow::None;  // Reset control flow
                            return Ok(());  // Exit the loop
                        } else if self.control_flow == ControlFlow::Continue {
                            self.control_flow = ControlFlow::None;  // Reset control flow
                            break;  // Go to the next iteration
                        }
                    }
                }
            }
            Stmt::For { initializer, update, condition, body } => {
                // Handle initializer specially to support variable declarations
                match &initializer {
                    Expr::Binary { left, operator, right } => {
                        // Check if this looks like a declaration (i : 0)
                        if operator.token_type == TokenType::Colon {
                            if let Expr::VariableRef(name) = &**left {
                                // This is a variable declaration - evaluate right side and set variable
                                let value = self.evaluate(*right.clone())?;
                                self.environment.insert(name.clone(), value);
                            } else {
                                // Just evaluate it normally
                                self.evaluate(initializer.clone())?;
                            }
                        } else {
                            // Just evaluate it normally
                            self.evaluate(initializer.clone())?;
                        }
                    }
                    _ => {
                        // Just evaluate it normally
                        self.evaluate(initializer.clone())?;
                    }
                }

                loop {
                    // Evaluate the condition
                    let condition_value = self.evaluate(condition.clone())?;

                    // Determine if the condition is "truthy"
                    let is_truthy = match condition_value {
                        Value::Boolean(b) => b,
                        Value::Number(n) => n != 0,
                        Value::Text(s) => !s.is_empty(),
                        Value::Null => false,
                    };

                    if !is_truthy {
                        break;  // Exit the loop if the condition is false
                    }

                    // Execute the loop body
                    for stmt in body.clone() {
                        self.execute(stmt)?;

                        // Check for control flow interruptions
                        if self.control_flow == ControlFlow::Break {
                            self.control_flow = ControlFlow::None;  // Reset control flow
                            return Ok(());  // Exit the loop
                        } else if self.control_flow == ControlFlow::Continue {
                            self.control_flow = ControlFlow::None;  // Reset control flow
                            break;  // Go to the next iteration
                        }
                    }

                    // Update the loop counter - special handling for assignments
                    match &update {
                        Expr::Binary { left, operator, right } => {
                            // Handle variable assignment (i : value)
                            if operator.token_type == TokenType::Colon {
                                if let Expr::VariableRef(name) = &**left {
                                    // This is a variable assignment - evaluate right side and set variable
                                    let value = self.evaluate(*right.clone())?;
                                    self.environment.insert(name.clone(), value);
                                } else {
                                    // Just evaluate it normally
                                    self.evaluate(update.clone())?;
                                }
                            } else {
                                // Not an assignment, might be an expression that calculates a new value
                                // Get the result of the expression
                                let result = self.evaluate(update.clone())?;

                                // Check if this is a recognized update pattern like "$i + 1"
                                if let Expr::Binary { left: var_expr, operator: _, right: _ } = &update {
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
                            self.evaluate(update.clone())?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn evaluate(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Ternary { condition, then_branch, else_branch } => {
                // Evaluate the condition
                let condition_value = self.evaluate(*condition)?;

                // Check if the condition is truthy
                let is_truthy = match condition_value {
                    Value::Boolean(b) => b,
                    Value::Number(n) => n != 0,
                    Value::Text(s) => !s.is_empty(),
                    Value::Null => false,
                };

                // Based on the condition, evaluate either the then branch or the else branch
                if is_truthy {
                    self.evaluate(*then_branch)
                } else {
                    self.evaluate(*else_branch)
                }
            },
            Expr::VariableRef(name) => {
                if name.starts_with('$') {
                    let var_name = name[1..].to_string();
                    match self.environment.get(&var_name) {
                        Some(value) => Ok(value.clone()),
                        None => Err(format!("Undefined variable: {}", var_name)),
                    }
                } else {
                    Err(format!("Invalid variable reference: {}", name))
                }
            }
            Expr::NumberLiteral(value) => Ok(Value::Number(value)),
            Expr::TextLiteral(value) => Ok(Value::Text(value)),
            Expr::BooleanLiteral(value) => Ok(Value::Boolean(value)),
            Expr::Grouping { expression } => {
                self.evaluate(*expression)
            }
            Expr::Unary { operator, right } => {
                let right = self.evaluate(*right)?;
                
                match operator.token_type {
                    TokenType::Minus => {
                        match right {
                            Value::Number(n) => Ok(Value::Number(-n)),
                            Value::Text(s) => {
                                match s.parse::<i64>() {
                                    Ok(n) => Ok(Value::Number(-n)),
                                    Err(_) => Err(format!("Cannot negate text value: {}", s)),
                                }
                            }
                            _ => Err("Cannot negate non-numeric value".to_string()),
                        }
                    }
                    TokenType::Not => {
                        match right {
                            Value::Boolean(b) => Ok(Value::Boolean(!b)),
                            Value::Number(n) => Ok(Value::Boolean(n == 0)),
                            Value::Text(s) => Ok(Value::Boolean(s.is_empty())),
                            _ => Err("Cannot apply logical not to non-boolean value".to_string()),
                        }
                    }
                    _ => Err(format!("Invalid unary operator: {:?}", operator.token_type)),
                }
            }
            Expr::Binary { left, operator, right } => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                
                match operator.token_type {
                    // Arithmetic operators
                    TokenType::Plus => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Text(s1 + &s2)),
                            (Value::Text(s), Value::Number(n)) => Ok(Value::Text(s + &n.to_string())),
                            (Value::Number(n), Value::Text(s)) => Ok(Value::Text(n.to_string() + &s)),
                            _ => Err("Cannot add incompatible types".to_string()),
                        }
                    }
                    TokenType::Minus => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
                            (Value::Text(s1), Value::Number(n2)) => {
                                match s1.parse::<i64>() {
                                    Ok(n1) => Ok(Value::Number(n1 - n2)),
                                    Err(_) => Err(format!("Cannot subtract from text: {}", s1)),
                                }
                            }
                            (Value::Number(n1), Value::Text(s2)) => {
                                match s2.parse::<i64>() {
                                    Ok(n2) => Ok(Value::Number(n1 - n2)),
                                    Err(_) => Err(format!("Cannot subtract text: {}", s2)),
                                }
                            }
                            _ => Err("Cannot subtract incompatible types".to_string()),
                        }
                    }
                    TokenType::Star => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
                            _ => Err("Cannot multiply non-numeric values".to_string()),
                        }
                    }
                    TokenType::Slash => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => {
                                if n2 == 0 {
                                    return Err("Division by zero".to_string());
                                }
                                Ok(Value::Number(n1 / n2))
                            }
                            _ => Err("Cannot divide non-numeric values".to_string()),
                        }
                    }
                    TokenType::Percent => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => {
                                if n2 == 0 {
                                    return Err("Modulo by zero".to_string());
                                }
                                Ok(Value::Number(n1 % n2))
                            }
                            _ => Err("Cannot perform modulo on non-numeric values".to_string()),
                        }
                    }
                    
                    // Comparison operators
                    TokenType::Equal => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 == n2)),
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 == s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => Ok(Value::Boolean(b1 == b2)),
                            _ => Ok(Value::Boolean(false)), // Different types are never equal
                        }
                    }
                    TokenType::NotEqual => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 != n2)),
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 != s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => Ok(Value::Boolean(b1 != b2)),
                            _ => Ok(Value::Boolean(true)), // Different types are always not equal
                        }
                    }
                    TokenType::Greater => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 > n2)),
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 > s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => Ok(Value::Boolean(b1 && !b2)), // true > false
                            _ => Err("Cannot compare different types".to_string()),
                        }
                    }
                    TokenType::GreaterEqual => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 >= n2)),
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 >= s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => Ok(Value::Boolean(b1 >= b2)), // booleans can be compared
                            _ => Err("Cannot compare different types".to_string()),
                        }
                    }
                    TokenType::Less => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 < n2)),
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 < s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => Ok(Value::Boolean(!b1 && b2)), // false < true
                            _ => Err("Cannot compare different types".to_string()),
                        }
                    }
                    TokenType::LessEqual => {
                        match (left, right) {
                            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 <= n2)),
                            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Boolean(s1 <= s2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => Ok(Value::Boolean(b1 <= b2)), // booleans can be compared
                            _ => Err("Cannot compare different types".to_string()),
                        }
                    }
                    
                    // Logical operators
                    TokenType::And => {
                        let left_bool = match left {
                            Value::Boolean(b) => b,
                            Value::Number(n) => n != 0,
                            Value::Text(s) => !s.is_empty(),
                            _ => false,
                        };
                        
                        if !left_bool {
                            return Ok(Value::Boolean(false)); // Short-circuit evaluation
                        }
                        
                        let right_bool = match right {
                            Value::Boolean(b) => b,
                            Value::Number(n) => n != 0,
                            Value::Text(s) => !s.is_empty(),
                            _ => false,
                        };
                        
                        Ok(Value::Boolean(right_bool))
                    }
                    TokenType::Or => {
                        let left_bool = match left {
                            Value::Boolean(b) => b,
                            Value::Number(n) => n != 0,
                            Value::Text(s) => !s.is_empty(),
                            _ => false,
                        };
                        
                        if left_bool {
                            return Ok(Value::Boolean(true)); // Short-circuit evaluation
                        }
                        
                        let right_bool = match right {
                            Value::Boolean(b) => b,
                            Value::Number(n) => n != 0,
                            Value::Text(s) => !s.is_empty(),
                            _ => false,
                        };
                        
                        Ok(Value::Boolean(right_bool))
                    }
                    
                    _ => Err(format!("Invalid binary operator: {:?}", operator.token_type)),
                }
            }
            Expr::Command { name, args } => {
                match name.as_str() {
                    "number" | "-number" => {
                        if args.len() != 1 {
                            return Err("Number command expects one argument".to_string());
                        }

                        let arg = self.evaluate(args[0].clone())?;
                        match arg {
                            Value::Number(n) => Ok(Value::Number(n)),
                            Value::Text(s) => {
                                match s.parse::<i64>() {
                                    Ok(n) => Ok(Value::Number(n)),
                                    Err(_) => Err(format!("Cannot convert '{}' to a number", s)),
                                }
                            }
                            Value::Boolean(b) => Ok(Value::Number(if b { 1 } else { 0 })),
                            _ => Err("Expected number, text or boolean".to_string()),
                        }
                    }
                    "text" | "-text" => {
                        if args.len() != 1 {
                            return Err("Text command expects one argument".to_string());
                        }

                        let arg = self.evaluate(args[0].clone())?;
                        match arg {
                            Value::Text(s) => Ok(Value::Text(s)),
                            Value::Number(n) => Ok(Value::Text(n.to_string())),
                            Value::Boolean(b) => Ok(Value::Text(if b { "true" } else { "false" }.to_string())),
                            _ => Err("Expected text, number or boolean".to_string()),
                        }
                    }
                    "bool" | "-bool" => {
                        if args.len() != 1 {
                            return Err("Boolean command expects one argument".to_string());
                        }

                        let arg = self.evaluate(args[0].clone())?;
                        match arg {
                            Value::Boolean(b) => Ok(Value::Boolean(b)),
                            Value::Number(n) => Ok(Value::Boolean(n != 0)),
                            Value::Text(s) => {
                                if s == "true" {
                                    Ok(Value::Boolean(true))
                                } else if s == "false" {
                                    Ok(Value::Boolean(false))
                                } else {
                                    Ok(Value::Boolean(!s.is_empty()))
                                }
                            }
                            _ => Err("Expected boolean, number or text".to_string()),
                        }
                    }
                    "asc" | "-asc" => {
                        if args.len() != 1 {
                            return Err(format!("Asc command expects one argument, got {}", args.len()));
                        }

                        let arg = self.evaluate(args[0].clone())?;
                        match arg {
                            Value::Number(n) => {
                                if let Some(c) = std::char::from_u32(n as u32) {
                                    Ok(Value::Text(c.to_string()))
                                } else {
                                    Err(format!("Invalid ASCII code: {}", n))
                                }
                            }
                            Value::Text(s) => {
                                // Try to parse the text as a number
                                match s.parse::<i64>() {
                                    Ok(n) => {
                                        if let Some(c) = std::char::from_u32(n as u32) {
                                            Ok(Value::Text(c.to_string()))
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
                            return Err(format!("Add command expects at least two arguments, got {}", args.len()));
                        }
                        
                        let mut result = 0;
                        for arg in args {
                            let value = self.evaluate(arg.clone())?;
                            match value {
                                Value::Number(n) => result += n,
                                Value::Text(s) => {
                                    match s.parse::<i64>() {
                                        Ok(n) => result += n,
                                        Err(_) => return Err(format!("Cannot convert '{}' to a number for addition", s)),
                                    }
                                }
                                _ => return Err("Expected number for addition".to_string()),
                            }
                        }
                        Ok(Value::Number(result))
                    }
                    "-sub" => {
                        if args.len() < 2 {
                            return Err(format!("Subtract command expects at least two arguments, got {}", args.len()));
                        }
                        
                        // Get the first value
                        let first_arg = self.evaluate(args[0].clone())?;
                        let mut result = match first_arg {
                            Value::Number(n) => n,
                            Value::Text(s) => {
                                match s.parse::<i64>() {
                                    Ok(n) => n,
                                    Err(_) => return Err(format!("Cannot convert '{}' to a number for subtraction", s)),
                                }
                            }
                            _ => return Err("Expected number for subtraction".to_string()),
                        };
                        
                        // Subtract all other values
                        for arg in args.iter().skip(1) {
                            let value = self.evaluate(arg.clone())?;
                            match value {
                                Value::Number(n) => result -= n,
                                Value::Text(s) => {
                                    match s.parse::<i64>() {
                                        Ok(n) => result -= n,
                                        Err(_) => return Err(format!("Cannot convert '{}' to a number for subtraction", s)),
                                    }
                                }
                                _ => return Err("Expected number for subtraction".to_string()),
                            }
                        }
                        Ok(Value::Number(result))
                    }
                    "-mul" => {
                        if args.len() < 2 {
                            return Err(format!("Multiply command expects at least two arguments, got {}", args.len()));
                        }
                        
                        // Start with 1 as the identity element for multiplication
                        let mut result = 1;
                        for arg in args {
                            let value = self.evaluate(arg.clone())?;
                            match value {
                                Value::Number(n) => result *= n,
                                Value::Text(s) => {
                                    match s.parse::<i64>() {
                                        Ok(n) => result *= n,
                                        Err(_) => return Err(format!("Cannot convert '{}' to a number for multiplication", s)),
                                    }
                                }
                                _ => return Err("Expected number for multiplication".to_string()),
                            }
                        }
                        Ok(Value::Number(result))
                    }
                    "-div" => {
                        if args.len() < 2 {
                            return Err(format!("Divide command expects at least two arguments, got {}", args.len()));
                        }
                        
                        // Get the first value
                        let first_arg = self.evaluate(args[0].clone())?;
                        let mut result = match first_arg {
                            Value::Number(n) => n,
                            Value::Text(s) => {
                                match s.parse::<i64>() {
                                    Ok(n) => n,
                                    Err(_) => return Err(format!("Cannot convert '{}' to a number for division", s)),
                                }
                            }
                            _ => return Err("Expected number for division".to_string()),
                        };
                        
                        // Divide by all other values
                        for arg in args.iter().skip(1) {
                            let value = self.evaluate(arg.clone())?;
                            match value {
                                Value::Number(n) => {
                                    if n == 0 {
                                        return Err("Division by zero".to_string());
                                    }
                                    result /= n;
                                }
                                Value::Text(s) => {
                                    match s.parse::<i64>() {
                                        Ok(n) => {
                                            if n == 0 {
                                                return Err("Division by zero".to_string());
                                            }
                                            result /= n;
                                        }
                                        Err(_) => return Err(format!("Cannot convert '{}' to a number for division", s)),
                                    }
                                }
                                _ => return Err("Expected number for division".to_string()),
                            }
                        }
                        Ok(Value::Number(result))
                    }
                    "-mod" => {
                        if args.len() != 2 {
                            return Err(format!("Modulo command expects exactly two arguments, got {}", args.len()));
                        }
                        
                        // Get the left operand
                        let left_arg = self.evaluate(args[0].clone())?;
                        let left = match left_arg {
                            Value::Number(n) => n,
                            Value::Text(s) => {
                                match s.parse::<i64>() {
                                    Ok(n) => n,
                                    Err(_) => return Err(format!("Cannot convert '{}' to a number for modulo", s)),
                                }
                            }
                            _ => return Err("Expected number for modulo".to_string()),
                        };
                        
                        // Get the right operand
                        let right_arg = self.evaluate(args[1].clone())?;
                        let right = match right_arg {
                            Value::Number(n) => n,
                            Value::Text(s) => {
                                match s.parse::<i64>() {
                                    Ok(n) => n,
                                    Err(_) => return Err(format!("Cannot convert '{}' to a number for modulo", s)),
                                }
                            }
                            _ => return Err("Expected number for modulo".to_string()),
                        };
                        
                        if right == 0 {
                            return Err("Modulo by zero".to_string());
                        }
                        
                        Ok(Value::Number(left % right))
                    }
                    _ => Err(format!("Unknown command: {}", name)),
                }
            }
        }
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