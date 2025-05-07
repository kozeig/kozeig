use crate::lexer::{Lexer, TokenType};
use crate::parser::{Expr, Parser, Stmt};
use std::fs;
use std::path::Path;

// Define a Bytecode enum to represent our platform-independent instructions
#[derive(Debug, Clone)]
pub enum Instruction {
    LoadNumber(i64),
    LoadText(String),
    LoadBoolean(bool),
    LoadVariable(String),
    StoreVariable(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    PrintValue,
    PrintNewline,
    ToAscii,
    NoOp,
    // Comparison operators
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Logical operators
    And,
    Or,
    Not,
    // Grouping
    GroupBegin,
    GroupEnd,
}

// Simple binary encoding for the instruction
impl Instruction {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Instruction::LoadNumber(n) => {
                bytes.push(0); // Opcode 0 = LoadNumber
                bytes.extend_from_slice(&n.to_le_bytes());
            }
            Instruction::LoadText(s) => {
                bytes.push(1); // Opcode 1 = LoadText
                let text_bytes = s.as_bytes();
                let len = text_bytes.len() as u32;
                bytes.extend_from_slice(&len.to_le_bytes());
                bytes.extend_from_slice(text_bytes);
            }
            Instruction::LoadBoolean(b) => {
                bytes.push(2); // Opcode 2 = LoadBoolean
                bytes.push(if *b { 1 } else { 0 });
            }
            Instruction::LoadVariable(name) => {
                bytes.push(3); // Opcode 3 = LoadVariable
                let name_bytes = name.as_bytes();
                let len = name_bytes.len() as u32;
                bytes.extend_from_slice(&len.to_le_bytes());
                bytes.extend_from_slice(name_bytes);
            }
            Instruction::StoreVariable(name) => {
                bytes.push(4); // Opcode 4 = StoreVariable
                let name_bytes = name.as_bytes();
                let len = name_bytes.len() as u32;
                bytes.extend_from_slice(&len.to_le_bytes());
                bytes.extend_from_slice(name_bytes);
            }
            Instruction::Add => bytes.push(5), // Opcode 5 = Add
            Instruction::Subtract => bytes.push(6), // Opcode 6 = Subtract
            Instruction::Multiply => bytes.push(7), // Opcode 7 = Multiply
            Instruction::Divide => bytes.push(8), // Opcode 8 = Divide
            Instruction::Modulo => bytes.push(9), // Opcode 9 = Modulo
            Instruction::PrintValue => bytes.push(10), // Opcode 10 = PrintValue
            Instruction::PrintNewline => bytes.push(11), // Opcode 11 = PrintNewline
            Instruction::ToAscii => bytes.push(12), // Opcode 12 = ToAscii
            Instruction::NoOp => bytes.push(13), // Opcode 13 = NoOp

            // Comparison operators
            Instruction::Equal => bytes.push(14), // Opcode 14 = Equal
            Instruction::NotEqual => bytes.push(15), // Opcode 15 = NotEqual
            Instruction::Greater => bytes.push(16), // Opcode 16 = Greater
            Instruction::GreaterEqual => bytes.push(17), // Opcode 17 = GreaterEqual
            Instruction::Less => bytes.push(18),  // Opcode 18 = Less
            Instruction::LessEqual => bytes.push(19), // Opcode 19 = LessEqual

            // Logical operators
            Instruction::And => bytes.push(20), // Opcode 20 = And
            Instruction::Or => bytes.push(21),  // Opcode 21 = Or
            Instruction::Not => bytes.push(22), // Opcode 22 = Not

            // Grouping
            Instruction::GroupBegin => bytes.push(23), // Opcode 23 = GroupBegin
            Instruction::GroupEnd => bytes.push(24),   // Opcode 24 = GroupEnd
        }
        bytes
    }
}

