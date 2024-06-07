use super::*;

#[test]
fn test_parse_string() {
    let mut iterator = "hello\\\"".chars().map(|x| Ok(x)).into_iter().peekable();
    let token: TokenType = Lexer::parse_string(&mut iterator);
    assert_eq!(token, TokenType::String("hello".to_string()));

    let mut iterator = "world1234\\\"".chars().map(|x| Ok(x)).into_iter().peekable();
    let token = Lexer::parse_string(&mut iterator);
    assert_eq!(token, TokenType::String("world1234".to_string()));
}

#[test]
fn test_parse_name() {
    let mut iterator = "foo123 if".chars().map(|x| Ok(x)).into_iter().peekable();
    let mut keywords: HashMap<String, TokenType> = HashMap::new();
    keywords.insert("if".to_string(), TokenType::Keyword("if".to_string()));
    let token = Lexer::parse_name(iterator.next().unwrap().unwrap(), &mut iterator, &keywords);
    assert_eq!(token, TokenType::Identifier("foo123".to_string()));

    let mut iterator = "if".chars().map(|x| Ok(x)).into_iter().peekable();
    let token = Lexer::parse_name(iterator.next().unwrap().unwrap(), &mut iterator, &keywords);
    assert_eq!(token, TokenType::Keyword("if".to_string()));
}
#[test]
fn test_parse_string_lexing() {
    let mut iterator = "\"gay\" \"\" \" not gonna sugarcoat it \"".chars().map(|x| Ok(x)).into_iter().peekable();
    let mut buffer = Vec::new();
    let keywords: HashMap<String, TokenType> = HashMap::new();
    while let Some(x) = Lexer::get_next_token(&mut iterator, &keywords) {
        buffer.push(x);
    }
    assert_eq!(buffer[0], TokenType::String("gay".to_string()));
    assert_eq!(buffer[1], TokenType::String("".to_string()));
    assert_eq!(buffer[2], TokenType::String(" not gonna sugarcoat it ".to_string()));
}

#[test]
fn test_parse_number() {
    let mut iterator = "123".chars().map(|x| Ok(x)).into_iter().peekable();
    let kw = HashMap::new();
    let token = Lexer::get_next_token(&mut iterator, &kw).unwrap();
    assert_eq!(token, TokenType::Number("123".to_string()));

    let mut iterator = "0xABC".chars().map(|x| Ok(x)).into_iter().peekable();
    let token = Lexer::get_next_token(&mut iterator, &kw).unwrap();
    assert_eq!(token, TokenType::Number("0xABC".to_string()));

    let mut iterator = "0".chars().map(|x| Ok(x)).into_iter().peekable();
    let token = Lexer::get_next_token(&mut iterator, &kw).unwrap();
    assert_eq!(token, TokenType::Number("0".to_string()));

    let mut iterator = "-8".chars().map(|x| Ok(x)).into_iter().peekable();
    let token = Lexer::get_next_token(&mut iterator, &kw).unwrap();
    assert_eq!(token, TokenType::Number("-8".to_string()));
}

#[test]
fn test_lex_simple_expression() {
    let mut iterator = "1 + b - c".chars().map(|x| Ok(x)).into_iter().peekable();
    let mut buffer = Vec::new();
    let keywords: HashMap<String, TokenType> = HashMap::new();
    while let Some(x) = Lexer::get_next_token(&mut iterator, &keywords) {
        buffer.push(x);
    }
    assert_eq!(buffer[0], TokenType::Number("1".to_string()));
    assert_eq!(buffer[1], TokenType::Operator("+".to_string()));
    assert_eq!(buffer[2], TokenType::Identifier("b".to_string()));
    assert_eq!(buffer[3], TokenType::Operator("-".to_string()));
    assert_eq!(buffer[4], TokenType::Identifier("c".to_string()));
}

#[test]
fn test_lex_simple_expression_extended() {
    let mut iterator = "1 + base - con".chars().map(|x| Ok(x)).into_iter().peekable();
    let mut buffer = Vec::new();
    let keywords: HashMap<String, TokenType> = HashMap::new();
    while let Some(x) = Lexer::get_next_token(&mut iterator, &keywords) {
        buffer.push(x);
    }
    assert_eq!(buffer[0], TokenType::Number("1".to_string()));
    assert_eq!(buffer[1], TokenType::Operator("+".to_string()));
    assert_eq!(buffer[2], TokenType::Identifier("base".to_string()));
    assert_eq!(buffer[3], TokenType::Operator("-".to_string()));
    assert_eq!(buffer[4], TokenType::Identifier("con".to_string()));
}

#[test]
fn test_lex_operators() {
    let mut iterator = " + - * / % ++ -- = += -= *= /= %= == != > < >= <= << >> &= |= ^= & | ^ ~ ! && || ? : , ; { } ( ) [ ] . ->".chars().map(|x| Ok(x)).into_iter().peekable();
    let mut buffer = Vec::new();
    let keywords: HashMap<String, TokenType> = HashMap::new();
    while let Some(x) = Lexer::get_next_token(&mut iterator, &keywords) {
        buffer.push(x);
    }
    assert_eq!(buffer.len(), 43);
    for x in buffer {
        assert_eq!(x, TokenType::Operator(x.to_string()));
    }
}