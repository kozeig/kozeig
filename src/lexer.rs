#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Register,        // variable declaration with ':'
    Command,         // commands starting with '-'
    Number,          // numeric literal
    Ascii,           // ascii code
    Text,            // string literal
    Variable,        // variable reference with '$'
    
    // Symbols
    Colon,           // ':'
    Comma,           // ','
    Semicolon,       // ';'
    
    // Arithmetic operators
    Plus,            // '+'
    Minus,           // '-'
    Star,            // '*'
    Slash,           // '/'
    Percent,         // '%'
    
    // Comparison operators
    Equal,           // '=='
    NotEqual,        // '!='
    Greater,         // '>'
    Less,            // '<'
    GreaterEqual,    // '>='
    LessEqual,       // '<='
    
    // Logical operators
    And,             // '&&'
    Or,              // '||'
    Not,             // '!'
    
    // Grouping
    LeftParen,       // '('
    RightParen,      // ')'
    
    // Comments and whitespace
    Comment,         // '@@'
    Newline,
    
    // End of file
    EOF,
}

#[derive(Debug, Clone)]
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
        
        self.tokens.push(Token::new(TokenType::EOF, String::new(), self.line));
        Ok(self.tokens.clone())
    }
    
    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        
        match c {
            ' ' | '\t' | '\r' => (),
            '\n' => {
                self.tokens.push(Token::new(TokenType::Newline, "\n".to_string(), self.line));
                self.line += 1;
            },
            ':' => self.tokens.push(Token::new(TokenType::Colon, ":".to_string(), self.line)),
            ',' => self.tokens.push(Token::new(TokenType::Comma, ",".to_string(), self.line)),
            ';' => {
                // Handle semicolons - check for double semicolon (;;)
                if self.match_char(';') {
                    self.tokens.push(Token::new(TokenType::Semicolon, ";;".to_string(), self.line));
                } else {
                    self.tokens.push(Token::new(TokenType::Semicolon, ";".to_string(), self.line));
                }
            },
            '$' => self.variable(),
            '\'' => self.string(),
            '@' => {
                if self.match_char('@') {
                    // Comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    let comment = self.source[self.start..self.current].iter().collect();
                    self.tokens.push(Token::new(TokenType::Comment, comment, self.line));
                } else {
                    return Err(format!("Unexpected character '@' at line {}", self.line));
                }
            },
            '-' => {
                // Check if this is a command (starts with a letter) or a unary minus
                if self.peek().is_alphabetic() {
                    // Command
                    while self.peek().is_alphabetic() && !self.is_at_end() {
                        self.advance();
                    }
                    let command = self.source[self.start..self.current].iter().collect();
                    self.tokens.push(Token::new(TokenType::Command, command, self.line));
                } else {
                    // Unary minus or minus operator
                    self.tokens.push(Token::new(TokenType::Minus, "-".to_string(), self.line));
                }
            },
            '+' => self.tokens.push(Token::new(TokenType::Plus, "+".to_string(), self.line)),
            '*' => self.tokens.push(Token::new(TokenType::Star, "*".to_string(), self.line)),
            '/' => self.tokens.push(Token::new(TokenType::Slash, "/".to_string(), self.line)),
            '%' => self.tokens.push(Token::new(TokenType::Percent, "%".to_string(), self.line)),
            '(' => self.tokens.push(Token::new(TokenType::LeftParen, "(".to_string(), self.line)),
            ')' => self.tokens.push(Token::new(TokenType::RightParen, ")".to_string(), self.line)),
            '=' => {
                if self.match_char('=') {
                    self.tokens.push(Token::new(TokenType::Equal, "==".to_string(), self.line));
                } else {
                    return Err(format!("Unexpected character '=' at line {}. Did you mean '=='?", self.line));
                }
            },
            '!' => {
                if self.match_char('=') {
                    self.tokens.push(Token::new(TokenType::NotEqual, "!=".to_string(), self.line));
                } else {
                    self.tokens.push(Token::new(TokenType::Not, "!".to_string(), self.line));
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.tokens.push(Token::new(TokenType::GreaterEqual, ">=".to_string(), self.line));
                } else {
                    self.tokens.push(Token::new(TokenType::Greater, ">".to_string(), self.line));
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.tokens.push(Token::new(TokenType::LessEqual, "<=".to_string(), self.line));
                } else {
                    self.tokens.push(Token::new(TokenType::Less, "<".to_string(), self.line));
                }
            },
            '&' => {
                if self.match_char('&') {
                    self.tokens.push(Token::new(TokenType::And, "&&".to_string(), self.line));
                } else {
                    return Err(format!("Unexpected character '&' at line {}. Did you mean '&&'?", self.line));
                }
            },
            '|' => {
                if self.match_char('|') {
                    self.tokens.push(Token::new(TokenType::Or, "||".to_string(), self.line));
                } else {
                    return Err(format!("Unexpected character '|' at line {}. Did you mean '||'?", self.line));
                }
            },
            _ => {
                if c.is_alphabetic() {
                    self.identifier();
                } else if c.is_digit(10) {
                    self.number();
                } else {
                    return Err(format!("Unexpected character '{}' at line {}", c, self.line));
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
        self.tokens.push(Token::new(TokenType::Register, text, self.line));
    }
    
    fn number(&mut self) {
        while self.peek().is_digit(10) && !self.is_at_end() {
            self.advance();
        }
        
        let value: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(TokenType::Number, value, self.line));
    }
    
    fn string(&mut self) {
        while self.peek() != '\'' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        
        if self.is_at_end() {
            panic!("Unterminated string at line {}", self.line);
        }
        
        // Closing quote
        self.advance();
        
        // Extract the string value (without the quotes)
        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.tokens.push(Token::new(TokenType::Text, value, self.line));
    }
    
    fn variable(&mut self) {
        while (self.peek().is_alphanumeric() || self.peek() == '_') && !self.is_at_end() {
            self.advance();
        }
        
        let value: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(TokenType::Variable, value, self.line));
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