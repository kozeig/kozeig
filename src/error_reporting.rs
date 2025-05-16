use crate::lexer::Token;
use std::error::Error;
use std::fmt;

/// Represents the location of code in a source file
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: Option<usize>,
    pub file: Option<String>,
}

impl SourceLocation {
    pub fn new(line: usize) -> Self {
        SourceLocation {
            line,
            column: None,
            file: None,
        }
    }

    pub fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub fn with_file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }

    pub fn from_token(token: &Token) -> Self {
        SourceLocation {
            line: token.line,
            column: None, // Token doesn't store column info currently
            file: None,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(file) = &self.file {
            write!(f, "{}:", file)?;
        }

        write!(f, "line {}", self.line)?;

        if let Some(column) = self.column {
            write!(f, ", column {}", column)?;
        }

        Ok(())
    }
}

/// Enhanced error type for Kozeig language
#[derive(Debug)]
pub enum LutError {
    /// Lexer errors occur during tokenization
    Lexer {
        message: String,
        location: SourceLocation,
    },

    /// Parser errors occur during parsing the token stream
    Parser {
        message: String,
        location: SourceLocation,
    },

    /// Runtime errors occur during program execution
    Runtime {
        message: String,
        location: Option<SourceLocation>,
    },

    /// Compiler errors occur during compilation
    Compiler {
        message: String,
        location: Option<SourceLocation>,
    },

    /// I/O errors occur when reading files or other I/O operations
    IO { message: String },
}

impl LutError {
    pub fn lexer_error(message: impl Into<String>, line: usize) -> Self {
        LutError::Lexer {
            message: message.into(),
            location: SourceLocation::new(line),
        }
    }

    pub fn parser_error(message: impl Into<String>, token: &Token) -> Self {
        LutError::Parser {
            message: message.into(),
            location: SourceLocation::from_token(token),
        }
    }

    pub fn runtime_error(message: impl Into<String>, line: Option<usize>) -> Self {
        LutError::Runtime {
            message: message.into(),
            location: line.map(SourceLocation::new),
        }
    }

    pub fn compiler_error(message: impl Into<String>, line: Option<usize>) -> Self {
        LutError::Compiler {
            message: message.into(),
            location: line.map(SourceLocation::new),
        }
    }

    pub fn io_error(message: impl Into<String>) -> Self {
        LutError::IO {
            message: message.into(),
        }
    }
}

impl fmt::Display for LutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LutError::Lexer { message, location } => {
                write!(f, "Lexer error at {}: {}", location, message)
            }
            LutError::Parser { message, location } => {
                write!(f, "Parser error at {}: {}", location, message)
            }
            LutError::Runtime { message, location } => {
                if let Some(loc) = location {
                    write!(f, "Runtime error at {}: {}", loc, message)
                } else {
                    write!(f, "Runtime error: {}", message)
                }
            }
            LutError::Compiler { message, location } => {
                if let Some(loc) = location {
                    write!(f, "Compilation error at {}: {}", loc, message)
                } else {
                    write!(f, "Compilation error: {}", message)
                }
            }
            LutError::IO { message } => {
                write!(f, "I/O error: {}", message)
            }
        }
    }
}

impl Error for LutError {}

