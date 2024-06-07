mod tree;
mod lexer;
mod parser;
mod token;
use parser::Parser;
use utf8_read::Reader;
use std::fs::File;
use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::token::TokenType;

// Goal for now, implement language.

fn main() {
  let filename = "main.cps".to_string();
  let mut keywords: HashMap<String, TokenType> = HashMap::new();
  // Default statements
  // Preprocess
  keywords.insert("#define".to_string(), TokenType::Preprocessor);
  // Condition
  keywords.insert("if".to_string(), TokenType::Keyword("if".to_string()));
  keywords.insert("else".to_string(), TokenType::Keyword("else".to_string()));
  // loops
  keywords.insert("while".to_string(), TokenType::Keyword("while".to_string()));
  keywords.insert("for".to_string(), TokenType::Keyword("for".to_string()));
  keywords.insert("do".to_string(), TokenType::Keyword("do".to_string()));
  keywords.insert("continue".to_string(), TokenType::Keyword("continue".to_string()));
  keywords.insert("break".to_string(), TokenType::Keyword("break".to_string()));
  keywords.insert("return".to_string(), TokenType::Keyword("return".to_string()));
  keywords.insert("goto".to_string(), TokenType::Keyword("goto".to_string()));
  // Structs
  keywords.insert("enum".to_string(), TokenType::Keyword("enum".to_string()));
  keywords.insert("struct".to_string(), TokenType::Keyword("struct".to_string()));
  keywords.insert("sizeof".to_string(), TokenType::Keyword("sizeof".to_string()));
  // """Match"""
  keywords.insert("switch".to_string(), TokenType::Keyword("swich".to_string()));
  keywords.insert("case".to_string(), TokenType::Keyword("case".to_string()));
  // modifiers
  keywords.insert("unsigned".to_string(), TokenType::Keyword("unsigned".to_string()));
  keywords.insert("static".to_string(), TokenType::Keyword("static".to_string()));
  keywords.insert("volatile".to_string(), TokenType::Keyword("volatile".to_string()));

  // C#
  // classes
  keywords.insert("interface".to_string(), TokenType::Keyword("inteface".to_string()));
  keywords.insert("class".to_string(), TokenType::Keyword("class".to_string()));
  keywords.insert("volatile".to_string(), TokenType::Keyword("volatile".to_string()));
  keywords.insert("abstract".to_string(), TokenType::Keyword("absctract".to_string()));
  keywords.insert("this".to_string(), TokenType::Keyword("this".to_string()));
  // type
  keywords.insert("typeof".to_string(), TokenType::Keyword("typeof".to_string())); 
  // error
  keywords.insert("throw".to_string(), TokenType::Keyword("throw".to_string()));
  // try
  keywords.insert("try".to_string(), TokenType::Keyword("try".to_string()));
  keywords.insert("catch".to_string(), TokenType::Keyword("catch".to_string()));
  keywords.insert("finally".to_string(), TokenType::Keyword("finaly".to_string()));

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
  keywords.insert("void".to_string(), TokenType::Keyword("void".to_string()));
  keywords.insert("byte".to_string(), TokenType::Keyword("byte".to_string()));
  keywords.insert("sbyte".to_string(), TokenType::Keyword("sbyte".to_string()));

  keywords.insert("char".to_string(), TokenType::Keyword("char".to_string()));
  keywords.insert("uchar".to_string(), TokenType::Keyword("uchar".to_string()));

  keywords.insert("short".to_string(), TokenType::Keyword("short".to_string()));
  keywords.insert("ushort".to_string(), TokenType::Keyword("ushort".to_string()));

  keywords.insert("int".to_string(), TokenType::Keyword("int".to_string()));
  keywords.insert("uint".to_string(), TokenType::Keyword("uint".to_string()));

  keywords.insert("long".to_string(), TokenType::Keyword("long".to_string()));
  keywords.insert("ulong".to_string(), TokenType::Keyword("ulong".to_string()));

  keywords.insert("float".to_string(), TokenType::Keyword("float".to_string()));
  keywords.insert("double".to_string(), TokenType::Keyword("double".to_string()));

  
  let mut file= File::open(filename).expect("Error opening file");
  let mut reader= Reader::new(&mut file); // Use the correct reference
  let mut iterator  = reader.into_iter().peekable();
  let mut buffer = Vec::new();
  while let Some(x) = Lexer::get_next_token(&mut iterator,&keywords) {
    buffer.push(x);
  }
  let mut parser = Parser::new(buffer);
  let mut instructions = parser.parse();
  /*
  
  
  let mut root = TreeNode::new(Token::new(TokenType::Semicolon, String::new()));
  parser.hit_tree(&mut root);

  println!();
   */
}




