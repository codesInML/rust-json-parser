use std::{str::Chars, vec};

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenType {
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Colon,        // :
    Comma,        // ,
    String,       // ""
    Number,       // 123
    Boolean,      // true or false
    Null,         // null
    EndOfFile,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
    position: usize,
}

pub struct Lexer {
    content: String,
    position: usize,
    tokens: Vec<Token>,
}

impl Token {
    fn new(token_type: TokenType, value: Option<String>, position: usize) -> Self {
        Token {
            token_type,
            value,
            position,
        }
    }
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Lexer {
            content,
            position: 0,
            tokens: Vec::new(),
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        return &self.tokens;
    }

    fn tokenize_string(chars: &mut Chars) -> (String, usize) {
        let mut count = 0;
        let mut str_content = String::new();

        while let Some(char) = chars.next() {
            count += 1;

            if char == '\n' {
                break;
            } else if char != '"' {
                str_content.push_str(&char.to_string());
            } else {
                break;
            }
        }

        return (str_content, count);
    }

    fn tokenize_non_string(chars: &mut Chars, content: &mut String) -> (usize, Option<char>) {
        let mut count = 0;
        let mut end_char = None;

        while let Some(char) = chars.next() {
            count += 1;

            if char.is_whitespace() {
                break;
            } else if char != ',' && char != '}' && char != ']' {
                content.push_str(&char.to_string());
            } else {
                end_char = Some(char);
            }
        }

        return (count, end_char);
    }

    fn non_string_token_gen(
        chars: &mut Chars,
        char: char,
        token_type: TokenType,
        position: usize,
    ) -> (Vec<Token>, usize) {
        let mut tokens = Vec::new();

        let mut content = char.to_string();
        let (count, end_char) = Lexer::tokenize_non_string(chars, &mut content);

        match end_char {
            Some(char) => {
                tokens.push(Token::new(token_type, Some(content), position + count));
                if char == ',' {
                    tokens.push(Token::new(TokenType::Comma, None, position + count));
                } else if char == '}' {
                    tokens.push(Token::new(TokenType::LeftBrace, None, position + count));
                } else {
                    tokens.push(Token::new(TokenType::LeftBracket, None, position + count));
                }
            }
            None => tokens.push(Token::new(token_type, Some(content), position + count)),
        }

        return (tokens, count);
    }