// Bytecode compiler that outputs a JSON representation of instructions
pub struct BytecodeCompiler {
    instructions: Vec<Instruction>,
    variables: Vec<String>,
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        BytecodeCompiler {
            instructions: Vec::new(),
            variables: Vec::new(),
        }
    }

    pub fn compile(&mut self, statements: Vec<Stmt>) -> Result<Vec<Instruction>, String> {
        for stmt in statements {
            self.compile_statement(stmt)?;
        }

        // Return the resulting instructions
        Ok(self.instructions.clone())
    }

    fn compile_statement(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Declaration { name, initializer } => {
                if !self.variables.contains(&name) {
                    self.variables.push(name.clone());
                }

                self.compile_expression(initializer)?;
                self.instructions.push(Instruction::StoreVariable(name));
            }
            Stmt::Expression(expr) => {
                self.compile_expression(expr)?;
                // Pop the result since it's not used
                self.instructions.push(Instruction::NoOp);
            }
            Stmt::Command { name, args } => {
                match name.as_str() {
                    "-print" => {
                        for (i, arg) in args.iter().enumerate() {
                            self.compile_expression(arg.clone())?;
                            self.instructions.push(Instruction::PrintValue);

                            // Print a space between arguments (but not after the last one)
                            if i < args.len() - 1 {
                                self.instructions
                                    .push(Instruction::LoadText(" ".to_string()));
                                self.instructions.push(Instruction::PrintValue);
                            }
                        }
                        self.instructions.push(Instruction::PrintNewline);
                    }
                    // Other commands would be handled similarly
                    _ => return Err(format!("Unknown command in bytecode compiler: {}", name)),
                }
            }
            Stmt::Print(exprs) => {
                for (i, expr) in exprs.iter().enumerate() {
                    self.compile_expression(expr.clone())?;
                    self.instructions.push(Instruction::PrintValue);

                    // Print a space between arguments (but not after the last one)
                    if i < exprs.len() - 1 {
                        self.instructions
                            .push(Instruction::LoadText(" ".to_string()));
                        self.instructions.push(Instruction::PrintValue);
                    }
                }
                self.instructions.push(Instruction::PrintNewline);
            }
            Stmt::Comment(_) => {
                // Comments are ignored in the compiled output
            }
        }

        Ok(())
    }

    fn compile_expression(&mut self, expr: Expr) -> Result<(), String> {
        match expr {
            Expr::VariableRef(name) => {
                if name.starts_with('$') {
                    let var_name = name[1..].to_string();
                    self.instructions.push(Instruction::LoadVariable(var_name));
                } else {
                    return Err(format!("Invalid variable reference: {}", name));
                }
            }
            Expr::NumberLiteral(value) => {
                self.instructions.push(Instruction::LoadNumber(value));
            }
            Expr::TextLiteral(value) => {
                self.instructions.push(Instruction::LoadText(value));
            }
            Expr::BooleanLiteral(value) => {
                self.instructions.push(Instruction::LoadBoolean(value));
            }
            Expr::Grouping { expression } => {
                self.instructions.push(Instruction::GroupBegin);
                self.compile_expression(*expression)?;
                self.instructions.push(Instruction::GroupEnd);
            }
            Expr::Unary { operator, right } => {
                // For negation, we need to push 0 first, then the value, then subtract
                if operator.token_type == TokenType::Minus {
                    self.instructions.push(Instruction::LoadNumber(0));
                }

                self.compile_expression(*right)?;

                match operator.token_type {
                    TokenType::Minus => self.instructions.push(Instruction::Subtract),
                    TokenType::Not => self.instructions.push(Instruction::Not),
                    _ => {
                        return Err(format!(
                            "Unsupported unary operator: {:?}",
                            operator.token_type
                        ))
                    }
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                // Compile left and right operands
                self.compile_expression(*left)?;
                self.compile_expression(*right)?;

                // Emit the appropriate instruction for the operator
                match operator.token_type {
                    TokenType::Plus => self.instructions.push(Instruction::Add),
                    TokenType::Minus => self.instructions.push(Instruction::Subtract),
                    TokenType::Star => self.instructions.push(Instruction::Multiply),
                    TokenType::Slash => self.instructions.push(Instruction::Divide),
                    TokenType::Percent => self.instructions.push(Instruction::Modulo),

                    TokenType::Equal => self.instructions.push(Instruction::Equal),
                    TokenType::NotEqual => self.instructions.push(Instruction::NotEqual),
                    TokenType::Greater => self.instructions.push(Instruction::Greater),
                    TokenType::GreaterEqual => self.instructions.push(Instruction::GreaterEqual),
                    TokenType::Less => self.instructions.push(Instruction::Less),
                    TokenType::LessEqual => self.instructions.push(Instruction::LessEqual),

                    TokenType::And => self.instructions.push(Instruction::And),
                    TokenType::Or => self.instructions.push(Instruction::Or),

                    _ => {
                        return Err(format!(
                            "Unsupported binary operator: {:?}",
                            operator.token_type
                        ))
                    }
                }
            }
            Expr::Command { name, args } => {
                match name.as_str() {
                    "-number" => {
                        if args.len() != 1 {
                            return Err("Number command expects one argument".to_string());
                        }

                        // Just compile the expression (converts to number implicitly)
                        self.compile_expression(args[0].clone())?;
                    }
                    "-text" => {
                        if args.len() != 1 {
                            return Err("Text command expects one argument".to_string());
                        }

                        // Just compile the expression (converts to text implicitly)
                        self.compile_expression(args[0].clone())?;
                    }
                    "-bool" => {
                        if args.len() != 1 {
                            return Err("Boolean command expects one argument".to_string());
                        }

                        // Just compile the expression (it will be treated as boolean)
                        self.compile_expression(args[0].clone())?;
                    }
                    "-asc" => {
                        if args.len() != 1 {
                            return Err("Asc command expects one argument".to_string());
                        }

                        self.compile_expression(args[0].clone())?;
                        self.instructions.push(Instruction::ToAscii);
                    }
                    // Add more commands as needed
                    _ => return Err(format!("Unknown command: {}", name)),
                }
            }
        }

        Ok(())
    }
}

