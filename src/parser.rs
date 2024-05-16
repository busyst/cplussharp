
use token::Token;
use tree::TreeNode;

use crate::{token::{self, TokenType}, tree};
pub struct Parser {
    buffer : Vec<Token>,
    counter : u8,
  }
  
impl Parser {
    pub fn new() -> Self {
        let buffer: Vec<Token> = Vec::new();
        
        Self {
        buffer,
        counter: 0,
        }
    }
    pub fn add_to_buffer(&mut self, token: Token) {
      self.buffer.push(token);
    }
    
    pub fn update(&mut self) {
      let l: &Token = self.buffer.last().unwrap();  
      if matches!(l.token_type(),TokenType::LBrace){
        self.counter += 1;
        return;
      }
      if matches!(l.token_type(),TokenType::RBrace){
        if self.counter == 0{
          panic!("Mismatched parentheses, update type!")
        }
        self.counter -= 1;
        if self.counter==0{
          self.hit();
        }
        return;
      }
      if(matches!(l.token_type(),TokenType::Semicolon)&&self.counter==0){
        self.hit();
        return;
      }
    }
    pub fn hit(&mut self){
      self.preprocess();
  
      self.rpn_buffer();
    }

    pub fn hit_tree(&mut self,root: &mut TreeNode<Token>){
      self.preprocess();
  
      self.rpn_buffer();
      self.make_tree(root);
      for x in &self.buffer {
        print!("{}", x);
      }
      println!();
      self.buffer.clear();
    }
    fn preprocess(&mut self) {
      let mut i: usize = 0;
      let mut l: usize = self.buffer.len();
      let buffer_mut = &mut self.buffer;
  
      while i < l {
        let a = buffer_mut.get(i).unwrap();

        if (l - i) >= 2 {
          if a.token_type() == TokenType::Keyword && a.val() == "unsigned" {
            let b = buffer_mut.get_mut(i + 1).unwrap();
            if b.val() == "int" || b.val() == "byte" || b.val() == "short" || b.val() == "long" || b.val() == "char" {
              buffer_mut.get_mut(i + 1).unwrap().val_mut().insert(0, 'u');
              buffer_mut.remove(i);
              l -= 1;
              continue;
            }
            panic!("[unsigned] (byte/short/int/long/char) unexpected error!");
          } else if a.token_type() == TokenType::Variable {
            let b = buffer_mut.get(i + 1).unwrap();
            if b.token_type() == TokenType::LParen {
              buffer_mut.get_mut(i).unwrap().set_token_type(TokenType::Keyword);
              continue;
            }
          }
          if a.token_type() == TokenType::Keyword{
            let b = buffer_mut.get(i + 1).unwrap();
            if b.token_type() == TokenType::Operator &&  b.val() == "*" {
              buffer_mut.get_mut(i + 1).unwrap().set_token_type(TokenType::UnOperator);
              continue;
            }
          }
          if i!=0 && a.token_type() == TokenType::Operator && a.val() == "," {
            let b = buffer_mut.get(i + 1).unwrap();
            if b.token_type() == TokenType::RParen{
              buffer_mut.remove(i);
              continue;
            }
          }
        }
        if i!=0 && a.token_type() == TokenType::Operator && (l - i) >= 1  &&  (a.val() == "*" || a.val() == "+" || a.val() == "-") {
          let b = buffer_mut.get(i - 1).unwrap();
          if b.token_type() != TokenType::Variable && b.token_type() != TokenType::Number && b.token_type() != TokenType::RParen {
            buffer_mut.get_mut(i).unwrap().set_token_type(TokenType::UnOperator);
            continue;
          }
        }
        // TODO: add other define types
        if a.token_type() == TokenType::Keyword && a.val() == "#define" {
          println!("{} {} {}",buffer_mut[i].val(),buffer_mut[i+1].val(),buffer_mut[i+2].val());
          buffer_mut.remove(i+2);
          buffer_mut.remove(i+1);
          buffer_mut.remove(i);
          l -= 3;
          continue;
        }
  
        i += 1;
      }
    }
  
  
    fn make_tree(&mut self,root: &mut TreeNode<Token>) {
      if self.buffer.len() == 0 {
        panic!("Tree with length 0?")
      }
      
      let last_token = self.buffer.pop().unwrap();
      let root_a = last_token.to_tree_node_clone();
      root.set_val((*root_a.val()).clone());

      fn rec(buffer : &mut Vec<Token>,curr: &mut TreeNode<Token>) {
        loop {
          match buffer.pop() {
            Some(tok) => match tok.token_type() {
              TokenType::Operator | TokenType::UnOperator | TokenType::Keyword => {
                curr.add_child(tok.to_tree_node_clone());
                rec(buffer, &mut curr.children_mut().last_mut().unwrap());
              },
              _ =>{
                curr.add_child(tok.to_tree_node_clone());
                if curr.children().len() >=2 || curr.val().token_type() == TokenType::Keyword || curr.val().token_type() == TokenType::UnOperator {
                  return;
                }
              },
            },
            None => return, // Buffer is empty, exit the loop
          }
          if curr.children().is_empty() {
            return; // Current node has no children, exit the loop
          }
        }
      }
      rec(&mut self.buffer,root);
      
      println!("------>");
      println!("{}",root);
      println!("------<");
    }

