use std::{collections::HashMap, iter::Peekable};
use crate::token::{Token, TokenType};

pub struct Lexer{
    keywords: HashMap<String, TokenType>,
}
impl Lexer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let keywords: HashMap<String, TokenType> = HashMap::new();
        Self {
            keywords,
        }
    }
    pub fn new_keywords(keywords: HashMap<String, TokenType>) -> Self {
        Self {
        keywords,
        }
    }
    
    pub fn get_next_token<I>(&self, iterator: &mut Peekable<I>) -> Option<Token> where I: Iterator<Item = Result<char,utf8_read::Error>> {
        loop {
            let ch = match iterator.next() {
                Some(Ok(value)) => value,
                _ => return  None
            };
            
            let p = match iterator.peek() {
                Some(Ok(value)) => *value,
                _ => '\0', // Default value if iterator.peek() returns None or an error
            };
            // Ignore whitespace characters
            if ch.is_whitespace() {
                continue;
            }

            // Handle operators
            match ch {
                '^' | '%' | '+' | '-' | '*' | ',' | '.' => return Some(Token::new(TokenType::Operator, ch.to_string())),
                '(' => return Some(Token::new(TokenType::LParen,String::new())),
                ')' => return Some(Token::new(TokenType::RParen,String::new())),
                '{' => return Some(Token::new(TokenType::LBrace,String::new())),
                '}' => return Some(Token::new(TokenType::RBrace,String::new())),
                '[' => return Some(Token::new(TokenType::LSquareBrace,String::new())),
                ']' => return Some(Token::new(TokenType::RSquareBrace,String::new())),
                ';' => return Some(Token::new(TokenType::Semicolon,String::new())),
                ':' => return Some(Token::new(TokenType::Colon,String::new())),
                '!' | '~' => return Some(Token::new(TokenType::UnOperator,'!'.to_string())),
                '=' => {
                if p == '=' || p == '>'{
                    let mut temp = ch.to_string();
                    temp.push(p);
                    iterator.next();
                    return Some(Token::new(TokenType::Operator,temp));
                } 
                return Some(Token::new(TokenType::Operator,ch.to_string()));
                }
                '/' => {
                if p == '/' {
                    // Skip line comments
                    while let Some(Ok(ch)) = iterator.next() {
                        if ch == '\n' {
                        break;
                        }
                    }
                    continue;
                } else if p == '*' {
                    let mut x = false;
                    while let Some(Ok(ch)) = iterator.next() {
                        if ch == '*' {
                            x = true;
                            continue;
                        }
                        else if x && ch == '/' {
                            break;
                        }
                        x = false;
                        continue;
                    }
                } 
                else {
                    return Some(Token::new(TokenType::Operator, ch.to_string()));
                }
                }
                '|' | '&' | '<' | '>' => {
                if p == ch || p == '=' {
                    // Handle compound operators
                    let mut temp = ch.to_string();
                    temp.push(ch);
                    iterator.next();
                    return Some(Token::new(TokenType::Operator, temp));
                } else {
                    return Some(Token::new(TokenType::Operator, ch.to_string()));
                }
                }
                '#' => return Some(self.parse_name('#', iterator)),
                '"' => return Some(Lexer::parse_string(iterator)),
                '\'' =>{
                let mut buff = String::new();
                while let Some(Ok(x)) = iterator.next() {
                    if x == '\''{
                        break;
                    }
                    buff.push(x);
                }
                if buff.len() == 1{
                    return Some(Token::new(TokenType::Char, buff));
                }
                if buff.starts_with('\\') && buff.len() == 2{
                    let c = buff.pop().unwrap();
                    match c {
                        'n' => return Some(Token::new(TokenType::Char, "\n".to_string())),
                        _ => panic!("Wrong escape sequence!"),
                    }
                }
                println!()
                }
                _ => {
                    // Handle numbers and identifiers
                    if ch.is_digit(10) {
                        return Some(Lexer::parse_number(ch, iterator));
                    } else if ch.is_ascii_alphabetic() {
                        return Some(self.parse_name(ch, iterator));
                    } else {
                        println!("Error while lexing: {} | {} unexpected", ch, ch as u64);
                        return None;
                    }
                }
            }
        }
    }

    fn parse_number<I>(first_digit: char, iterator: &mut Peekable<I>) -> Token  where I: Iterator<Item = Result<char, utf8_read::Error>>{
        let mut buff = String::new();
        let mut digit = first_digit;
        let mut radix = 10;
        match iterator.peek() {
            None => return Token::new(TokenType::Number, buff),
            Some(Ok(d)) =>{
                if *d == 'b'{
                    radix = 1;
                }
                else if *d == 'x' {
                    radix = 16;
                }
            }
            _ => {}
        }
        loop {
            buff.push(digit);
            
            if let Some(Ok(ch)) = iterator.peek() {
                if ch.is_digit(radix) || *ch == 'x' {
                    digit = *ch;
                    iterator.next();
                    continue;
                }
            } else {
                break; // Stop if end of input is reached
            }
            
            return Token::new(TokenType::Number, buff);
        }
        // If loop exits without returning, return a number token
        Token::new(TokenType::Number, buff)
    }    
    fn parse_name<I>(&self, first_char: char, iterator: &mut Peekable<I>) -> Token where I: Iterator<Item = Result<char, utf8_read::Error>>,{
        let mut buff = String::new();
        let mut letter = first_char;
        loop {
            buff.push(letter);
    
            match iterator.peek() {
                Some(Ok(c)) => {
                    if !((*c).is_ascii_alphabetic() || *c == '_' || (*c).is_ascii_digit()) {
                        break;
                    }
                }
                _ => break,
            }
            letter = iterator.next().unwrap().unwrap();
        }
        if let Some(token_type) = self.keywords.get(&buff) {
            Token::new(token_type.clone(), buff)
        } else {
            Token::new(TokenType::Variable, buff)
        }
    }
    fn parse_string<I>(iterator: &mut Peekable<I>) -> Token where I: Iterator<Item = Result<char, utf8_read::Error>>,{
        let mut buff = String::new();
        loop {
            let letter = match iterator.next() {
                Some(Ok('"')) => return Token::new(TokenType::String, buff),
                Some(Ok(x)) => x,
                Some(Err(_)) => panic!("Error while reading input"),
                None => panic!("Unexpected end of input"),
            };
            match letter {
                '"' => return Token::new(TokenType::String, buff),
                _ => buff.push(letter), // Continue reading characters if not a closing quote
            }
        }
    }
    
}


