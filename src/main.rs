use std::env;
use std::fs;
use std::process;

mod compiler;
mod dependency_manager;
mod error_reporting;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: koze [build|run|jit|debug] <file> [-s|--silent]");
        eprintln!("  -s, --silent   Run in silent mode with minimal output");
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "debug" => {
            if args.len() < 3 {
                eprintln!("Usage: koze debug <file> [-s|--silent]");
                process::exit(1);
            }
            let file_path = &args[2];

            // Check for silent mode flag
            let silent_mode = args.iter().any(|arg| arg == "-s" || arg == "--silent");

            match fs::read_to_string(file_path) {
                Ok(source) => {
                    // Parse the file and print the AST for debugging
                    let mut lexer = lexer::Lexer::new(&source);
                    match lexer.scan_tokens() {
                        Ok(tokens) => {
                            if !silent_mode {
                                println!("Tokens:");
                                for token in &tokens {
                                    println!("{:?}: {:?}", token.token_type, token.lexeme);
                                }
                            }

                            let mut parser = parser::Parser::new(tokens);
                            match parser.parse() {
                                Ok(statements) => {
                                    if !silent_mode {
                                        println!("\nParsed AST:");
                                        for stmt in &statements {
                                            println!("{:#?}", stmt);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Parse error: {}", e);
                                    process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Lexer error: {}", e);
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                    process::exit(1);
                }
            }
        }
        "build" => {
            if args.len() < 3 {
                eprintln!("Usage: koze build <file> [-s|--silent]");
                process::exit(1);
            }
            let file_path = &args[2];

            // Check for silent mode flag
            let silent_mode = args.iter().any(|arg| arg == "-s" || arg == "--silent");

            match fs::read_to_string(file_path) {
                Ok(source) => {
                    match compiler::compile(&source, file_path, silent_mode) {
                        Ok(_) => {
                            if !silent_mode {
                                println!("Successfully compiled {}", file_path);
                            }
                        }
                        Err(e) => {
                            // Print enhanced error with source context
                            error_reporting::print_error_with_context(&e, &source);
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                    process::exit(1);
                }
            }
        }
        "run" => {
            if args.len() < 3 {
                eprintln!("Usage: koze run <file> [-s|--silent]");
                process::exit(1);
            }
            let file_path = &args[2];

            // Check for silent mode flag
            let silent_mode = args.iter().any(|arg| arg == "-s" || arg == "--silent");

            match fs::read_to_string(file_path) {
                Ok(source) => {
                    let path = std::path::Path::new(file_path);
                    if silent_mode {
                        match interpreter::run_silent(&source, Some(path)) {
                            Ok(_) => (),
                            Err(e) => {
                                // Print enhanced error with source context
                                error_reporting::print_error_with_context(&e, &source);
                                process::exit(1);
                            }
                        }
                    } else {
                        match interpreter::run(&source, Some(path)) {
                            Ok(_) => (),
                            Err(e) => {
                                // Print enhanced error with source context
                                error_reporting::print_error_with_context(&e, &source);
                                process::exit(1);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                    process::exit(1);
                }
            }
        }
        "jit" => {
            if args.len() < 3 {
                eprintln!("Usage: koze jit <file> [-s|--silent]");
                process::exit(1);
            }
            let file_path = &args[2];

            // Check for silent mode flag
            let silent_mode = args.iter().any(|arg| arg == "-s" || arg == "--silent");

            match fs::read_to_string(file_path) {
                Ok(source) => {
                    // Use the LLVM JIT compiler to compile and execute
                    match compiler::jit_compile_and_run(&source, file_path, silent_mode) {
                        Ok(_) => (), // The JIT execution already happened
                        Err(e) => {
                            // Print enhanced error with source context
                            error_reporting::print_error_with_context(&e, &source);
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage: koze [build|run|jit|debug] <file> [-s|--silent]");
            eprintln!("  -s, --silent   Run in silent mode with minimal output");
            process::exit(1);
        }
    }
}
