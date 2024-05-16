mod tree;
mod lexer;
mod token;
mod parser;

use utf8_read::Reader;
use std::fs::File;
use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::{Token, TokenType};
use crate::tree::TreeNode;
// Goal for now, implement C language.

fn main() {
  let filename = "main.cps".to_string();
  let mut keywords: HashMap<String, TokenType> = HashMap::new();
  // Default statements
  // Preprocess
  keywords.insert("#define".to_string(), TokenType::Keyword);
  // Condition
  keywords.insert("if".to_string(), TokenType::Keyword);
  keywords.insert("else".to_string(), TokenType::Keyword);
  // loops
  keywords.insert("while".to_string(), TokenType::Keyword);
  keywords.insert("for".to_string(), TokenType::Keyword);
  keywords.insert("do".to_string(), TokenType::Keyword);
  keywords.insert("continue".to_string(), TokenType::Keyword);
  keywords.insert("break;".to_string(), TokenType::Keyword);
  keywords.insert("return".to_string(), TokenType::Keyword);
  keywords.insert("goto".to_string(), TokenType::Keyword);
  // Structs
  keywords.insert("enum".to_string(), TokenType::Keyword);
  keywords.insert("struct".to_string(), TokenType::Keyword);
  keywords.insert("sizeof".to_string(), TokenType::Keyword);
  // """Match"""
  keywords.insert("switch".to_string(), TokenType::Keyword);
  keywords.insert("case".to_string(), TokenType::Keyword);
  // modifiers
  keywords.insert("unsigned".to_string(), TokenType::Keyword);
  keywords.insert("static".to_string(), TokenType::Keyword);
  keywords.insert("volatile".to_string(), TokenType::Keyword);

  // C#
  // classes
  keywords.insert("interface".to_string(), TokenType::Keyword);
  keywords.insert("class".to_string(), TokenType::Keyword);
  keywords.insert("volatile".to_string(), TokenType::Keyword);
  keywords.insert("abstract".to_string(), TokenType::Keyword);
  keywords.insert("this".to_string(), TokenType::Keyword);
  // type
  keywords.insert("typeof".to_string(), TokenType::Keyword); 
  // error
  keywords.insert("throw".to_string(), TokenType::Keyword);
  // try
  keywords.insert("try".to_string(), TokenType::Keyword);
  keywords.insert("catch".to_string(), TokenType::Keyword);
  keywords.insert("finally".to_string(), TokenType::Keyword);

  // Default data sizes
  // -------------------
  // *void: depends on system 
  // byte/ubyte: 8 bits      // signed byte is not commonly used
  // short/ushort: 16 bits
  // char/uchar: 16 bits      // signed char is rarely used; imagine having '\0' represented as -16k
  // int/uint: 32 bits
  // float: 32 bits
  // long/ulong: 64 bits
  // double: 64 bits
  // ------------------- 
  // By default, sizes will be consistent with C# but can be modified using preprocessor directives if needed.
  keywords.insert("void".to_string(), TokenType::Keyword);
  keywords.insert("byte".to_string(), TokenType::Keyword);
  keywords.insert("char".to_string(), TokenType::Keyword);
  keywords.insert("short".to_string(), TokenType::Keyword);
  keywords.insert("ushort".to_string(), TokenType::Keyword);
  keywords.insert("int".to_string(), TokenType::Keyword);
  keywords.insert("float".to_string(), TokenType::Keyword);
  keywords.insert("uint".to_string(), TokenType::Keyword);
  keywords.insert("long".to_string(), TokenType::Keyword);
  keywords.insert("ulong".to_string(), TokenType::Keyword);
  keywords.insert("double".to_string(), TokenType::Keyword);


  let mut parser = Parser::new();
  
  let mut file= File::open(filename).expect("Error opening file");
  let mut reader= Reader::new(&mut file); // Use the correct reference
  let mut iterator  = reader.into_iter().peekable();
  while let Some(x) = Lexer::get_next_token(&mut iterator,&keywords) {
    parser.add_to_buffer(x);
  }
  let mut root = TreeNode::new(Token::new(TokenType::Semicolon, String::new()));
  parser.hit_tree(&mut root);

  println!();
}