/// Convert from raw string errors to LutError
impl From<String> for LutError {
    fn from(error: String) -> Self {
        // Check if it's a parser error with line information
        if let Some(line_info) = extract_parser_error(&error) {
            let (line, message) = line_info;

            // Check for specific patterns to provide better error messages
            if message.contains("Expected '{'") && message.contains("after command") {
                let command = message.split('\'').nth(3).unwrap_or("").trim();
                return LutError::Parser {
                    message: format!("Expected '{{' after command '{}'\n\nTip: Commands need to be followed by arguments in braces, e.g. {}{{arg1, arg2}}",
                            command, command),
                    location: SourceLocation::new(line),
                };
            } else if message.contains("Expected expression") {
                return LutError::Parser {
                    message: format!("{}\n\nTip: Make sure you've provided a valid expression. Common expressions are: \n - Variable references (e.g. $var_name)\n - Literals (e.g. 42, 'text', true)\n - Commands (e.g. add{{1, 2}})",
                            message),
                    location: SourceLocation::new(line),
                };
            }

            // Check for specific parser error patterns
            if message.contains("Expect '}'") {
                return LutError::Parser {
                    message: format!("{}\n\nTip: Missing closing brace. Every opening '{{' needs a matching closing '}}'.", message),
                    location: SourceLocation::new(line),
                };
            } else if message.contains("Expected right brace") {
                return LutError::Parser {
                    message: format!("{}\n\nTip: Missing closing brace. Every opening '{{' needs a matching closing '}}'.", message),
                    location: SourceLocation::new(line),
                };
            } else if message.contains("Expect ']'") {
                return LutError::Parser {
                    message: format!("{}\n\nTip: Missing closing bracket. Every opening '[' needs a matching closing ']'.", message),
                    location: SourceLocation::new(line),
                };
            } else if message.contains("End of file") {
                return LutError::Parser {
                    message: format!("{}\n\nTip: Unexpected end of file. Check for missing closing braces or brackets.", message),
                    location: SourceLocation::new(line),
                };
            } else if message.contains("Expected expression") {
                return LutError::Parser {
                    message: format!("{}\n\nTip: An expression is required here. Valid expressions include variables (e.g. $var), literals (e.g. 42, 'text', true), or commands (e.g. array{{1, 2}}).", message),
                    location: SourceLocation::new(line),
                };
            }

            // General parser error
            return LutError::Parser {
                message: format!("{}\n\nTip: Check the syntax at this location.", message),
                location: SourceLocation::new(line),
            };
        }

        // Try to extract line information from the error message if it's there
        if error.contains("line") {
            // Simple pattern matching to extract line numbers from error strings
            if let Some(idx) = error.find("line") {
                let after_line = &error[idx + 4..];
                if let Some(end_idx) =
                    after_line.find(|c: char| !c.is_digit(10) && c != ' ' && c != ':')
                {
                    if let Ok(line_num) = after_line[..end_idx].trim().parse::<usize>() {
                        // Extract the actual error message
                        let message_start = idx + 4 + end_idx;
                        let message = if message_start < error.len() {
                            error[message_start..].trim().to_string()
                        } else {
                            "Unknown error".to_string()
                        };

                        return LutError::Runtime {
                            message,
                            location: Some(SourceLocation::new(line_num)),
                        };
                    }
                }
            }
        }

        // Check for specific error patterns to enhance the messages
        if error.contains("Undefined variable") {
            let var_name = error.split_whitespace().last().unwrap_or("");
            return LutError::Runtime {
                message: format!("Undefined variable: {}\n\nTip: Variables must be declared before use with the format:\n  variable_name : {{ value }}", var_name),
                location: None,
            };
        } else if error.contains("Invalid variable reference") {
            let var_name = error.split_whitespace().last().unwrap_or("");
            return LutError::Runtime {
                message: format!("Invalid variable reference: {}\n\nTip: Variables must be prefixed with $ when used. Did you mean ${}?", var_name, var_name),
                location: None,
            };
        }

        // If we can't extract line info or match patterns, just use the whole message
        LutError::Runtime {
            message: error,
            location: None,
        }
    }
}

// Helper function to extract line information from parser errors
fn extract_parser_error(error_msg: &str) -> Option<(usize, String)> {
    // Common pattern in parser errors: "Got X at line Y"
    if let Some(line_idx) = error_msg.rfind("at line ") {
        let line_str = &error_msg[line_idx + 8..];
        if let Ok(line_num) = line_str.trim().parse::<usize>() {
            return Some((line_num, error_msg.to_string()));
        }
    }
    None
}

/// Convert from LutError to String for backward compatibility
impl From<LutError> for String {
    fn from(error: LutError) -> Self {
        error.to_string()
    }
}

/// Helper function to extract line information from an error message
pub fn extract_line_info(error_msg: &str) -> Option<(usize, String)> {
    // Check common patterns for line numbers
    let line_patterns = [
        ("line ", ""),     // "line 42"
        ("at line ", ""),  // "at line 42"
        ("Line ", ""),     // "Line 42"
        ("At line ", ""),  // "At line 42"
        (" line ", ""),    // "... line 42"
        (" at line ", ""), // "... at line 42"
        ("line:", ""),     // "line: 42"
        ("line=", ""),     // "line=42"
        ("line(", ")"),    // "line(42)"
    ];

    // Try each pattern
    for (prefix, suffix) in line_patterns {
        if let Some(idx) = error_msg.find(prefix) {
            let after_prefix = &error_msg[idx + prefix.len()..];

            // If we have a suffix, find it
            let (number_part, rest) = if !suffix.is_empty() {
                if let Some(suffix_idx) = after_prefix.find(suffix) {
                    (
                        &after_prefix[..suffix_idx],
                        &after_prefix[suffix_idx + suffix.len()..],
                    )
                } else {
                    continue; // Suffix not found, try next pattern
                }
            } else {
                // No suffix, just find the end of the number
                if let Some(end_idx) = after_prefix.find(|c: char| !c.is_digit(10) && c != ' ') {
                    (&after_prefix[..end_idx], &after_prefix[end_idx..])
                } else {
                    // If no delimiter after number, use the whole remaining string
                    (after_prefix, "")
                }
            };

            // Parse the line number
            if let Ok(line_num) = number_part.trim().parse::<usize>() {
                // Get the error message after the line number
                let message = if rest.is_empty() {
                    // If there's no message after the line number, use the part before
                    if idx > 0 {
                        error_msg[..idx].trim().to_string()
                    } else {
                        "Unknown error".to_string()
                    }
                } else {
                    // Use the part after the line number
                    rest.trim_start_matches(|c: char| c == ':' || c == ' ')
                        .to_string()
                };

                return Some((line_num, message));
            }
        }
    }

    // Try regex-like patterns for more complex formats
    // Like "Error: something at position (x,y)" or "Error in file.txt:42: message"
    if let Some(colon_pos) = error_msg.rfind(':') {
        if colon_pos < error_msg.len() - 1 {
            let after_colon = &error_msg[colon_pos + 1..];
            if let Ok(line_num) = after_colon.trim().parse::<usize>() {
                // Get everything before the colon as the message
                let message = error_msg[..colon_pos].trim().to_string();
                return Some((line_num, message));
            }
        }
    }

    None
}

