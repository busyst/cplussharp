
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
    fn preprocess(&mut self){
      let mut i = 0;
      loop {
        let l = self.buffer.len();
        if l == i{
          break;
        }
        if (l - i) >= 2{
          let a = self.buffer.get_mut(i).unwrap();
          if a.token_type() == TokenType::Keyword &&a.val()=="unsigned"{
            let b = self.buffer.get_mut(i + 1).unwrap();
            if b.val()=="int"||b.val()=="byte"||b.val()=="short"||b.val()=="long"  {
              b.val_mut().insert(0, 'u');
              self.buffer.remove(i);
              continue;
            }
            panic!("[unsigned] (byte/short/int/long) unexpected error!");
          }
          
  
  
        }
        i+=1;
      }
      println!();
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
              TokenType::Operator | TokenType::UnOperator => {
                curr.add_child(tok.to_tree_node_clone());
                rec(buffer, &mut curr.last_mut());
              },
              _ =>{
                curr.add_child(tok.to_tree_node_clone());
                if curr.children().len() >=2{
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

    /// Reverse prefix! Not postfix
    fn rpn_buffer(&mut self) {
      let mut output: Vec<Token> = Vec::new();
      let mut stack: Vec<Token> = Vec::new();
      while !self.buffer.is_empty() {
        let e = self.buffer.remove(0);
        match e.token_type() {
          TokenType::Keyword | TokenType::LParen | TokenType::LBrace | TokenType::LSquareBrace => stack.push(e),
          TokenType::RParen | TokenType::RBrace | TokenType::RSquareBrace => {
            let left = (e.token_type() as u8) - 1;
            if Some(stack.last()).unwrap().unwrap().token_type() as u8 == left{
                stack.pop();
                continue;
            }
            while let Some(top) = stack.pop() {
              if top.token_type() as u8 == left{
                break;
              }
              output.push(top);
            }
        },
        TokenType::UnOperator | TokenType::Operator => {
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
      TokenType::Variable | TokenType::Number | TokenType::Char | TokenType::String  =>{
        output.push(e);
      }
      TokenType::Semicolon => {
        output.push(e);
      }
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
          "." | "++" | "--" => 1,
  
          "*" | "/" | "%" => 3,
          "+" | "-" => 4,
          ">>"| "<<"  => 5,
          "<"| "<=" | ">=" | ">"  => 6,
          "=="| "!=" => 7,
          "&" => 8,
          "^" => 9,
          "|" => 10,
          "&&" => 11,
          "||" => 12,
          "=" => 14,
          "," => 15,
          _ => match t {
              TokenType::UnOperator => 1,
              TokenType::LBrace => 16,
              TokenType::LSquareBrace => 15,
              TokenType::LParen => 15,
              TokenType::Keyword => 0,
              _ => panic!("Unexpected token {},{}", t, *t as i8),
          },
      }
    }
}


#[cfg(test)]
mod parser_tests {
    use crate::{lexer::Lexer, token::TokenType};

    use super::*;
    #[test]
    fn test_parse_tree_simple_expression() {
      let mut iterator = "(1-2) + b * c".chars().map(|x|Ok(x)).into_iter().peekable();
      let mut parser = Parser::new();
      let lexer = Lexer::new();
      while let Some(x) = lexer.get_next_token(&mut iterator) {
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
      let lexer = Lexer::new();
      while let Some(x) = lexer.get_next_token(&mut iterator) {
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
}