// Convert instructions to serializable format
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::LoadNumber(n) => write!(f, "{{\"op\":\"load_number\",\"value\":{}}}", n),
            Instruction::LoadText(s) => write!(
                f,
                "{{\"op\":\"load_text\",\"value\":\"{}\"}}",
                s.replace("\"", "\\\"")
            ),
            Instruction::LoadBoolean(b) => write!(f, "{{\"op\":\"load_boolean\",\"value\":{}}}", if *b { "true" } else { "false" }),
            Instruction::LoadVariable(name) => {
                write!(f, "{{\"op\":\"load_variable\",\"name\":\"{}\"}}", name)
            }
            Instruction::StoreVariable(name) => {
                write!(f, "{{\"op\":\"store_variable\",\"name\":\"{}\"}}", name)
            }
            Instruction::Add => write!(f, "{{\"op\":\"add\"}}"),
            Instruction::Subtract => write!(f, "{{\"op\":\"subtract\"}}"),
            Instruction::Multiply => write!(f, "{{\"op\":\"multiply\"}}"),
            Instruction::Divide => write!(f, "{{\"op\":\"divide\"}}"),
            Instruction::Modulo => write!(f, "{{\"op\":\"modulo\"}}"),
            Instruction::PrintValue => write!(f, "{{\"op\":\"print_value\"}}"),
            Instruction::PrintNewline => write!(f, "{{\"op\":\"print_newline\"}}"),
            Instruction::ToAscii => write!(f, "{{\"op\":\"to_ascii\"}}"),
            Instruction::NoOp => write!(f, "{{\"op\":\"noop\"}}"),

            // Comparison operators
            Instruction::Equal => write!(f, "{{\"op\":\"equal\"}}"),
            Instruction::NotEqual => write!(f, "{{\"op\":\"not_equal\"}}"),
            Instruction::Greater => write!(f, "{{\"op\":\"greater\"}}"),
            Instruction::GreaterEqual => write!(f, "{{\"op\":\"greater_equal\"}}"),
            Instruction::Less => write!(f, "{{\"op\":\"less\"}}"),
            Instruction::LessEqual => write!(f, "{{\"op\":\"less_equal\"}}"),

            // Logical operators
            Instruction::And => write!(f, "{{\"op\":\"and\"}}"),
            Instruction::Or => write!(f, "{{\"op\":\"or\"}}"),
            Instruction::Not => write!(f, "{{\"op\":\"not\"}}"),

            // Grouping
            Instruction::GroupBegin => write!(f, "{{\"op\":\"group_begin\"}}"),
            Instruction::GroupEnd => write!(f, "{{\"op\":\"group_end\"}}"),
        }
    }
}

