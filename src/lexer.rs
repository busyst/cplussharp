use std::{collections::HashMap, iter::Peekable};
use crate::token::TokenType;
#[cfg(test)]
mod lexer_tests;

pub struct Lexer{}
impl Lexer {    
    pub fn get_next_token<I>(iterator: &mut Peekable<I>, keywords: &HashMap<String, TokenType>) -> Option<TokenType> where I: Iterator<Item = Result<char,utf8_read::Error>> {
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
                ',' | '.' | '(' | ')' | '{' | '}' |  '[' | ']' | ';' | ':' | '~' | '?'=> return Some(TokenType::Operator(ch.to_string())),
                '+' | '-' | '<' | '>' | '|' | '&'  =>{
                    if ch == p || p == '=' || (ch == '-' && p == '>')  {
                        let mut a = ch.to_string();
                        a.push(p);
                        iterator.next();
                        return Some(TokenType::Operator(a))
                    }
                    else if ch == '-'&& p.is_digit(10) {
                        return Some(Lexer::parse_number(ch, iterator));
                    }
                    return Some(TokenType::Operator(ch.to_string()))
                }
                '=' | '!'  | '*' | '%' | '^' => {
                    if p == '='{
                        let mut a = ch.to_string();
                        a.push(p);
                        iterator.next();
                        return Some(TokenType::Operator(a))
                    }
                    return Some(TokenType::Operator(ch.to_string()))
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
                    } else if p == '*' { // Skip multiline
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
                    else if p == '='{
                        let mut a = ch.to_string();
                        a.push(p);
                        iterator.next();
                        return Some(TokenType::Operator(a))
                    }
                    else {
                        return Some(TokenType::Operator(ch.to_string()))
                    }
                }
                '#' => return Some(TokenType::Preprocessor),
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
                        return Some(TokenType::Number(format!("{}",buff.chars().nth(0).unwrap() as u8)));
                    }
                    if buff.starts_with('\\') && buff.len() == 2{
                        let c = buff.pop().unwrap();
                        match c {
                            'n' => return Some(TokenType::Number(10.to_string())),
                            't' => return Some(TokenType::Number(9.to_string())),
                            'b' => return Some(TokenType::Number(8.to_string())),
                            'r' => return Some(TokenType::Number(13.to_string())),
                            'a' => return Some(TokenType::Number(7.to_string())),
                            '\'' => return Some(TokenType::Number(39.to_string())),
                            '"' => return Some(TokenType::Number(34.to_string())),
                            '\\' => return Some(TokenType::Number(92.to_string())),
                            'f' => return Some(TokenType::Number(12.to_string())),
                            'v' => return Some(TokenType::Number(11.to_string())),
                            '0' => return Some(TokenType::Number(0.to_string())),
                            _ => panic!("Wrong escape sequence!"),
                        }
                    }
                    todo!("Unimplemented escape sequence");
                }
                _ => {
                    // Handle numbers and identifiers
                    if ch.is_digit(10) {
                        return Some(Lexer::parse_number(ch, iterator));
                    } else if ch.is_ascii_alphabetic() {
                        return Some(Lexer::parse_name(ch, iterator, keywords));
                    } else {
                        println!("Error while lexing: {} | {} unexpected", ch, ch as u64);
                        return None;
                    }
                }
            }
        }
    }

    fn parse_number<I>(first_digit: char, iterator: &mut Peekable<I>) -> TokenType  where I: Iterator<Item = Result<char, utf8_read::Error>>{
        let mut digit = first_digit;
        let mut radix = 10;
        match iterator.peek() {
            None => return TokenType::Number(0.to_string()),
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
        let mut buff = String::new();
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
            
            return TokenType::Number(buff);
        }
        // If loop exits without returning, return a number token
        TokenType::Number(buff)
    }    
    fn parse_name<I>(first_char: char, iterator: &mut Peekable<I>, keywords: &HashMap<String, TokenType>) -> TokenType where I: Iterator<Item = Result<char, utf8_read::Error>>,{
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
        if let Some(token_type) = keywords.get(&buff) {
            token_type.clone()
        } else {
            TokenType::Identifier(buff)
        }
    }
    fn parse_string<I>(iterator: &mut Peekable<I>) -> TokenType where I: Iterator<Item = Result<char, utf8_read::Error>>,{
        let mut buff = String::new();
        loop {
            let letter = match iterator.next() {
                Some(Ok('"')) => return TokenType::String(buff),
                Some(Ok(x)) => x,
                Some(Err(_)) => panic!("Error while reading input"),
                None => panic!("Unexpected end of input"),
            };
            match letter {
                '"' => return TokenType::String(buff),
                _ => buff.push(letter), // Continue reading characters if not a closing quote
            }
        }
    }
    
}



