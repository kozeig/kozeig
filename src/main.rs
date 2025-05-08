use std::fs;
use std::env;
use std::process;

mod lexer;
mod parser;
mod interpreter;
mod compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: lut [build|run|jit] <file>");
        process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "build" => {
            if args.len() < 3 {
                eprintln!("Usage: lut build <file>");
                process::exit(1);
            }
            let file_path = &args[2];
            
            match fs::read_to_string(file_path) {
                Ok(source) => {
                    match compiler::compile(&source, file_path) {
                        Ok(_) => println!("Successfully compiled {}", file_path),
                        Err(e) => {
                            eprintln!("Compilation error: {}", e);
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
                eprintln!("Usage: lut run <file>");
                process::exit(1);
            }
            let file_path = &args[2];
            
            match fs::read_to_string(file_path) {
                Ok(source) => {
                    match interpreter::run(&source) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("Runtime error: {}", e);
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
        "jit" => {
            if args.len() < 3 {
                eprintln!("Usage: lut jit <file>");
                process::exit(1);
            }
            let file_path = &args[2];
            
            match fs::read_to_string(file_path) {
                Ok(source) => {
                    // Use the LLVM JIT compiler to compile and execute
                    match compiler::jit_compile_and_run(&source, file_path) {
                        Ok(_) => (), // The JIT execution already happened
                        Err(e) => {
                            eprintln!("JIT compilation error: {}", e);
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
            eprintln!("Usage: lut [build|run|jit] <file>");
            process::exit(1);
        }
    }
}