pub fn compile(source: &str, file_path: &str) -> Result<(), String> {
    //use tempfile::NamedTempFile;
    use std::io::Write;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut bytecode_compiler = BytecodeCompiler::new();
    let instructions = bytecode_compiler.compile(statements)?;

    let mut bytecode_data = Vec::new();
    for instruction in &instructions {
        bytecode_data.extend(instruction.to_bytes());
    }

    // Create the output binary path
    // For file input like examples/hello.lut, just use basename 'hello'
    let source_path = Path::new(file_path);
    let file_stem = source_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    let file_stem_str = file_stem.to_string();
    let out_path = Path::new(&file_stem_str);

    // Create a temp C file with the embedded bc
    let temp_dir = tempfile::tempdir().map_err(|e| format!("Failed to create temp dir: {}", e))?;
    let c_path = temp_dir.path().join("bytecode.c");
    let mut c_file =
        fs::File::create(&c_path).map_err(|e| format!("Failed to create C file: {}", e))?;

    writeln!(c_file, "#include <stdio.h>").map_err(|e| format!("Write error: {}", e))?;
    writeln!(c_file, "#include <stdlib.h>").map_err(|e| format!("Write error: {}", e))?;
    writeln!(c_file, "#include <string.h>").map_err(|e| format!("Write error: {}", e))?;
    writeln!(c_file, "#include <stdint.h>").map_err(|e| format!("Write error: {}", e))?;
    writeln!(c_file).map_err(|e| format!("Write error: {}", e))?;

    writeln!(c_file, "const unsigned char BYTECODE[] = {{")
        .map_err(|e| format!("Write error: {}", e))?;

    for (i, byte) in bytecode_data.iter().enumerate() {
        if i > 0 && i % 12 == 0 {
            writeln!(c_file).map_err(|e| format!("Write error: {}", e))?;
        }
        write!(c_file, "0x{:02x}, ", byte).map_err(|e| format!("Write error: {}", e))?;
    }

    writeln!(c_file, "\n}};").map_err(|e| format!("Write error: {}", e))?;
    writeln!(
        c_file,
        "const size_t BYTECODE_SIZE = {};",
        bytecode_data.len()
    )
    .map_err(|e| format!("Write error: {}", e))?;

    // runtime implementation in C
    writeln!(c_file, r#"
// val types
typedef enum {{
    NUMBER,
    TEXT,
    BOOLEAN
}} ValueType;

// val struct
typedef struct {{
    ValueType type;
    union {{
        int64_t number;
        char* text;
        int boolean;  // Using int (0 or 1) to represent boolean values
    }} data;
}} Value;

// instructions
typedef enum {{
    LOAD_NUMBER,
    LOAD_TEXT,
    LOAD_BOOLEAN,
    LOAD_VARIABLE,
    STORE_VARIABLE,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    MODULO,
    PRINT_VALUE,
    PRINT_NEWLINE,
    TO_ASCII,
    NOOP,
    // Comparison operators
    EQUAL,
    NOT_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Logical operators
    AND,
    OR,
    NOT,
    // Grouping
    GROUP_BEGIN,
    GROUP_END
}} InstructionType;

// var storage
typedef struct {{
    char* name;
    Value value;
}} Variable;

// env
typedef struct {{
    Variable* variables;
    size_t variable_count;
    size_t variable_capacity;

    Value* stack;
    size_t stack_size;
    size_t stack_capacity;
}} Environment;

// create and init env
Environment* create_environment() {{
    Environment* env = (Environment*)malloc(sizeof(Environment));
    env->variables = NULL;
    env->variable_count = 0;
    env->variable_capacity = 0;

    env->stack = NULL;
    env->stack_size = 0;
    env->stack_capacity = 0;

    return env;
}}

// push val to stack
void push(Environment* env, Value value) {{
    if (env->stack_size >= env->stack_capacity) {{
        env->stack_capacity = env->stack_capacity == 0 ? 8 : env->stack_capacity * 2;
        env->stack = (Value*)realloc(env->stack, env->stack_capacity * sizeof(Value));
    }}

    env->stack[env->stack_size++] = value;
}}

// pop val from stack
Value pop(Environment* env) {{
    if (env->stack_size == 0) {{
        fprintf(stderr, "Runtime error: Stack underflow\n");
        exit(1);
    }}

    return env->stack[--env->stack_size];
}}

// store var
void store_variable(Environment* env, const char* name, Value value) {{
    // Look for existing var
    for (size_t i = 0; i < env->variable_count; i++) {{
        if (strcmp(env->variables[i].name, name) == 0) {{
            // Free old text if needed
            if (env->variables[i].value.type == TEXT && env->variables[i].value.data.text != NULL) {{
                free(env->variables[i].value.data.text);
            }}

            env->variables[i].value = value;
            return;
        }}
    }}

    // Add new var
    if (env->variable_count >= env->variable_capacity) {{
        env->variable_capacity = env->variable_capacity == 0 ? 8 : env->variable_capacity * 2;
        env->variables = (Variable*)realloc(env->variables, env->variable_capacity * sizeof(Variable));
    }}

    env->variables[env->variable_count].name = strdup(name);
    env->variables[env->variable_count].value = value;
    env->variable_count++;
}}

// load var
Value load_variable(Environment* env, const char* name) {{
    for (size_t i = 0; i < env->variable_count; i++) {{
        if (strcmp(env->variables[i].name, name) == 0) {{
            return env->variables[i].value;
        }}
    }}

    fprintf(stderr, "Runtime error: Undefined variable '%s'\n", name);
    exit(1);
}}

// create text val
Value create_text_value(const char* text) {{
    Value value;
    value.type = TEXT;
    value.data.text = strdup(text);
    return value;
}}

// create number val
Value create_number_value(int64_t number) {{
    Value value;
    value.type = NUMBER;
    value.data.number = number;
    return value;
}}

// create boolean val
Value create_boolean_value(int boolean) {{
    Value value;
    value.type = BOOLEAN;
    value.data.boolean = boolean ? 1 : 0;
    return value;
}}

// Properly cleanup memory resources
void cleanup_environment(Environment* env) {{
    if (env == NULL) return;

    // Free variables
    if (env->variables != NULL) {{
        for (size_t i = 0; i < env->variable_count; i++) {{
            if (env->variables[i].name != NULL) {{
                free(env->variables[i].name);
            }}
            
            if (env->variables[i].value.type == TEXT && env->variables[i].value.data.text != NULL) {{
                free(env->variables[i].value.data.text);
            }}
        }}
        free(env->variables);
    }}

    // Free stack values
    if (env->stack != NULL) {{
        for (size_t i = 0; i < env->stack_size; i++) {{
            if (env->stack[i].type == TEXT && env->stack[i].data.text != NULL) {{
                free(env->stack[i].data.text);
            }}
        }}
        free(env->stack);
    }}

    // Free the environment itself
    free(env);
}}

// run
void run_bytecode(unsigned char* bytecode, size_t size) {{
    Environment* env = create_environment();

    // Simple bytecode interpreter
    size_t ip = 0;  // Instruction pointer

    while (ip < size) {{
        InstructionType instr_type = bytecode[ip++];

        switch (instr_type) {{
            case LOAD_NUMBER: {{
                // Read 8-byte number
                int64_t value = 0;
                memcpy(&value, &bytecode[ip], sizeof(int64_t));
                ip += sizeof(int64_t);

                push(env, create_number_value(value));
                break;
            }}

            case LOAD_TEXT: {{
                // Read string length
                uint32_t len = 0;
                memcpy(&len, &bytecode[ip], sizeof(uint32_t));
                ip += sizeof(uint32_t);

                // Read string
                char* text = (char*)malloc(len + 1);
                memcpy(text, &bytecode[ip], len);
                text[len] = '\0';
                ip += len;

                Value value;
                value.type = TEXT;
                value.data.text = text;
                push(env, value);
                break;
            }}

            case LOAD_BOOLEAN: {{
                // Read boolean value (0 or 1)
                int boolean_val = bytecode[ip++];
                push(env, create_boolean_value(boolean_val));
                break;
            }}

            case LOAD_VARIABLE: {{
                // Read variable name length
                uint32_t len = 0;
                memcpy(&len, &bytecode[ip], sizeof(uint32_t));
                ip += sizeof(uint32_t);

                // Read variable name
                char* name = (char*)malloc(len + 1);
                memcpy(name, &bytecode[ip], len);
                name[len] = '\0';
                ip += len;

                Value value = load_variable(env, name);
                push(env, value);

                free(name);
                break;
            }}

            case STORE_VARIABLE: {{
                // Read variable name length
                uint32_t len = 0;
                memcpy(&len, &bytecode[ip], sizeof(uint32_t));
                ip += sizeof(uint32_t);

                // Read variable name
                char* name = (char*)malloc(len + 1);
                memcpy(name, &bytecode[ip], len);
                name[len] = '\0';
                ip += len;

                Value value = pop(env);
                store_variable(env, name, value);

                free(name);
                break;
            }}

            case ADD: {{
                Value right = pop(env);
                Value left = pop(env);

                int64_t left_val = left.type == NUMBER ? left.data.number :
                                  (left.type == TEXT ? atoll(left.data.text) : 0);
                int64_t right_val = right.type == NUMBER ? right.data.number :
                                   (right.type == TEXT ? atoll(right.data.text) : 0);

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(left_val + right_val));
                break;
            }}

            case SUBTRACT: {{
                Value right = pop(env);
                Value left = pop(env);

                // Get left value
                int64_t left_val;
                if (left.type == NUMBER) {{
                    left_val = left.data.number;
                }} else if (left.type == TEXT && left.data.text) {{
                    // Safely convert text to number
                    char* endptr = NULL;
                    left_val = strtoll(left.data.text, &endptr, 10);
                    // Check if conversion was successful
                    if (endptr == left.data.text) {{
                        fprintf(stderr, "Runtime error: Cannot convert '%s' to a number for subtraction\n", left.data.text);
                        exit(1);
                    }}
                }} else {{
                    left_val = 0;
                }}

                // Get right value
                int64_t right_val;
                if (right.type == NUMBER) {{
                    right_val = right.data.number;
                }} else if (right.type == TEXT && right.data.text) {{
                    // Safely convert text to number
                    char* endptr = NULL;
                    right_val = strtoll(right.data.text, &endptr, 10);
                    // Check if conversion was successful
                    if (endptr == right.data.text) {{
                        fprintf(stderr, "Runtime error: Cannot convert '%s' to a number for subtraction\n", right.data.text);
                        exit(1);
                    }}
                }} else {{
                    right_val = 0;
                }}

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(left_val - right_val));
                break;
            }}

            case MULTIPLY: {{
                Value right = pop(env);
                Value left = pop(env);

                int64_t left_val = left.type == NUMBER ? left.data.number :
                                  (left.type == TEXT ? atoll(left.data.text) : 0);
                int64_t right_val = right.type == NUMBER ? right.data.number :
                                   (right.type == TEXT ? atoll(right.data.text) : 0);

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(left_val * right_val));
                break;
            }}

            case DIVIDE: {{
                Value right = pop(env);
                Value left = pop(env);

                int64_t left_val = left.type == NUMBER ? left.data.number :
                                  (left.type == TEXT ? atoll(left.data.text) : 0);
                int64_t right_val = right.type == NUMBER ? right.data.number :
                                   (right.type == TEXT ? atoll(right.data.text) : 0);

                if (right_val == 0) {{
                    fprintf(stderr, "Runtime error: Division by zero\n");
                    exit(1);
                }}

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(left_val / right_val));
                break;
            }}

            case MODULO: {{
                Value right = pop(env);
                Value left = pop(env);

                int64_t left_val = left.type == NUMBER ? left.data.number :
                                  (left.type == TEXT ? atoll(left.data.text) : 0);
                int64_t right_val = right.type == NUMBER ? right.data.number :
                                   (right.type == TEXT ? atoll(right.data.text) : 0);

                if (right_val == 0) {{
                    fprintf(stderr, "Runtime error: Modulo by zero\n");
                    exit(1);
                }}

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(left_val % right_val));
                break;
            }}

            case PRINT_VALUE: {{
                Value value = pop(env);

                if (value.type == NUMBER) {{
                    printf("%lld", value.data.number);
                }} else if (value.type == TEXT) {{
                    printf("%s", value.data.text);
                    free(value.data.text);
                }} else if (value.type == BOOLEAN) {{
                    printf("%s", value.data.boolean ? "true" : "false");
                }}

                break;
            }}

            case PRINT_NEWLINE: {{
                printf("\n");
                break;
            }}

            case TO_ASCII: {{
                Value value = pop(env);
                int64_t code = value.type == NUMBER ? value.data.number :
                              (value.type == TEXT ? atoll(value.data.text) : 0);

                if (value.type == TEXT && value.data.text) {{
                    free(value.data.text);
                }}

                char ascii_char = (char)code;
                char* text = (char*)malloc(2);
                text[0] = ascii_char;
                text[1] = '\0';

                Value text_value;
                text_value.type = TEXT;
                text_value.data.text = text;
                push(env, text_value);
                break;
            }}

            case NOOP:
                // Do nothing
                break;

            // Comparison operators
            case EQUAL: {{
                Value right = pop(env);
                Value left = pop(env);
                int64_t result = 0;

                if (left.type == NUMBER && right.type == NUMBER) {{
                    result = (left.data.number == right.data.number) ? 1 : 0;
                }} else if (left.type == TEXT && right.type == TEXT) {{
                    result = (strcmp(left.data.text, right.data.text) == 0) ? 1 : 0;
                }} else {{
                    result = 0; // Different types are never equal
                }}

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(result));
                break;
            }}

            case NOT_EQUAL: {{
                Value right = pop(env);
                Value left = pop(env);
                int64_t result = 0;

                if (left.type == NUMBER && right.type == NUMBER) {{
                    result = (left.data.number != right.data.number) ? 1 : 0;
                }} else if (left.type == TEXT && right.type == TEXT) {{
                    result = (strcmp(left.data.text, right.data.text) != 0) ? 1 : 0;
                }} else {{
                    result = 1; // Different types are always not equal
                }}

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(result));
                break;
            }}

            case GREATER: {{
                Value right = pop(env);
                Value left = pop(env);
                int64_t result = 0;

                if (left.type == NUMBER && right.type == NUMBER) {{
                    result = (left.data.number > right.data.number) ? 1 : 0;
                }} else if (left.type == TEXT && right.type == TEXT) {{
                    result = (strcmp(left.data.text, right.data.text) > 0) ? 1 : 0;
                }} else {{
                    fprintf(stderr, "Runtime error: Cannot compare different types\n");
                    exit(1);
                }}

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(result));
                break;
            }}

            case GREATER_EQUAL: {{
                Value right = pop(env);
                Value left = pop(env);
                int64_t result = 0;

                if (left.type == NUMBER && right.type == NUMBER) {{
                    result = (left.data.number >= right.data.number) ? 1 : 0;
                }} else if (left.type == TEXT && right.type == TEXT) {{
                    result = (strcmp(left.data.text, right.data.text) >= 0) ? 1 : 0;
                }} else {{
                    fprintf(stderr, "Runtime error: Cannot compare different types\n");
                    exit(1);
                }}

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(result));
                break;
            }}

            case LESS: {{
                Value right = pop(env);
                Value left = pop(env);
                int64_t result = 0;

                if (left.type == NUMBER && right.type == NUMBER) {{
                    result = (left.data.number < right.data.number) ? 1 : 0;
                }} else if (left.type == TEXT && right.type == TEXT) {{
                    result = (strcmp(left.data.text, right.data.text) < 0) ? 1 : 0;
                }} else {{
                    fprintf(stderr, "Runtime error: Cannot compare different types\n");
                    exit(1);
                }}

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(result));
                break;
            }}

            case LESS_EQUAL: {{
                Value right = pop(env);
                Value left = pop(env);
                int64_t result = 0;

                if (left.type == NUMBER && right.type == NUMBER) {{
                    result = (left.data.number <= right.data.number) ? 1 : 0;
                }} else if (left.type == TEXT && right.type == TEXT) {{
                    result = (strcmp(left.data.text, right.data.text) <= 0) ? 1 : 0;
                }} else {{
                    fprintf(stderr, "Runtime error: Cannot compare different types\n");
                    exit(1);
                }}

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(result));
                break;
            }}

            // Logical operators
            case AND: {{
                Value right = pop(env);
                Value left = pop(env);

                int64_t left_bool = (left.type == NUMBER && left.data.number != 0) ||
                                   (left.type == TEXT && left.data.text && strlen(left.data.text) > 0);

                int64_t right_bool = (right.type == NUMBER && right.data.number != 0) ||
                                    (right.type == TEXT && right.data.text && strlen(right.data.text) > 0);

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(left_bool && right_bool ? 1 : 0));
                break;
            }}

            case OR: {{
                Value right = pop(env);
                Value left = pop(env);

                int64_t left_bool = (left.type == NUMBER && left.data.number != 0) ||
                                   (left.type == TEXT && left.data.text && strlen(left.data.text) > 0);

                int64_t right_bool = (right.type == NUMBER && right.data.number != 0) ||
                                    (right.type == TEXT && right.data.text && strlen(right.data.text) > 0);

                // Free text values if needed
                if (left.type == TEXT && left.data.text) free(left.data.text);
                if (right.type == TEXT && right.data.text) free(right.data.text);

                push(env, create_number_value(left_bool || right_bool ? 1 : 0));
                break;
            }}

            case NOT: {{
                Value value = pop(env);

                int64_t bool_val = (value.type == NUMBER && value.data.number != 0) ||
                                  (value.type == TEXT && value.data.text && strlen(value.data.text) > 0);

                // Free text values if needed
                if (value.type == TEXT && value.data.text) free(value.data.text);

                push(env, create_number_value(bool_val ? 0 : 1));
                break;
            }}

            // Grouping (no effect on runtime, used for parsing)
            case GROUP_BEGIN:
            case GROUP_END:
                break;

            default:
                fprintf(stderr, "Runtime error: Unknown instruction %d\n", instr_type);
                exit(1);
        }}
    }}

    cleanup_environment(env);
}}

