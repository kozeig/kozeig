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
    Ternary {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
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
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        initializer: Expr,
        update: Expr,
        condition: Expr,
        body: Vec<Stmt>,
    },
    Break,
    Continue,
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
        // Handle the keywords for control structures
        if self.match_token(TokenType::If) {
            return self.if_statement();
        }

        if self.match_token(TokenType::While) {
            return self.while_statement();
        }

        if self.match_token(TokenType::For) {
            return self.for_statement();
        }

        if self.match_token(TokenType::Break) {
            return Ok(Stmt::Break);
        }

        if self.match_token(TokenType::Continue) {
            return Ok(Stmt::Continue);
        }

        if self.match_token(TokenType::Register) {
            let name = self.previous().lexeme.clone();

            self.consume(TokenType::Colon, "Expect ':' after register name.")?;

            // Skip any whitespace
            while self.match_token(TokenType::Newline) {}

            // Check for the new bracket syntax
            if self.match_token(TokenType::LeftBrace) {
                // We've got a type command with arguments in curly braces
                if self.match_token(TokenType::Command) {
                    let type_cmd = self.previous().lexeme.clone();

                    // Parse the value
                    let value = self.expression()?;

                    // Expect closing brace
                    self.consume(TokenType::RightBrace, "Expect '}' after type expression")?;

                    // Create a command expression for the type operation
                    let initializer = Expr::Command {
                        name: type_cmd,
                        args: vec![value],
                    };

                    return Ok(Stmt::Declaration {
                        name,
                        initializer,
                    });
                } else {
                    return Err("Expected type command after '{'".to_string());
                }
            } else {
                // Handle the direct value case (boolean literals, arithmetic expressions, etc.)
                let initializer = self.expression()?;

                return Ok(Stmt::Declaration {
                    name,
                    initializer,
                });
            }
        }

        if self.match_token(TokenType::Command) {
            let command = self.previous().lexeme.clone();

            // Check for the new command syntax with curly braces
            if self.match_token(TokenType::LeftBrace) {
                let mut args = Vec::new();

                // Parse command arguments
                if !self.check(TokenType::RightBrace) {
                    args.push(self.expression()?);

                    while self.match_token(TokenType::Comma) {
                        args.push(self.expression()?);
                    }
                }

                // Expect the closing brace
                self.consume(TokenType::RightBrace, "Expect '}' after command arguments")?;

                // Handle print command
                if command == "print" {
                    return Ok(Stmt::Print(args));
                } else {
                    return Ok(Stmt::Command {
                        name: command,
                        args,
                    });
                }
            } else {
                // No left brace found - this is an error in the new syntax
                return Err(format!("Expected '{{' after command '{}'", command));
            }
        }
        
        self.expression_statement()
    }
    
    // Parse an if statement with the newer syntax: if { condition } [ ... ] else [ ... ]
    fn if_statement(&mut self) -> Result<Stmt, String> {
        // Expect left brace for condition
        self.consume(TokenType::LeftBrace, "Expect '{' after 'if'")?;

        // Parse the condition
        let condition = self.expression()?;

        // Expect right brace after condition
        self.consume(TokenType::RightBrace, "Expect '}' after if condition")?;

        // Expect left bracket for if body
        self.consume(TokenType::LeftBracket, "Expect '[' to begin if body")?;

        // Skip any newlines
        while self.match_token(TokenType::Newline) {}

        // Parse the then branch statements
        let mut then_branch = Vec::new();

        // Continue until we hit a right bracket or EOF
        while !self.check(TokenType::RightBracket) && !self.is_at_end() {
            // Skip newlines
            if self.match_token(TokenType::Newline) {
                continue;
            }

            // Handle comments within blocks
            if self.match_token(TokenType::Comment) {
                let comment = self.previous().lexeme.clone();
                then_branch.push(Stmt::Comment(comment));
                continue;
            }

            // Process the statement
            let stmt = self.declaration()?;
            then_branch.push(stmt);
        }

        // Consume the closing bracket
        self.consume(TokenType::RightBracket, "Expect ']' after if body")?;

        // Check for an else clause
        let else_branch = if self.match_token(TokenType::Else) {
            // Expect left bracket for else body
            self.consume(TokenType::LeftBracket, "Expect '[' to begin else body")?;

            // Skip any newlines
            while self.match_token(TokenType::Newline) {}

            let mut else_statements = Vec::new();

            // Continue until we hit a right bracket or EOF
            while !self.check(TokenType::RightBracket) && !self.is_at_end() {
                // Skip newlines
                if self.match_token(TokenType::Newline) {
                    continue;
                }

                // Handle comments within blocks
                if self.match_token(TokenType::Comment) {
                    let comment = self.previous().lexeme.clone();
                    else_statements.push(Stmt::Comment(comment));
                    continue;
                }

                // Process the statement
                let stmt = self.declaration()?;
                else_statements.push(stmt);
            }

            // Consume the closing bracket
            self.consume(TokenType::RightBracket, "Expect ']' after else body")?;

            Some(else_statements)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    // Parse a while statement: while { condition } [ ... ]
    fn while_statement(&mut self) -> Result<Stmt, String> {
        // Expect left brace for condition
        self.consume(TokenType::LeftBrace, "Expect '{' after 'while'")?;

        // Parse the condition
        let condition = self.expression()?;

        // Expect right brace after condition
        self.consume(TokenType::RightBrace, "Expect '}' after while condition")?;

        // Expect left bracket for loop body
        self.consume(TokenType::LeftBracket, "Expect '[' to begin while loop body")?;

        // Skip any newlines
        while self.match_token(TokenType::Newline) {}

        // Parse the loop body
        let mut body = Vec::new();

        // Continue until we hit a right bracket or EOF
        while !self.check(TokenType::RightBracket) && !self.is_at_end() {
            // Skip newlines
            if self.match_token(TokenType::Newline) {
                continue;
            }

            // Handle comments within blocks
            if self.match_token(TokenType::Comment) {
                let comment = self.previous().lexeme.clone();
                body.push(Stmt::Comment(comment));
                continue;
            }

            // Process the statement
            let stmt = self.declaration()?;
            body.push(stmt);
        }

        // Consume the closing bracket
        self.consume(TokenType::RightBracket, "Expect ']' after while loop body")?;

        Ok(Stmt::While {
            condition,
            body,
        })
    }

    // Parse a for statement: for { init, update, condition } [ ... ]
    fn for_statement(&mut self) -> Result<Stmt, String> {
        // Expect left brace for the components
        self.consume(TokenType::LeftBrace, "Expect '{' after 'for'")?;

        // Parse the initializer expression - this is special handling for variable declarations
        let initializer = if self.match_token(TokenType::Register) {
            // We have a register (variable name), now we expect a colon
            let name = self.previous().lexeme.clone();

            self.consume(TokenType::Colon, "Expect ':' after register name in for loop initializer")?;

            // Get the expression after the colon
            let value_expr = self.expression()?;

            // Create a binary expression to represent the declaration
            let operator = Token::new(TokenType::Colon, ":".to_string(), self.previous().line);
            Expr::Binary {
                left: Box::new(Expr::VariableRef(name)),
                operator,
                right: Box::new(value_expr),
            }
        } else {
            // Regular expression for initializer
            self.expression()?
        };

        // Expect a comma
        self.consume(TokenType::Comma, "Expect ',' after initializer in for loop")?;

        // Parse the update expression
        let update = self.expression()?;

        // Expect a comma
        self.consume(TokenType::Comma, "Expect ',' after update expression in for loop")?;

        // Parse the condition
        let condition = self.expression()?;

        // Expect right brace
        self.consume(TokenType::RightBrace, "Expect '}' after for loop components")?;

        // Expect left bracket for loop body
        self.consume(TokenType::LeftBracket, "Expect '[' to begin for loop body")?;

        // Skip any newlines
        while self.match_token(TokenType::Newline) {}

        // Parse the loop body
        let mut body = Vec::new();

        // Continue until we hit a right bracket or EOF
        while !self.check(TokenType::RightBracket) && !self.is_at_end() {
            // Skip newlines
            if self.match_token(TokenType::Newline) {
                continue;
            }

            // Handle comments within blocks
            if self.match_token(TokenType::Comment) {
                let comment = self.previous().lexeme.clone();
                body.push(Stmt::Comment(comment));
                continue;
            }

            // Process the statement
            let stmt = self.declaration()?;
            body.push(stmt);
        }

        // Consume the closing bracket
        self.consume(TokenType::RightBracket, "Expect ']' after for loop body")?;

        Ok(Stmt::For {
            initializer,
            update,
            condition,
            body,
        })
    }

    fn check_command(&self, name: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        if self.peek().token_type != TokenType::Command {
            return false;
        }
        
        self.peek().lexeme == name
    }
    
    fn match_command(&mut self, name: &str) -> bool {
        if self.check_command(name) {
            self.advance();
            return true;
        }
        
        false
    }
    
    fn consume_command(&mut self, name: &str, message: &str) -> Result<&Token, String> {
        if self.check_command(name) {
            return Ok(self.advance());
        }
        
        Err(format!(
            "{} - Got {:?} at line {}",
            message,
            self.peek().token_type,
            self.peek().line
        ))
    }
    
    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        Ok(Stmt::Expression(expr))
    }
    
    fn expression(&mut self) -> Result<Expr, String> {
        self.conditional()
    }

    fn conditional(&mut self) -> Result<Expr, String> {
        // Parse the condition part
        let expr = self.logical_or()?;

        // Check if this is a ternary expression
        if self.match_token(TokenType::QuestionMark) {
            // Parse the "then" branch
            let then_branch = self.expression()?;

            // Expect a colon
            self.consume(TokenType::Colon, "Expect ':' in ternary expression")?;

            // Parse the "else" branch
            let else_branch = self.conditional()?;

            // Create and return the ternary expression
            return Ok(Expr::Ternary {
                condition: Box::new(expr),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            });
        }

        // Not a ternary expression, just return the parsed expression
        Ok(expr)
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

            // Check for the new command syntax with curly braces
            if self.match_token(TokenType::LeftBrace) {
                // Parse command arguments
                if !self.check(TokenType::RightBrace) {
                    args.push(self.expression()?);

                    while self.match_token(TokenType::Comma) {
                        args.push(self.expression()?);
                    }
                }

                // Expect the closing brace
                self.consume(TokenType::RightBrace, "Expect '}' after command arguments")?;
            }
            // Backward compatibility for -asc command
            else if name == "asc" && self.match_token(TokenType::Number) {
                let value = self.previous().lexeme.parse::<i64>().unwrap();
                args.push(Expr::NumberLiteral(value));
            }
            // Error case - no left brace found
            else {
                return Err(format!("Expected '{{' after command '{}' in expression", name));
            }

            return Ok(Expr::Command {
                name,
                args,
            });
        }
        
        // Special case: If we've reached EOF, just return a placeholder expression rather than error
        if self.is_at_end() || self.peek().token_type == TokenType::EOF {
            // Return a null expression instead of an error
            Ok(Expr::NumberLiteral(0))  // Placeholder that won't cause further errors
        } else {
            Err(format!(
                "Expected expression, got {:?} at line {}",
                self.peek().token_type,
                self.peek().line
            ))
        }
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
        self.current >= self.tokens.len() || self.peek().token_type == TokenType::EOF
    }
    
    fn peek(&self) -> &Token {
        if self.current >= self.tokens.len() {
            // If we're trying to look past the end, return the last token (which should be EOF)
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.current]
        }
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}