    pub fn tokenize(&mut self) {
        let mut chars = self.content.chars();

        while let Some(char) = chars.next() {
            if char.is_whitespace() {
                continue;
            } else if char == '{' {
                self.tokens
                    .push(Token::new(TokenType::LeftBrace, None, self.position));
            } else if char == '}' {
                self.tokens
                    .push(Token::new(TokenType::RightBrace, None, self.position));
            } else if char == '[' {
                self.tokens
                    .push(Token::new(TokenType::LeftBracket, None, self.position));
            } else if char == ']' {
                self.tokens
                    .push(Token::new(TokenType::RightBracket, None, self.position));
            } else if char == ':' {
                self.tokens
                    .push(Token::new(TokenType::Colon, None, self.position));
            } else if char == ',' {
                self.tokens
                    .push(Token::new(TokenType::Comma, None, self.position));
            } else if char == '"' {
                let (str_content, count) = Lexer::tokenize_string(&mut chars);
                self.position += count;
                self.tokens.push(Token::new(
                    TokenType::String,
                    Some(str_content),
                    self.position,
                ));
            } else if char == 'f' || char == 't' {
                let (mut tokens, count) = Lexer::non_string_token_gen(
                    &mut chars,
                    char,
                    TokenType::Boolean,
                    self.position,
                );
                self.tokens.append(&mut tokens);
                self.position += count;
            } else if char == 'n' {
                let (mut tokens, count) =
                    Lexer::non_string_token_gen(&mut chars, char, TokenType::Null, self.position);
                self.tokens.append(&mut tokens);
                self.position += count;
            } else {
                let (mut tokens, count) =
                    Lexer::non_string_token_gen(&mut chars, char, TokenType::Number, self.position);
                self.tokens.append(&mut tokens);
                self.position += count;
            }
            self.position += 1;
        }

        self.tokens
            .push(Token::new(TokenType::EndOfFile, None, self.position));
    }
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn token_error(position: usize, value: Option<&String>) -> String {
        match value {
            Some(value) => format!("Unexpected token {} at position {}.", value, position),
            None => format!("Unexpected token at position {}.", position),
        }
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }
    }

    fn get_current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_comma(&self) -> bool {
        self.get_current_token().token_type == TokenType::Comma
    }

    fn is_colon(&self) -> bool {
        self.get_current_token().token_type == TokenType::Colon
    }

    fn is_right_brace(&self) -> bool {
        self.get_current_token().token_type == TokenType::RightBrace
    }

    fn is_right_bracket(&self) -> bool {
        self.get_current_token().token_type == TokenType::RightBracket
    }

    fn is_null(value: &String) -> bool {
        return *value == "null".to_string();
    }

    fn is_boolean(value: &String) -> bool {
        return *value == "false".to_string() || *value == "true".to_string();
    }

    fn is_number(value: &String) -> bool {
        return value.parse::<f64>().is_ok();
    }

    fn is_not_valid_array_next_value(&self, token: &TokenType) -> bool {
        vec![
            TokenType::Colon,
            TokenType::Comma,
            TokenType::RightBrace,
            TokenType::RightBracket,
            TokenType::EndOfFile,
        ]
        .contains(token)
    }

    fn expect_end_of_file(&self) -> bool {
        let token = self.tokens.get(self.current + 1);

        match token {
            Some(token) => {
                if token.token_type != TokenType::EndOfFile {
                    return false;
                }
            }
            None => return false,
        }

        true
    }

    fn get_value(value: Option<&String>) -> Result<&String, String> {
        match value {
            Some(value) => Ok(value),
            None => Err("invalid token".to_string()),
        }
    }

    fn validate_value(&mut self) -> Result<(), String> {
        let token = self.get_current_token();

        match token.token_type {
            TokenType::String => {}
            TokenType::Boolean => {
                let value = Parser::get_value(token.value.as_ref())?;
                if !Parser::is_boolean(value) {
                    return Err(Parser::token_error(token.position, token.value.as_ref()));
                }
            }
            TokenType::Number => {
                let value = Parser::get_value(token.value.as_ref())?;
                if !Parser::is_number(value) {
                    return Err(Parser::token_error(token.position, token.value.as_ref()));
                }
            }
            TokenType::Null => {
                let value = Parser::get_value(token.value.as_ref())?;
                if !Parser::is_null(value) {
                    return Err(Parser::token_error(token.position, token.value.as_ref()));
                }
            }
            TokenType::LeftBrace => {
                self.advance();
                self.validate_object()?;
            }
            TokenType::LeftBracket => {
                self.advance();
                self.validate_array()?;
            }
            _ => {
                return Err(Parser::token_error(token.position, token.value.as_ref()));
            }
        }

        return Ok(());
    }

    fn validate_object(&mut self) -> Result<(), String> {
        loop {
            match self.get_current_token().token_type {
                TokenType::RightBrace => break,
                TokenType::String => {
                    self.advance();
                    if !self.is_colon() {
                        let token = self.get_current_token();
                        return Err(Parser::token_error(token.position, token.value.as_ref()));
                    }
                    self.advance();
                    self.validate_value()?;
                    self.advance();

                    if self.is_comma() {
                        self.advance();
                        let token = self.get_current_token();
                        if token.token_type != TokenType::String {
                            return Err(Parser::token_error(token.position, token.value.as_ref()));
                        }
                        continue;
                    } else if self.is_right_brace() {
                        break;
                    } else {
                        let token = self.get_current_token();
                        return Err(Parser::token_error(token.position, token.value.as_ref()));
                    }
                }
                _ => {
                    let token = self.get_current_token();
                    return Err(Parser::token_error(token.position, token.value.as_ref()));
                }
            }
        }

        Ok(())
    }

    fn validate_array(&mut self) -> Result<(), String> {
        loop {
            let token_type = self.get_current_token().token_type;

            if token_type == TokenType::RightBracket {
                break;
            }
            self.validate_value()?;
            self.advance();

            if self.is_comma() {
                self.advance();
                let token = self.get_current_token();
                if self.is_not_valid_array_next_value(&token.token_type) {
                    return Err(Parser::token_error(token.position, token.value.as_ref()));
                }
                continue;
            } else if self.is_right_bracket() {
                break;
            } else {
                let token = self.get_current_token();
                return Err(Parser::token_error(token.position, token.value.as_ref()));
            }
        }

        Ok(())
    }

    fn validate_first_token(&self) -> Result<(), String> {
        let token = self.get_current_token();

        if token.token_type == TokenType::EndOfFile {
            let msg = format!("empty JSON file");
            return Err(msg);
        } else if token.token_type != TokenType::LeftBrace
            && token.token_type != TokenType::LeftBracket
        {
            match &token.value {
                Some(value) => {
                    if token.token_type == TokenType::Null && !Parser::is_null(value) {
                        return Err(Parser::token_error(token.position, Some(value)));
                    }
                    if token.token_type == TokenType::Boolean && !Parser::is_boolean(value) {
                        return Err(Parser::token_error(token.position, Some(value)));
                    }
                    if token.token_type == TokenType::Number && !Parser::is_number(value) {
                        return Err(Parser::token_error(token.position, Some(value)));
                    }

                    if !self.expect_end_of_file() {
                        return Err(Parser::token_error(token.position, Some(value)));
                    }
                }
                None => {
                    return Err(Parser::token_error(token.position, None));
                }
            }
        }

        return Ok(());
    }

    pub fn parse(&mut self) -> Result<i32, String> {
        self.validate_first_token()?;
        let token_type = self.get_current_token().token_type;

        if token_type != TokenType::LeftBrace && token_type != TokenType::LeftBracket {
            return Ok(0);
        }

        self.advance();

        match token_type {
            TokenType::LeftBrace => {
                self.validate_object()?;
            }
            TokenType::LeftBracket => {}
            _ => {
                let token = self.get_current_token();
                return Err(Parser::token_error(token.position, token.value.as_ref()));
            }
        }

        if !self.expect_end_of_file() {
            let token = self.get_current_token();
            return Err(Parser::token_error(token.position, token.value.as_ref()));
        }

        Ok(0)
    }
}