    /// Reverse prefix! Not postfix, and not prefix
    fn rpn_buffer(&mut self) {
      let mut output: Vec<Token> = Vec::new();
      let mut stack: Vec<Token> = Vec::new();
      while !self.buffer.is_empty() {
        let e = self.buffer.remove(0);
        match e.token_type() {
          TokenType::Keyword | TokenType::LParen | TokenType::LBrace | TokenType::LSquareBrace => stack.push(e),
          TokenType::RParen | TokenType::RBrace | TokenType::RSquareBrace => {
            let left = (e.token_type() as u8) - 1;
            if let Some(last) = stack.last() {
              if last.token_type() as u8 == left {
                stack.pop();
                continue;
              }
            }
            while let Some(top) = stack.pop() {
              if top.token_type() as u8 == left{
                break;
              }
              output.push(top);
            }
            if let Some(last) = stack.last() {
              if last.token_type() == TokenType::Keyword {
                output.push(stack.pop().unwrap());
              }
            }
          },
        TokenType::UnOperator | TokenType::Operator  => {
            let priority = Parser::precedence(&e.val(), &e.token_type());
            while let Some(top) = stack.last() {
              if !matches!(&top.token_type() , TokenType::LParen){
                let top_p = Parser::precedence(&top.val(), &top.token_type());
                if top_p < priority{
                  output.push(stack.pop().unwrap());
                }
                else {
                  break;
                }
              } else {
                break;
              }
          }
          stack.push(e);
        },
        TokenType::Variable | TokenType::Number | TokenType::String | TokenType::Pointer  =>{
          output.push(e);
        }
        TokenType::Semicolon =>{}
        _ => panic!("{}",e),
      }
    }
    while let Some(top) = stack.pop() {
      if top.token_type() == TokenType::LParen||top.token_type() == TokenType::LBrace||top.token_type() == TokenType::LSquareBrace {
        panic!("Mismatched parentheses or tmt {}",top);
      }
      output.push(top);
    }
      self.buffer = output;
    }
  
  
    fn precedence(s: &str, t: &TokenType) -> u8 {
      match s {
          "." | "->" | "++" | "--" => 1,
          "*" | "/" | "%" => 2,
          "+" | "-" => 3,
          "<<" | ">>" => 4,
          "<" | "<=" | ">" | ">=" => 6,
          "==" | "!=" => 7,
          "&" => 8,
          "^" => 9,
          "|" => 10,
          "&&" => 11,
          "||" => 12,
          "=" => 13,
          "," => 14,
          "?" => 15,
          _ => match t {
            TokenType::UnOperator => 0,
            TokenType::Colon => 16,
            TokenType::LBrace => 17,
            TokenType::RBrace => 18,
            TokenType::LSquareBrace => 19,
            TokenType::RSquareBrace => 20,
            TokenType::LParen => 21,
            TokenType::RParen => 22,
            TokenType::Keyword => 23,
            _ => panic!("{:?}",t),
          },
      }
  }
  
  
}