#[cfg(test)]
mod lexer_tests {
    use crate::token::TokenType;

    use super::*;
    
    #[test]
    fn test_parse_string() {
        let mut iterator = "hello\"".chars().map(|x|Ok(x)).into_iter().peekable();

        let token: Token = Lexer::parse_string(&mut iterator);
        assert_eq!(token.token_type(), TokenType::String);
        assert_eq!(token.val(), "hello");

        let mut iterator = "world1234\"".chars().map(|x|Ok(x)).into_iter().peekable();
        let token = Lexer::parse_string(&mut iterator);
        assert_eq!(token.token_type(), TokenType::String);
        assert_eq!(token.val(), "world1234");

        // Add more test cases for edge cases, invalid inputs, etc.
    }

    #[test]
    fn test_parse_name() {
        let mut iterator = "foo123".chars().map(|x|Ok(x)).into_iter().peekable();
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert("if".to_string(), TokenType::Keyword);
        let lex = Lexer::new_keywords(keywords);

        let token = lex.parse_name(iterator.next().unwrap().unwrap(), &mut iterator);
        assert_eq!(token.token_type(), TokenType::Variable);
        assert_eq!(token.val(), "foo123");

        let mut iterator = "if".chars().map(|x|Ok(x)).into_iter().peekable();
        let token = lex.parse_name(iterator.next().unwrap().unwrap(), &mut iterator);
        assert_eq!(token.token_type(), TokenType::Keyword);
        assert_eq!(token.val(), "if");

        // Add more test cases for edge cases, invalid inputs, etc.
    }
    
