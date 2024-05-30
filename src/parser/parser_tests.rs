use std::collections::HashMap;
use crate::{lexer::Lexer, token::TokenType};

use super::*;

#[test]
fn test_parse_tree_simple_expression() {
  let mut iterator = "(1 - 2) + b * c".chars().map(|x|Ok(x)).into_iter().peekable();
  let mut buffer = Vec::new();
  while let Some(x) = {
    let iterator: &mut Peekable<I> = &mut iterator;
    let keywords = &HashMap::new();
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
            '^' | '%' | '*' | ',' | '.' => return Some(Token::new(TokenType::Operator, ch.to_string())),
            '+' | '-'  =>{
                if ch == p {
                    let mut a = ch.to_string();
                    a.push(ch);
                    iterator.next();
                    return Some(Token::new(TokenType::UnOperator, a))
                }
                return Some(Token::new(TokenType::Operator, ch.to_string()))
            }
            '(' => return Some(Token::new(TokenType::LParen,String::new())),
            ')' => return Some(Token::new(TokenType::RParen,String::new())),
            '{' => return Some(Token::new(TokenType::LBrace,String::new())),
            '}' => return Some(Token::new(TokenType::RBrace,String::new())),
            '[' => return Some(Token::new(TokenType::LSquareBrace,String::new())),
            ']' => return Some(Token::new(TokenType::RSquareBrace,String::new())),
            ';' => return Some(Token::new(TokenType::Semicolon,String::new())),
            ':' => return Some(Token::new(TokenType::Colon,String::new())),
            '~' => return Some(Token::new(TokenType::UnOperator,'~'.to_string())),
            '!' => {
                if p == '='{
                    let mut a = ch.to_string();
                    a.push(p);
                    iterator.next();
                    return Some(Token::new(TokenType::Operator, a))
                }
                return Some(Token::new(TokenType::UnOperator,'!'.to_string()))
            }
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
            '#' => return Some(Lexer::parse_name('#', iterator, keywords)),
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
                    return Some(Token::new(TokenType::Number, format!("{}",buff.chars().nth(0).unwrap() as u8)));
                }
                if buff.starts_with('\\') && buff.len() == 2{
                    let c = buff.pop().unwrap();
                    match c {
                        'n' => return Some(Token::new(TokenType::Number, 10.to_string())),
                        _ => panic!("Wrong escape sequence!"),
                    }
                }
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
} {
    buffer.push(x);
  }
  let mut root = TreeNode::new(Token::new(TokenType::Semicolon, String::new()));
  Parser::hit_tree(&mut buffer,&mut root);

  rec(&mut root);
  fn rec(parent: &mut TreeNode<Token>){
    let childrens: &mut Vec<TreeNode<Token>> = parent.children_mut();
    for child in childrens {
      if (*child).val().token_type() == TokenType::Operator{
        rec(child);
        (*child).set_val(Token::new(TokenType::Variable, "eax".to_string()));
      }
    }
    let mut a = String::new();
    for child in parent.children_mut() {
      a.push_str(child.val().val());
      a.push(' ');
    }
    a.push_str(parent.val().val());
    

    //println!("{}",a);
  }
  // Add more test cases for edge cases, invalid inputs, etc.
}