/// Function to pretty print error messages with source context
pub fn print_error_with_context(error: &LutError, source_code: &str) {
    let location = match error {
        LutError::Lexer { location, .. } => Some(location),
        LutError::Parser { location, .. } => Some(location),
        LutError::Runtime { location, .. } => location.as_ref(),
        LutError::Compiler { location, .. } => location.as_ref(),
        LutError::IO { .. } => None,
    };

    // Get error type and message for formatting
    let (error_type, error_message) = match error {
        LutError::Lexer { message, .. } => ("Lexer error", message),
        LutError::Parser { message, .. } => ("Parser error", message),
        LutError::Runtime { message, .. } => ("Runtime error", message),
        LutError::Compiler { message, .. } => ("Compilation error", message),
        LutError::IO { message } => ("I/O error", message),
    };

    // Print colored error header
    eprintln!("\x1b[1;31m{}\x1b[0m", "=".repeat(50));

    // Format error message with type and location
    if let Some(loc) = location {
        eprintln!(
            "\x1b[1;31m{}\x1b[0m at \x1b[1;33m{}\x1b[0m:",
            error_type, loc
        );
    } else {
        eprintln!("\x1b[1;31m{}\x1b[0m:", error_type);
    }

    // Print the actual error message with enhanced formatting
    let message_lines: Vec<&str> = error_message.lines().collect();

    // First line is the main error
    if !message_lines.is_empty() {
        eprintln!("\x1b[1m{}\x1b[0m", message_lines[0]);

        // Additional lines might be suggestions or tips
        for line in message_lines.iter().skip(1) {
            if line.starts_with("Tip:") {
                eprintln!("\x1b[1;36m{}\x1b[0m", line); // Cyan for tips
            } else if line.contains("Did you mean") {
                eprintln!("\x1b[1;32m{}\x1b[0m", line); // Green for suggestions
            } else if line.starts_with("  - ") {
                eprintln!("\x1b[1;32m{}\x1b[0m", line); // Green for suggestion items
            } else {
                eprintln!("{}", line);
            }
        }
    }

    // If we have location info, print source context
    if let Some(loc) = location {
        let lines: Vec<&str> = source_code.lines().collect();
        let line_idx = loc.line.saturating_sub(1); // Convert to 0-based index

        if line_idx < lines.len() {
            // Print a few lines of context
            let start_line = line_idx.saturating_sub(2);
            let end_line = std::cmp::min(line_idx + 3, lines.len());

            eprintln!("\n\x1b[1mSource context:\x1b[0m");
            for i in start_line..end_line {
                if i == line_idx {
                    // Highlight the error line
                    eprintln!(
                        "\x1b[33m{:>4}\x1b[0m \x1b[31m>\x1b[0m \x1b[1m{}\x1b[0m",
                        i + 1,
                        lines[i]
                    );

                    // Print an error pointer if we have column info
                    if let Some(col) = loc.column {
                        let spaces = " ".repeat(6 + col);
                        eprintln!(
                            "{}\x1b[1;31m^\x1b[0m\x1b[31m-- Error occurs here\x1b[0m",
                            spaces
                        );
                    } else {
                        // Try to find the specific position in the line for better pointer placement
                        if let Some(error_term) = message_lines.first().and_then(|msg| {
                            // Extract a potential error term from the error message
                            if msg.contains("variable") && msg.contains(':') {
                                msg.split(':').nth(1).map(|s| s.trim())
                            } else if msg.contains("'") {
                                let start = msg.find('\'').map(|i| i + 1).unwrap_or(0);
                                let end = msg[start..]
                                    .find('\'')
                                    .map(|i| i + start)
                                    .unwrap_or(msg.len());
                                Some(&msg[start..end])
                            } else {
                                None
                            }
                        }) {
                            if let Some(pos) = lines[i].find(error_term) {
                                let spaces = " ".repeat(6 + pos);
                                eprintln!(
                                    "{}\x1b[1;31m^\x1b[0m\x1b[31m-- Error occurs here\x1b[0m",
                                    spaces
                                );
                            } else {
                                eprintln!(
                                    "      \x1b[1;31m^\x1b[0m\x1b[31m-- Error occurs here\x1b[0m"
                                );
                            }
                        } else {
                            eprintln!(
                                "      \x1b[1;31m^\x1b[0m\x1b[31m-- Error occurs here\x1b[0m"
                            );
                        }
                    }
                } else {
                    eprintln!("\x1b[90m{:>4} |\x1b[0m {}", i + 1, lines[i]);
                }
            }
        }
    }

    eprintln!("\x1b[1;31m{}\x1b[0m", "=".repeat(50));
}