int main() {{
    run_bytecode((unsigned char*)BYTECODE, BYTECODE_SIZE);
    return 0;
}}
"#).map_err(|e| format!("Write error: {}", e))?;

    c_file.flush().map_err(|e| format!("Flush error: {}", e))?;

    // Compile the C file
    println!("Compiling to standalone binary: {}", out_path.display());

    // use system compiler directly
    let output_path = out_path.to_str().unwrap_or("./output");

    // command for binary
    #[cfg(unix)]
    let status = std::process::Command::new("cc")
        .arg("-o")
        .arg(output_path)
        .arg(&c_path)
        .arg("-O2")
        .status()
        .map_err(|e| format!("Failed to execute compiler: {}", e))?;

    #[cfg(windows)] // untested just assume this works i guess
    let status = std::process::Command::new("cl")
        .arg("/Fe:")
        .arg(output_path)
        .arg(&c_path)
        .arg("/O2")
        .status()
        .map_err(|e| format!("Failed to execute compiler: {}", e))?;

    if !status.success() {
        return Err("C compiler failed".to_string());
    }

    // make it go vroom vroom
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata =
            fs::metadata(out_path).map_err(|e| format!("Failed to get metadata: {}", e))?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(out_path, perms)
            .map_err(|e| format!("Failed to set permissions: {}", e))?;
    }

    println!("Created executable: {}", out_path.display());
    println!("Run with: ./{}", out_path.display());

    Ok(())
}