#[cfg(test)]
mod parser_tests {
    use std::collections::HashMap;

    use crate::{lexer::Lexer, token::TokenType};

    use super::*;
    #[test]
    fn test_parse_tree_simple_expression() {
      let mut iterator = "(1-2) + b * c".chars().map(|x|Ok(x)).into_iter().peekable();
      let mut parser = Parser::new();
      while let Some(x) = Lexer::get_next_token(&mut iterator,&HashMap::new()) {
        parser.add_to_buffer(x);
      }
      let mut root = TreeNode::new(Token::new(TokenType::Semicolon, String::new()));
      parser.hit_tree(&mut root);

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
      let mut parser = Parser::new();
      while let Some(x) = Lexer::get_next_token(&mut iterator,&HashMap::new()) {
        parser.add_to_buffer(x);
      }
      parser.hit();
      let buffer = parser.buffer;
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
      // Add more test cases for edge cases, invalid inputs, etc.
    }
    #[test]
    fn test_parse_simple_expression_with_function() {
      let mut iterator = "a * pow(1,a) + b".chars().map(|x|Ok(x)).into_iter().peekable();
      let mut parser = Parser::new();
      while let Some(x) = Lexer::get_next_token(&mut iterator,&HashMap::new()) {
        parser.add_to_buffer(x);
      }
      parser.hit();
      let buffer =  parser.buffer;
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
      // Add more test cases for edge cases, invalid inputs, etc.
    }
    #[test]
    fn test_parse_tree_simple_expression_with_function() {
      let mut iterator = "3 + pow(1) + b * c".chars().map(|x|Ok(x)).into_iter().peekable();
      let mut parser = Parser::new();
      while let Some(x) = Lexer::get_next_token(&mut iterator,&HashMap::new()) {
        parser.add_to_buffer(x);
      }
      let mut root = TreeNode::new(Token::new(TokenType::Semicolon, String::new()));
      parser.hit_tree(&mut root);

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
      let mut iterator1 = "int *(*pointer) = 0".chars().map(|x| Ok(x)).into_iter().peekable();
      let mut iterator2 = "(int*)* pointer = 0".chars().map(|x| Ok(x)).into_iter().peekable();
      let mut iterator3 = "int** pointer = 0".chars().map(|x| Ok(x)).into_iter().peekable();
      let mut parser1 = Parser::new();
      let mut parser2 = Parser::new();
      let mut parser3 = Parser::new();

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
      keywords.insert("char".to_string(), TokenType::Keyword);
      
      
      while let Some(x) = Lexer::get_next_token(&mut iterator1,&keywords) {
          parser1.add_to_buffer(x);
      }
  
      while let Some(x) = Lexer::get_next_token(&mut iterator2,&keywords) {
          parser2.add_to_buffer(x);
      }
  
      while let Some(x) = Lexer::get_next_token(&mut iterator3,&keywords) {
          parser3.add_to_buffer(x);
      }
      parser1.hit();
      parser2.hit();
      parser2.preprocess();
      parser3.hit();
  
      println!("");
  }
  #[test]
  fn test_parse_simple_function() {
    let mut iterator = "int a = 0; ".chars().map(|x|Ok(x)).into_iter().peekable();
    let mut parser = Parser::new();
    let mut keywords: HashMap<String, TokenType> = HashMap::new();
    keywords.insert("int".to_string(), TokenType::Keyword);
    while let Some(x) = Lexer::get_next_token(&mut iterator,&keywords) {
      parser.add_to_buffer(x);
    }
    parser.hit();
    // Add more test cases for edge cases, invalid inputs, etc.
  }
  
}
