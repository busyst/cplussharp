mod tree;
mod lexer;
mod token;
mod parser;


use utf8_read::Reader;
use std::fs::File;
use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::TokenType;
fn main() {
  let filename = "main.cps".to_string();
  let mut keywords: HashMap<String, TokenType> = HashMap::new();
  keywords.insert("if".to_string(), TokenType::Keyword);
  keywords.insert("return".to_string(), TokenType::Keyword);
  keywords.insert("unsigned".to_string(), TokenType::Keyword);
  keywords.insert("void".to_string(), TokenType::Keyword);
  keywords.insert("#define".to_string(), TokenType::Keyword);

  keywords.insert("ulong".to_string(), TokenType::Keyword);
  keywords.insert("long".to_string(), TokenType::Keyword);
  keywords.insert("uint".to_string(), TokenType::Keyword);
  keywords.insert("int".to_string(), TokenType::Keyword);
  keywords.insert("ushort".to_string(), TokenType::Keyword);
  keywords.insert("short".to_string(), TokenType::Keyword);
  keywords.insert("byte".to_string(), TokenType::Keyword);

  let lexer = Lexer::new_keywords(keywords);

  let mut parser = Parser::new();
  
  let mut file= File::open(filename).expect("Error opening file");
  let mut reader= Reader::new(&mut file); // Use the correct reference
  let mut iterator  = reader.into_iter().peekable();
  while let Some(x) = lexer.get_next_token(&mut iterator) {
    parser.add_to_buffer(x);
    parser.update();
  }

  println!();
}




