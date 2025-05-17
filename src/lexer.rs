#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Register, // variable declaration with ':'
    Command,  // commands starting with '-'
    Number,   // integer literal
    Float,    // floating point literal
    Hex,      // hexadecimal literal
    Binary,   // binary literal
    Ascii,    // ascii code
    Text,     // string literal
    Variable, // variable reference with '$'
    Boolean,  // boolean literal (true/false)
    If,       // 'if' keyword
    Else,     // 'else' keyword
    While,    // 'while' keyword
    For,      // 'for' keyword
    Break,    // 'break' keyword
    Continue, // 'continue' keyword

    // Function related keywords
    Func, // 'func' keyword for function definitions
    Pub,  // 'pub' keyword for public functions
    Prot, // 'prot' keyword for protected functions
    Call, // 'call' keyword for function calls

    // Import related keywords
    Use,  // 'use' keyword for imports
    From, // 'from' keyword for imports

    // Symbols
    Colon,              // ':'
    DoubleColon,        // '::'
    Comma,              // ','
    Semicolon,          // ';'
    StatementSeparator, // ';;'
    QuestionMark,       // '?'
    LeftBracket,        // '['
    RightBracket,       // ']'

    // Arithmetic operators
    Plus,    // '+'
    Minus,   // '-'
    Star,    // '*'
    Slash,   // '/'
    Percent, // '%'

    // Comparison operators
    Equal,        // '=='
    NotEqual,     // '!='
    Greater,      // '>'
    Less,         // '<'
    GreaterEqual, // '>='
    LessEqual,    // '<='

    // Logical operators
    And, // '&&'
    Or,  // '||'
    Not, // '!'

    // Grouping
    LeftParen,  // '('
    RightParen, // ')'
    LeftBrace,  // '{'
    RightBrace, // '}'

    // Comments and whitespace
    Comment, // '@@' or '--' for comments
    Newline,

    // Version specifier for imports
    At,  // '@' for version specification
    Dot, // '.' for path separators in imports

    // End of file
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::EOF, String::new(), self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();

        match c {
            ' ' | '\t' | '\r' => (),
            '\n' => {
                self.tokens
                    .push(Token::new(TokenType::Newline, "\n".to_string(), self.line));
                self.line += 1;
            }
            ':' => {
                // Check for double colon ::
                if self.match_char(':') {
                    self.tokens.push(Token::new(
                        TokenType::DoubleColon,
                        "::".to_string(),
                        self.line,
                    ));
                } else {
                    self.tokens
                        .push(Token::new(TokenType::Colon, ":".to_string(), self.line));
                }
            }
            '?' => self.tokens.push(Token::new(
                TokenType::QuestionMark,
                "?".to_string(),
                self.line,
            )),
            ',' => self
                .tokens
                .push(Token::new(TokenType::Comma, ",".to_string(), self.line)),
            ';' => {
                // Handle semicolons - check for double semicolon (;;)
                if self.match_char(';') {
                    // Double semicolon is a statement separator
                    self.tokens.push(Token::new(
                        TokenType::StatementSeparator,
                        ";;".to_string(),
                        self.line,
                    ));
                } else {
                    // Single semicolon
                    self.tokens
                        .push(Token::new(TokenType::Semicolon, ";".to_string(), self.line));
                }
            }
            '$' => self.variable(),
            '\'' => self.string()?,
            '-' => {
                if self.match_char('-') {
                    // Comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    let comment = self.source[self.start..self.current].iter().collect();
                    self.tokens
                        .push(Token::new(TokenType::Comment, comment, self.line));
                } else {
                    // Just handle as minus operator
                    self.tokens
                        .push(Token::new(TokenType::Minus, "-".to_string(), self.line));
                }
            }
            '+' => self
                .tokens
                .push(Token::new(TokenType::Plus, "+".to_string(), self.line)),
            '*' => self
                .tokens
                .push(Token::new(TokenType::Star, "*".to_string(), self.line)),
            '/' => self
                .tokens
                .push(Token::new(TokenType::Slash, "/".to_string(), self.line)),
            '%' => self
                .tokens
                .push(Token::new(TokenType::Percent, "%".to_string(), self.line)),
            '(' => self
                .tokens
                .push(Token::new(TokenType::LeftParen, "(".to_string(), self.line)),
            ')' => self.tokens.push(Token::new(
                TokenType::RightParen,
                ")".to_string(),
                self.line,
            )),
            '{' => self
                .tokens
                .push(Token::new(TokenType::LeftBracket, "{".to_string(), self.line)),
            '}' => self.tokens.push(Token::new(
                TokenType::RightBracket,
                "}".to_string(),
                self.line,
            )),
            '[' => self.tokens.push(Token::new(
                TokenType::LeftBrace,
                "[".to_string(),
                self.line,
            )),
            ']' => self.tokens.push(Token::new(
                TokenType::RightBrace,
                "]".to_string(),
                self.line,
            )),
            '=' => {
                if self.match_char('=') {
                    self.tokens
                        .push(Token::new(TokenType::Equal, "==".to_string(), self.line));
                } else {
                    return Err(format!(
                        "Unexpected character '=' at line {}. Did you mean '=='?",
                        self.line
                    ));
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.tokens
                        .push(Token::new(TokenType::NotEqual, "!=".to_string(), self.line));
                } else {
                    self.tokens
                        .push(Token::new(TokenType::Not, "!".to_string(), self.line));
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.tokens.push(Token::new(
                        TokenType::GreaterEqual,
                        ">=".to_string(),
                        self.line,
                    ));
                } else {
                    self.tokens
                        .push(Token::new(TokenType::Greater, ">".to_string(), self.line));
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.tokens.push(Token::new(
                        TokenType::LessEqual,
                        "<=".to_string(),
                        self.line,
                    ));
                } else {
                    self.tokens
                        .push(Token::new(TokenType::Less, "<".to_string(), self.line));
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.tokens
                        .push(Token::new(TokenType::And, "&&".to_string(), self.line));
                } else {
                    return Err(format!(
                        "Unexpected character '&' at line {}. Did you mean '&&'?",
                        self.line
                    ));
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.tokens
                        .push(Token::new(TokenType::Or, "||".to_string(), self.line));
                } else {
                    return Err(format!(
                        "Unexpected character '|' at line {}. Did you mean '||'?",
                        self.line
                    ));
                }
            }
            '@' => self
                .tokens
                .push(Token::new(TokenType::At, "@".to_string(), self.line)),
            '.' => self
                .tokens
                .push(Token::new(TokenType::Dot, ".".to_string(), self.line)),
            _ => {
                if c.is_alphabetic() {
                    self.identifier();
                } else if c.is_digit(10) {
                    self.number();
                } else {
                    return Err(format!(
                        "Unexpected character '{}' at line {}",
                        c, self.line
                    ));
                }
            }
        }

        Ok(())
    }

    fn identifier(&mut self) {
        while (self.peek().is_alphanumeric() || self.peek() == '_') && !self.is_at_end() {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();

        // Check for reserved keywords
        match text.as_str() {
            "true" | "false" => self
                .tokens
                .push(Token::new(TokenType::Boolean, text, self.line)),
            "if" => self.tokens.push(Token::new(TokenType::If, text, self.line)),
            "else" => self
                .tokens
                .push(Token::new(TokenType::Else, text, self.line)),
            "while" => self
                .tokens
                .push(Token::new(TokenType::While, text, self.line)),
            "for" => self
                .tokens
                .push(Token::new(TokenType::For, text, self.line)),
            "break" => self
                .tokens
                .push(Token::new(TokenType::Break, text, self.line)),
            "continue" => self
                .tokens
                .push(Token::new(TokenType::Continue, text, self.line)),
            // Function-related keywords
            "func" => self
                .tokens
                .push(Token::new(TokenType::Func, text, self.line)),
            "pub" => self
                .tokens
                .push(Token::new(TokenType::Pub, text, self.line)),
            "prot" => self
                .tokens
                .push(Token::new(TokenType::Prot, text, self.line)),
            "call" => self
                .tokens
                .push(Token::new(TokenType::Call, text, self.line)),
            "use" => self
                .tokens
                .push(Token::new(TokenType::Use, text, self.line)),
            "from" => self
                .tokens
                .push(Token::new(TokenType::From, text, self.line)),
            "print" | "text" | "number" | "bool" | "asc" | "fp" | "hex" | "bin" | "array" => self
                .tokens
                .push(Token::new(TokenType::Command, text, self.line)),
            _ => self
                .tokens
                .push(Token::new(TokenType::Register, text, self.line)),
        }
    }

    fn number(&mut self) {
        // Check for hex format
        if self.peek() == '0' && (self.peek_next() == 'x' || self.peek_next() == 'X') {
            // Consume '0x' or '0X'
            self.advance(); // '0'
            self.advance(); // 'x' or 'X'

            // Parse hex digits
            while self.peek().is_digit(16)
                || ('a'..='f').contains(&self.peek())
                || ('A'..='F').contains(&self.peek())
            {
                self.advance();
            }

            let value: String = self.source[self.start..self.current].iter().collect();
            self.tokens
                .push(Token::new(TokenType::Hex, value, self.line));
            return;
        }

        // Check for binary format
        if self.peek() == '0' && (self.peek_next() == 'b' || self.peek_next() == 'B') {
            // Consume '0b' or '0B'
            self.advance(); // '0'
            self.advance(); // 'b' or 'B'

            // Parse binary digits
            while self.peek() == '0' || self.peek() == '1' {
                self.advance();
            }

            let value: String = self.source[self.start..self.current].iter().collect();
            self.tokens
                .push(Token::new(TokenType::Binary, value, self.line));
            return;
        }

        // Parse integer part
        while self.peek().is_digit(10) && !self.is_at_end() {
            self.advance();
        }

        // Look for a decimal point followed by a number
        let is_float = if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance(); // Consume the '.'

            // Parse decimal part
            while self.peek().is_digit(10) && !self.is_at_end() {
                self.advance();
            }

            true
        } else {
            false
        };

        let value: String = self.source[self.start..self.current].iter().collect();

        if is_float {
            self.tokens
                .push(Token::new(TokenType::Float, value, self.line));
        } else {
            self.tokens
                .push(Token::new(TokenType::Number, value, self.line));
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '\'' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(format!("Unterminated string at line {}", self.line));
        }

        // Closing quote
        self.advance();

        // Extract the string value (without the quotes)
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.tokens
            .push(Token::new(TokenType::Text, value, self.line));

        Ok(())
    }

    fn variable(&mut self) {
        while (self.peek().is_alphanumeric() || self.peek() == '_') && !self.is_at_end() {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(TokenType::Variable, value, self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
