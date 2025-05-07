use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    VariableRef(String),
    NumberLiteral(i64),
    TextLiteral(String),
    BooleanLiteral(bool),
    Command {
        name: String,
        args: Vec<Expr>,
    },
    // New operator expressions
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Declaration {
        name: String,
        initializer: Expr,
    },
    Expression(Expr),
    Command {
        name: String, 
        args: Vec<Expr>,
    },
    Print(Vec<Expr>),
    Comment(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }
    
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            if self.match_token(TokenType::Newline) || self.match_token(TokenType::Semicolon) {
                continue;
            }
            
            if self.match_token(TokenType::Comment) {
                let comment = self.previous().lexeme.clone();
                statements.push(Stmt::Comment(comment));
                continue;
            }
            
            let stmt = self.declaration()?;
            statements.push(stmt);
            
            // Skip any newlines or semicolons
            while self.match_token(TokenType::Newline) || self.match_token(TokenType::Semicolon) {}
        }
        
        Ok(statements)
    }
    
    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(TokenType::Register) {
            let name = self.previous().lexeme.clone();
            
            self.consume(TokenType::Colon, "Expect ':' after register name.")?;
            
            // Skip any whitespace
            while self.match_token(TokenType::Newline) {}
            
            let initializer = self.expression()?;
            
            return Ok(Stmt::Declaration {
                name,
                initializer,
            });
        }
        
        if self.match_token(TokenType::Command) {
            let command = self.previous().lexeme.clone();
            
            if command == "-print" {
                let mut args = Vec::new();
                
                // Handle arguments
                if !self.check(TokenType::Newline) && !self.check(TokenType::EOF) && !self.check(TokenType::Semicolon) {
                    args.push(self.expression()?);
                    
                    while self.match_token(TokenType::Comma) {
                        args.push(self.expression()?);
                    }
                }
                
                return Ok(Stmt::Print(args));
            } else {
                // Other commands
                let mut args = Vec::new();
                
                // Special handling for -asc which requires exactly one numeric argument
                if command == "-asc" && self.match_token(TokenType::Number) {
                    let value = self.previous().lexeme.parse::<i64>().unwrap();
                    args.push(Expr::NumberLiteral(value));
                }
                // Handle arguments if any for other commands
                else if !self.check(TokenType::Newline) && !self.check(TokenType::EOF) && !self.check(TokenType::Semicolon) {
                    args.push(self.expression()?);
                    
                    while self.match_token(TokenType::Comma) {
                        args.push(self.expression()?);
                    }
                }
                
                return Ok(Stmt::Command {
                    name: command,
                    args,
                });
            }
        }
        
        self.expression_statement()
    }
    
    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        Ok(Stmt::Expression(expr))
    }
    
    fn expression(&mut self) -> Result<Expr, String> {
        self.logical_or()
    }
    
    fn logical_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.logical_and()?;
        
        while self.match_token(TokenType::Or) {
            let operator = self.previous().clone();
            let right = self.logical_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn logical_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        
        while self.match_token(TokenType::And) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        
        while self.match_token(TokenType::Equal) || self.match_token(TokenType::NotEqual) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        
        while self.match_token(TokenType::Greater) || 
              self.match_token(TokenType::GreaterEqual) || 
              self.match_token(TokenType::Less) || 
              self.match_token(TokenType::LessEqual) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        
        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        
        while self.match_token(TokenType::Star) || 
              self.match_token(TokenType::Slash) || 
              self.match_token(TokenType::Percent) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(TokenType::Not) || self.match_token(TokenType::Minus) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        
        self.primary()
    }
    
    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(TokenType::Variable) {
            let name = self.previous().lexeme.clone();
            return Ok(Expr::VariableRef(name));
        }
        
        if self.match_token(TokenType::Number) {
            let value = self.previous().lexeme.parse::<i64>().unwrap();
            return Ok(Expr::NumberLiteral(value));
        }
        
        if self.match_token(TokenType::Text) {
            let value = self.previous().lexeme.clone();
            return Ok(Expr::TextLiteral(value));
        }
        
        if self.match_token(TokenType::Boolean) {
            let value = self.previous().lexeme.clone() == "true";
            return Ok(Expr::BooleanLiteral(value));
        }
        
        if self.match_token(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }
        
        if self.match_token(TokenType::Command) {
            let name = self.previous().lexeme.clone();
            let mut args = Vec::new();
            
            // Handle specific commands that require exactly one numeric argument
            if name == "-asc" && self.match_token(TokenType::Number) {
                let value = self.previous().lexeme.parse::<i64>().unwrap();
                args.push(Expr::NumberLiteral(value));
            }
            // Handle other commands
            else if !self.check(TokenType::Newline) && !self.check(TokenType::EOF) && !self.check(TokenType::Semicolon) {
                args.push(self.expression()?);
                
                while self.match_token(TokenType::Comma) {
                    args.push(self.expression()?);
                }
            }
            
            return Ok(Expr::Command {
                name,
                args,
            });
        }
        
        Err(format!(
            "Expected expression, got {:?} at line {}",
            self.peek().token_type,
            self.peek().line
        ))
    }
    
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        
        Err(format!(
            "{} - Got {:?} at line {}",
            message,
            self.peek().token_type,
            self.peek().line
        ))
    }
    
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }
    
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}