    #[test]
    fn test_parse_string_lexing() {
        let mut iterator = "\"gay\" \"\" \" not gonna sugarcoat it \"".chars().map(|x|Ok(x)).into_iter().peekable();
        let mut buffer = Vec::new();
        let lexer = Lexer::new();
        while let Some(x) = lexer.get_next_token(&mut iterator) {
            buffer.push(x);
        }
        assert_eq!(buffer[0].token_type(),TokenType::String);
        assert_eq!(buffer[0].val(),"gay");
        assert_eq!(buffer[1].token_type(),TokenType::String);
        assert_eq!(buffer[1].val(),"");
        assert_eq!(buffer[2].token_type(),TokenType::String);
        assert_eq!(buffer[2].val()," not gonna sugarcoat it ");
        // Add more test cases for edge cases, invalid inputs, etc.
    }

    #[test]
    fn test_parse_number() {
        let mut iterator = "123".chars().map(|x|Ok(x)).into_iter().peekable();
        let token = Lexer::parse_number(iterator.next().unwrap().unwrap(), &mut iterator);
        assert_eq!(token.token_type(), TokenType::Number);
        assert_eq!(token.val(), "123");

        let mut iterator = "0xABC".chars().map(|x|Ok(x)).into_iter().peekable();
        let token = Lexer::parse_number(iterator.next().unwrap().unwrap(), &mut iterator);
        assert_eq!(token.token_type(), TokenType::Number);
        assert_eq!(token.val(), "0xABC");

        // Add more test cases for edge cases, invalid inputs, etc.
    }
    
    #[test]
    fn test_lex_simple_expression() {
        
        let mut iterator = "1 + b - c".chars().map(|x|Ok(x)).into_iter().peekable();
        let mut buffer = Vec::new();
        let lexer = Lexer::new();
        while let Some(x) = lexer.get_next_token(&mut iterator) {
            buffer.push(x);
        }
        assert_eq!(buffer[0].token_type(), TokenType::Number);
        assert_eq!(buffer[0].val(), "1");
        assert_eq!(buffer[1].token_type(), TokenType::Operator);
        assert_eq!(buffer[1].val(), "+");
        assert_eq!(buffer[2].token_type(), TokenType::Variable);
        assert_eq!(buffer[2].val(), "b");
        assert_eq!(buffer[3].token_type(), TokenType::Operator);
        assert_eq!(buffer[3].val(), "-");
        assert_eq!(buffer[4].token_type(), TokenType::Variable);
        assert_eq!(buffer[4].val(), "c");

        // Add more test cases for edge cases, invalid inputs, etc.
    }
    #[test]
    fn test_lex_simple_expression_extended() {
        
        let mut iterator = "1 + base - con".chars().map(|x|Ok(x)).into_iter().peekable();
        let mut buffer = Vec::new();
        let lexer = Lexer::new();
        while let Some(x) = lexer.get_next_token(&mut iterator) {
            buffer.push(x);
        }
        assert_eq!(buffer[0].token_type(), TokenType::Number);
        assert_eq!(buffer[0].val(), "1");
        assert_eq!(buffer[1].token_type(), TokenType::Operator);
        assert_eq!(buffer[1].val(), "+");
        assert_eq!(buffer[2].token_type(), TokenType::Variable);
        assert_eq!(buffer[2].val(), "base");
        assert_eq!(buffer[3].token_type(), TokenType::Operator);
        assert_eq!(buffer[3].val(), "-");
        assert_eq!(buffer[4].token_type(), TokenType::Variable);
        assert_eq!(buffer[4].val(), "con");

        // Add more test cases for edge cases, invalid inputs, etc.
    }
}