#[test]
fn test_parse_simple_expression() {
  let mut iterator = "(1-2) + b * c".chars().map(|x|Ok(x)).into_iter().peekable();
  let mut buffer = Vec::new();
  while let Some(x) = {
    let iterator: &mut Peekable<I> = &mut iterator;
    let keywords = &HashMap::new();
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
            '^' | '%' | '*' | ',' | '.' => return Some(Token::new(TokenType::Operator, ch.to_string())),
            '+' | '-'  =>{
                if ch == p {
                    let mut a = ch.to_string();
                    a.push(ch);
                    iterator.next();
                    return Some(Token::new(TokenType::UnOperator, a))
                }
                return Some(Token::new(TokenType::Operator, ch.to_string()))
            }
            '(' => return Some(Token::new(TokenType::LParen,String::new())),
            ')' => return Some(Token::new(TokenType::RParen,String::new())),
            '{' => return Some(Token::new(TokenType::LBrace,String::new())),
            '}' => return Some(Token::new(TokenType::RBrace,String::new())),
            '[' => return Some(Token::new(TokenType::LSquareBrace,String::new())),
            ']' => return Some(Token::new(TokenType::RSquareBrace,String::new())),
            ';' => return Some(Token::new(TokenType::Semicolon,String::new())),
            ':' => return Some(Token::new(TokenType::Colon,String::new())),
            '~' => return Some(Token::new(TokenType::UnOperator,'~'.to_string())),
            '!' => {
                if p == '='{
                    let mut a = ch.to_string();
                    a.push(p);
                    iterator.next();
                    return Some(Token::new(TokenType::Operator, a))
                }
                return Some(Token::new(TokenType::UnOperator,'!'.to_string()))
            }
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
            '#' => return Some(Lexer::parse_name('#', iterator, keywords)),
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
                    return Some(Token::new(TokenType::Number, format!("{}",buff.chars().nth(0).unwrap() as u8)));
                }
                if buff.starts_with('\\') && buff.len() == 2{
                    let c = buff.pop().unwrap();
                    match c {
                        'n' => return Some(Token::new(TokenType::Number, 10.to_string())),
                        _ => panic!("Wrong escape sequence!"),
                    }
                }
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
} {
    buffer.push(x);
  }
  Parser::hit(&mut buffer);
  assert_eq!(buffer[0].token_type(), TokenType::Number);
  assert_eq!(buffer[0].val(), "1");
  assert_eq!(buffer[1].token_type(), TokenType::Number);
  assert_eq!(buffer[1].val(), "2");
  assert_eq!(buffer[2].token_type(), TokenType::Operator);
  assert_eq!(buffer[2].val(), "-");
  assert_eq!(buffer[3].token_type(), TokenType::Variable);
  assert_eq!(buffer[3].val(), "b");
  assert_eq!(buffer[4].token_type(), TokenType::Variable);
  assert_eq!(buffer[4].val(), "c");
  assert_eq!(buffer[5].token_type(), TokenType::Operator);
  assert_eq!(buffer[5].val(), "*");
  assert_eq!(buffer[6].token_type(), TokenType::Operator);
  assert_eq!(buffer[6].val(), "+");
}
#[test]
fn test_parse_simple_expression_with_function() {
  let mut iterator = "a * pow(1,a) + b".chars().map(|x|Ok(x)).into_iter().peekable();
  let mut buffer = Vec::new();
  while let Some(x) = {
    let iterator: &mut Peekable<I> = &mut iterator;
    let keywords = &HashMap::new();
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
            '^' | '%' | '*' | ',' | '.' => return Some(Token::new(TokenType::Operator, ch.to_string())),
            '+' | '-'  =>{
                if ch == p {
                    let mut a = ch.to_string();
                    a.push(ch);
                    iterator.next();
                    return Some(Token::new(TokenType::UnOperator, a))
                }
                return Some(Token::new(TokenType::Operator, ch.to_string()))
            }
            '(' => return Some(Token::new(TokenType::LParen,String::new())),
            ')' => return Some(Token::new(TokenType::RParen,String::new())),
            '{' => return Some(Token::new(TokenType::LBrace,String::new())),
            '}' => return Some(Token::new(TokenType::RBrace,String::new())),
            '[' => return Some(Token::new(TokenType::LSquareBrace,String::new())),
            ']' => return Some(Token::new(TokenType::RSquareBrace,String::new())),
            ';' => return Some(Token::new(TokenType::Semicolon,String::new())),
            ':' => return Some(Token::new(TokenType::Colon,String::new())),
            '~' => return Some(Token::new(TokenType::UnOperator,'~'.to_string())),
            '!' => {
                if p == '='{
                    let mut a = ch.to_string();
                    a.push(p);
                    iterator.next();
                    return Some(Token::new(TokenType::Operator, a))
                }
                return Some(Token::new(TokenType::UnOperator,'!'.to_string()))
            }
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
            '#' => return Some(Lexer::parse_name('#', iterator, keywords)),
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
                    return Some(Token::new(TokenType::Number, format!("{}",buff.chars().nth(0).unwrap() as u8)));
                }
                if buff.starts_with('\\') && buff.len() == 2{
                    let c = buff.pop().unwrap();
                    match c {
                        'n' => return Some(Token::new(TokenType::Number, 10.to_string())),
                        _ => panic!("Wrong escape sequence!"),
                    }
                }
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
} {
    buffer.push(x);
  }
  Parser::hit(&mut buffer);
  let buffer = buffer;
  assert_eq!(buffer[0].token_type(), TokenType::Variable);
  assert_eq!(buffer[0].val(), "a");
  assert_eq!(buffer[1].token_type(), TokenType::Number);
  assert_eq!(buffer[1].val(), "1");
  assert_eq!(buffer[2].token_type(), TokenType::Variable);
  assert_eq!(buffer[2].val(), "a");
  assert_eq!(buffer[3].token_type(), TokenType::Operator);
  assert_eq!(buffer[3].val(), ",");
  assert_eq!(buffer[4].token_type(), TokenType::Keyword);
  assert_eq!(buffer[4].val(), "pow");
  assert_eq!(buffer[5].token_type(), TokenType::Operator);
  assert_eq!(buffer[5].val(), "*");
  assert_eq!(buffer[6].token_type(), TokenType::Variable);
  assert_eq!(buffer[6].val(), "b");
  assert_eq!(buffer[7].token_type(), TokenType::Operator);
  assert_eq!(buffer[7].val(), "+");
}
#[test]
fn test_parse_tree_simple_expression_with_function() {
  let mut iterator = "3 + pow(1) + b * c".chars().map(|x|Ok(x)).into_iter().peekable();
  let mut buffer = Vec::new();
  while let Some(x) = {
    let iterator: &mut Peekable<I>Lexer::parse_number
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
            '^' | '%' | '*' | ',' | '.' => return Some(Token::new(TokenType::Operator, ch.to_string())),
            '+' | '-'  =>{
                if ch == p {
                    let mut a = ch.to_string();
                    a.push(ch);
                    iterator.next();
                    return Some(Token::new(TokenType::UnOperator, a))
                }
                return Some(Token::new(TokenType::Operator, ch.to_string()))
            }
            '(' => return Some(Token::new(TokenType::LParen,String::new())),
            ')' => return Some(Token::new(TokenType::RParen,String::new())),
            '{' => return Some(Token::new(TokenType::LBrace,String::new())),
            '}' => return Some(Token::new(TokenType::RBrace,String::new())),
            '[' => return Some(Token::new(TokenType::LSquareBrace,String::new())),
            ']' => return Some(Token::new(TokenType::RSquareBrace,String::new())),
            ';' => return Some(Token::new(TokenType::Semicolon,String::new())),
            ':' => return Some(Token::new(TokenType::Colon,String::new())),
            '~' => return Some(Token::new(TokenType::UnOperator,'~'.to_string())),
            '!' => {
                if p == '='{
                    let mut a = ch.to_string();
                    a.push(p);
                    iterator.next();
                    return Some(Token::new(TokenType::Operator, a))
                }
                return Some(Token::new(TokenType::UnOperator,'!'.to_string()))
            }
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
            '#' => return Some(Lexer::parse_name('#', iterator, keywords)),
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
                    return Some(Token::new(TokenType::Number, format!("{}",buff.chars().nth(0).unwrap() as u8)));
                }
                if buff.starts_with('\\') && buff.len() == 2{
                    let c = buff.pop().unwrap();
                    match c {
                        'n' => return Some(Token::new(TokenType::Number, 10.to_string())),
                        _ => panic!("Wrong escape sequence!"),
                    }
                }
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
} {
    buffer.push(x);
  }
  let mut root = TreeNode::new(Token::new(TokenType::Semicolon, String::new()));
  Parser::hit_tree(&mut buffer,&mut root);

  rec(&mut root);
  fn rec(parent: &mut TreeNode<Token>){
    let childrens: &mut Vec<TreeNode<Token>> = parent.children_mut();
    for child in childrens {
      if (*child).val().token_type() == TokenType::Operator{
        rec(child);
        (*child).set_val(Token::new(TokenType::Variable, "eax".to_string()));
      }
    }
    let mut a = String::new();
    for child in parent.children_mut() {
      a.push_str(child.val().val());
      a.push(' ');
    }
    a.push_str(parent.val().val());
    

    //println!("{}",a);
  }
  // Add more test cases for edge cases, invalid inputs, etc.
}
#[test]
fn test_parse_c_function_declaration() {
  let mut iterator1 = "int (*pointer) = 0".chars().map(|x| Ok(x)).into_iter().peekable();
  let mut iterator2 = "(int*) pointer = 0".chars().map(|x| Ok(x)).into_iter().peekable();
  let mut iterator3 = "int* pointer = 0".chars().map(|x| Ok(x)).into_iter().peekable();
  let mut buffer1 = Vec::new();
  let mut buffer2 = Vec::new();
  let mut buffer3 = Vec::new();

  let mut keywords: HashMap<String, TokenType> = HashMap::new();
  keywords.insert("int".to_string(), TokenType::Keyword);
  
  
  while let Some(x) = {
    let iterator: &mut Peekable<I> = &mut iterator1;
    let keywords = &keywords;
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
            '^' | '%' | '*' | ',' | '.' => return Some(Token::new(TokenType::Operator, ch.to_string())),
            '+' | '-'  =>{
                if ch == p {
                    let mut a = ch.to_string();
                    a.push(ch);
                    iterator.next();
                    return Some(Token::new(TokenType::UnOperator, a))
                }
                return Some(Token::new(TokenType::Operator, ch.to_string()))
            }
            '(' => return Some(Token::new(TokenType::LParen,String::new())),
            ')' => return Some(Token::new(TokenType::RParen,String::new())),
            '{' => return Some(Token::new(TokenType::LBrace,String::new())),
            '}' => return Some(Token::new(TokenType::RBrace,String::new())),
            '[' => return Some(Token::new(TokenType::LSquareBrace,String::new())),
            ']' => return Some(Token::new(TokenType::RSquareBrace,String::new())),
            ';' => return Some(Token::new(TokenType::Semicolon,String::new())),
            ':' => return Some(Token::new(TokenType::Colon,String::new())),
            '~' => return Some(Token::new(TokenType::UnOperator,'~'.to_string())),
            '!' => {
                if p == '='{
                    let mut a = ch.to_string();
                    a.push(p);
                    iterator.next();
                    return Some(Token::new(TokenType::Operator, a))
                }
                return Some(Token::new(TokenType::UnOperator,'!'.to_string()))
            }
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
            '#' => return Some(Lexer::parse_name('#', iterator, keywords)),
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
                    return Some(Token::new(TokenType::Number, format!("{}",buff.chars().nth(0).unwrap() as u8)));
                }
                if buff.starts_with('\\') && buff.len() == 2{
                    let c = buff.pop().unwrap();
                    match c {
                        'n' => return Some(Token::new(TokenType::Number, 10.to_string())),
                        _ => panic!("Wrong escape sequence!"),
                    }
                }
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
} {
    buffer1.push(x);
  }
  
  while let Some(x) = {
    let iterator: &mut Peekable<I> = &mut iterator2;
    let keywords = &keywords;
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
            '^' | '%' | '*' | ',' | '.' => return Some(Token::new(TokenType::Operator, ch.to_string())),
            '+' | '-'  =>{
                if ch == p {
                    let mut a = ch.to_string();
                    a.push(ch);
                    iterator.next();
                    return Some(Token::new(TokenType::UnOperator, a))
                }
                return Some(Token::new(TokenType::Operator, ch.to_string()))
            }
            '(' => return Some(Token::new(TokenType::LParen,String::new())),
            ')' => return Some(Token::new(TokenType::RParen,String::new())),
            '{' => return Some(Token::new(TokenType::LBrace,String::new())),
            '}' => return Some(Token::new(TokenType::RBrace,String::new())),
            '[' => return Some(Token::new(TokenType::LSquareBrace,String::new())),
            ']' => return Some(Token::new(TokenType::RSquareBrace,String::new())),
            ';' => return Some(Token::new(TokenType::Semicolon,String::new())),
            ':' => return Some(Token::new(TokenType::Colon,String::new())),
            '~' => return Some(Token::new(TokenType::UnOperator,'~'.to_string())),
            '!' => {
                if p == '='{
                    let mut a = ch.to_string();
                    a.push(p);
                    iterator.next();
                    return Some(Token::new(TokenType::Operator, a))
                }
                return Some(Token::new(TokenType::UnOperator,'!'.to_string()))
            }
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
            '#' => return Some(Lexer::parse_name('#', iterator, keywords)),
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
                    return Some(Token::new(TokenType::Number, format!("{}",buff.chars().nth(0).unwrap() as u8)));
                }
                if buff.starts_with('\\') && buff.len() == 2{
                    let c = buff.pop().unwrap();
                    match c {
                        'n' => return Some(Token::new(TokenType::Number, 10.to_string())),
                        _ => panic!("Wrong escape sequence!"),
                    }
                }
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
} {
    buffer2.push(x);
  }
  
  while let Some(x) = {
    let iterator: &mut Peekable<I> = &mut iterator3;
    let keywords = &keywords;
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
            '^' | '%' | '*' | ',' | '.' => return Some(Token::new(TokenType::Operator, ch.to_string())),
            '+' | '-'  =>{
                if ch == p {
                    let mut a = ch.to_string();
                    a.push(ch);
                    iterator.next();
                    return Some(Token::new(TokenType::UnOperator, a))
                }
                return Some(Token::new(TokenType::Operator, ch.to_string()))
            }
            '(' => return Some(Token::new(TokenType::LParen,String::new())),
            ')' => return Some(Token::new(TokenType::RParen,String::new())),
            '{' => return Some(Token::new(TokenType::LBrace,String::new())),
            '}' => return Some(Token::new(TokenType::RBrace,String::new())),
            '[' => return Some(Token::new(TokenType::LSquareBrace,String::new())),
            ']' => return Some(Token::new(TokenType::RSquareBrace,String::new())),
            ';' => return Some(Token::new(TokenType::Semicolon,String::new())),
            ':' => return Some(Token::new(TokenType::Colon,String::new())),
            '~' => return Some(Token::new(TokenType::UnOperator,'~'.to_string())),
            '!' => {
                if p == '='{
                    let mut a = ch.to_string();
                    a.push(p);
                    iterator.next();
                    return Some(Token::new(TokenType::Operator, a))
                }
                return Some(Token::new(TokenType::UnOperator,'!'.to_string()))
            }
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
            '#' => return Some(Lexer::parse_name('#', iterator, keywords)),
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
                    return Some(Token::new(TokenType::Number, format!("{}",buff.chars().nth(0).unwrap() as u8)));
                }
                if buff.starts_with('\\') && buff.len() == 2{
                    let c = buff.pop().unwrap();
                    match c {
                        'n' => return Some(Token::new(TokenType::Number, 10.to_string())),
                        _ => panic!("Wrong escape sequence!"),
                    }
                }
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
} {
    buffer3.push(x);
  }
  Parser::hit(&mut buffer1);
  Parser::hit(&mut buffer2);
  Parser::hit(&mut buffer3);
  
  println!("");
}
#[test]
fn test_parse_simple_function() {
  let mut iterator = "int a = 13-41+(15+51-(32*9)); ".chars().map(|x|Ok(x)).into_iter().peekable();
  let mut keywords: HashMap<String, TokenType> = HashMap::new();
  keywords.insert("int".to_string(), TokenType::Keyword);
  let mut buffer = Vec::new();
  while let Some(x) = {
    let iterator: &mut Peekable<I> = &mut iterator;
    let keywords = &HashMap::new();
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
            '^' | '%' | '*' | ',' | '.' => return Some(Token::new(TokenType::Operator, ch.to_string())),
            '+' | '-'  =>{
                if ch == p {
                    let mut a = ch.to_string();
                    a.push(ch);
                    iterator.next();
                    return Some(Token::new(TokenType::UnOperator, a))
                }
                return Some(Token::new(TokenType::Operator, ch.to_string()))
            }
            '(' => return Some(Token::new(TokenType::LParen,String::new())),
            ')' => return Some(Token::new(TokenType::RParen,String::new())),
            '{' => return Some(Token::new(TokenType::LBrace,String::new())),
            '}' => return Some(Token::new(TokenType::RBrace,String::new())),
            '[' => return Some(Token::new(TokenType::LSquareBrace,String::new())),
            ']' => return Some(Token::new(TokenType::RSquareBrace,String::new())),
            ';' => return Some(Token::new(TokenType::Semicolon,String::new())),
            ':' => return Some(Token::new(TokenType::Colon,String::new())),
            '~' => return Some(Token::new(TokenType::UnOperator,'~'.to_string())),
            '!' => {
                if p == '='{
                    let mut a = ch.to_string();
                    a.push(p);
                    iterator.next();
                    return Some(Token::new(TokenType::Operator, a))
                }
                return Some(Token::new(TokenType::UnOperator,'!'.to_string()))
            }
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
            '#' => return Some(Lexer::parse_name('#', iterator, keywords)),
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
                    return Some(Token::new(TokenType::Number, format!("{}",buff.chars().nth(0).unwrap() as u8)));
                }
                if buff.starts_with('\\') && buff.len() == 2{
                    let c = buff.pop().unwrap();
                    match c {
                        'n' => return Some(Token::new(TokenType::Number, 10.to_string())),
                        _ => panic!("Wrong escape sequence!"),
                    }
                }
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
} {
    buffer.push(x);
  }
  Parser::hit(&mut buffer);
}
