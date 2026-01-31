use crate::error::{AsmError, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Identifiers and literals
    Identifier(String),
    Number(i32),
    String(String),

    // Symbols
    Colon,  // :
    Comma,  // ,
    Hash,   // #
    At,     // @
    Plus,   // +
    Minus,  // -
    Star,   // *
    Slash,  // /
    LParen, // (
    RParen, // )
    Equals, // =
    Dot,    // .

    // End of line
    Newline,
    Eof,
}

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace_and_comments();

        if self.is_at_end() {
            return Ok(Token::Eof);
        }

        let ch = self.current();

        match ch {
            '\n' => {
                self.advance();
                self.line += 1;
                self.column = 1;
                Ok(Token::Newline)
            }
            ':' => {
                self.advance();
                Ok(Token::Colon)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }
            '#' => {
                self.advance();
                Ok(Token::Hash)
            }
            '@' => {
                self.advance();
                Ok(Token::At)
            }
            '+' => {
                self.advance();
                Ok(Token::Plus)
            }
            '-' => {
                self.advance();
                Ok(Token::Minus)
            }
            '*' => {
                self.advance();
                Ok(Token::Star)
            }
            '/' => {
                self.advance();
                Ok(Token::Slash)
            }
            '(' => {
                self.advance();
                Ok(Token::LParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RParen)
            }
            '=' => {
                self.advance();
                Ok(Token::Equals)
            }
            '.' => {
                self.advance();
                if self.current().is_alphabetic() {
                    self.position -= 1;
                    self.column -= 1;
                    self.read_directive()
                } else {
                    Ok(Token::Dot)
                }
            }
            '"' | '\'' => self.read_string(),
            _ if ch.is_alphabetic() || ch == '_' => self.read_identifier(),
            _ if ch.is_ascii_digit() => self.read_number(),
            _ => Err(AsmError::new(
                self.line,
                self.column,
                format!("Unexpected character: {ch}"),
            )),
        }
    }

    fn current(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    fn peek(&self) -> char {
        if self.position + 1 >= self.input.len() {
            '\0'
        } else {
            self.input[self.position + 1]
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
            self.column += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            match self.current() {
                ' ' | '\t' | '\r' => self.advance(),
                ';' => {
                    // Skip to end of line
                    while !self.is_at_end() && self.current() != '\n' {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn read_identifier(&mut self) -> Result<Token> {
        let start = self.position;
        while !self.is_at_end()
            && (self.current().is_alphanumeric()
                || self.current() == '_'
                || self.current() == '.'
                || self.current() == '$')
        {
            self.advance();
        }
        let name: String = self.input[start..self.position].iter().collect();
        Ok(Token::Identifier(name))
    }

    fn read_directive(&mut self) -> Result<Token> {
        self.advance(); // skip '.'
        let start = self.position;
        while !self.is_at_end() && self.current().is_alphanumeric() {
            self.advance();
        }
        let name: String = self.input[start..self.position].iter().collect();
        Ok(Token::Identifier(format!(".{name}")))
    }

    fn read_number(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_col = self.column;

        // Check for octal prefix (0)
        let radix = if self.current() == '0' && !self.peek().is_ascii_digit() {
            self.advance();
            return Ok(Token::Number(0));
        } else if self.current() == '0' {
            8 // Octal
        } else {
            10 // Decimal
        };

        let start = self.position;
        while !self.is_at_end() && self.current().is_ascii_alphanumeric() {
            self.advance();
        }

        let num_str: String = self.input[start..self.position].iter().collect();
        let value = i32::from_str_radix(&num_str, radix).map_err(|_| {
            AsmError::new(start_line, start_col, format!("Invalid number: {num_str}"))
        })?;

        Ok(Token::Number(value))
    }

    fn read_string(&mut self) -> Result<Token> {
        let quote = self.current();
        self.advance(); // skip opening quote

        let mut s = String::new();
        while !self.is_at_end() && self.current() != quote {
            if self.current() == '\\' {
                self.advance();
                if self.is_at_end() {
                    return Err(AsmError::new(self.line, self.column, "Unterminated string"));
                }
                // Simple escape handling
                match self.current() {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    'r' => s.push('\r'),
                    '\\' => s.push('\\'),
                    '"' => s.push('"'),
                    '\'' => s.push('\''),
                    _ => s.push(self.current()),
                }
            } else {
                s.push(self.current());
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(AsmError::new(self.line, self.column, "Unterminated string"));
        }

        self.advance(); // skip closing quote
        Ok(Token::String(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("MOV R0, R1");
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Identifier("MOV".to_string())
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Identifier("R0".to_string())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Identifier("R1".to_string())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_addressing_modes() {
        let mut lexer = Lexer::new("#100, @(R0)+, -(R1)");
        assert_eq!(lexer.next_token().unwrap(), Token::Hash);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(100));
    }
}
