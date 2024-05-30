use crate::token::TokenType;
#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
  // Preprocessor, Name , definition
  PreDefineConstant(String,Vec<TokenType>),
  PreDefineExpression(String,Vec<TokenType>),
  PreDefineExpressionWithParams(String,Vec<TokenType>,Vec<TokenType>),
  // left, right
  BitField(Vec<TokenType>,Vec<TokenType>),
  // label name
  Label(String),
  // declaration (before '{'), body
  StructureDefine(Vec<TokenType>, Vec<ASTNode>),
  // function and args
  FunctionCall(Vec<TokenType>),
  // (-= += = *= ...), left and right parts
  Operation(TokenType, Vec<TokenType>, Vec<TokenType>),
  // (++ --), left part
  UnaryOperation(TokenType, Vec<TokenType>),
  
}

use std::fmt;

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
          ASTNode::PreDefineExpression(_,_) =>{Ok(())}
          ASTNode::PreDefineExpressionWithParams(_,_,_) =>{Ok(())}

            ASTNode::PreDefineConstant(name, tokens) => {
                write!(f, "PreDefineMacros({},[", name)?;
                for token in tokens {
                    write!(f, "{},", token)?;
                }
                write!(f, "])")
            }
            ASTNode::BitField(left, right) => {
                write!(f, "BitField([")?;
                for token in left {
                    write!(f, "{},", token)?;
                }
                write!(f, "],[")?;
                for token in right {
                    write!(f, "{},", token)?;
                }
                write!(f, "])")
            }
            ASTNode::Label(name) => write!(f, "Label({})", name),
            ASTNode::StructureDefine(declaration, body) => {
                write!(f, "StructureDefine([")?;
                for token in declaration {
                    write!(f, "{},", token)?;
                }
                writeln!(f, "],[")?;
                for node in body {
                    writeln!(f, "{},", node)?;
                }
                write!(f, "])")
            }
            ASTNode::FunctionCall(tokens) => {
                write!(f, "FunctionCall([")?;
                for token in tokens {
                    write!(f, "{}", token)?;
                }
                write!(f, "])")
            }
            ASTNode::Operation(op, left, right) => {
                write!(f, "Operation({},[",op)?;
                for token in left {
                    write!(f, "{},", token)?;
                }
                write!(f, "],[")?;
                for token in right {
                    write!(f, "{},", token)?;
                }
                write!(f, "])")
            }
            ASTNode::UnaryOperation(op, tokens) => {
                write!(f, "UnaryOperation({},[",op)?;
                for token in tokens {
                    write!(f, "{},", token)?;
                }
                write!(f, "])")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
  tokens: Vec<TokenType>,
}

impl Parser {
  pub fn new(tokens: Vec<TokenType>) -> Self {
    Self { tokens }
  }
  pub fn parse(&mut self) -> Vec<ASTNode>{
    let mut ast = Vec::new();

    let mut i = 0;
    let mut parens = 0;
    while let Some(token) = self.tokens.get(i) {

      if let TokenType::Keyword(op) = token{
        if op == "unsigned"{
          if let TokenType::Keyword(sop) = self.tokens.get_mut(i+1).unwrap(){
            sop.insert(0, 'u');
          }
          self.tokens.remove(i);
        }
      }
      else if let TokenType::Operator(op) = token{
        let mut left = Vec::new();
        if op ==  "("{
          parens+=1;
        }
        else if op ==  ")"{
          parens-=1;
        }
        if parens!=0{
          i+=1;
          continue;
        }
        if op.ends_with('='){ // definetly assigment a = ...; or creation uint a = ...; 
          for _ in 0..i {
            left.push(self.tokens.remove(0));
          }
          let operation = self.tokens.remove(0);
          let right = self.capture_until_semicolon();
          ast.push(ASTNode::Operation(operation, left, right));
          i = 0;
          continue;
        }
        else if op == ";" { // probably funcion call a(); or single operator r.a++; or return a; or int a;'
          for _ in 0..i {
            left.push(self.tokens.remove(0));
          }
          self.tokens.remove(0);
          if let Some(TokenType::Operator(x)) = left.last(){
            if x == "++" || x == "--"{
              ast.push(ASTNode::UnaryOperation(left.remove(left.len()-1),left));
              i = 0;
              continue;
            }
          }
          ast.push(ASTNode::FunctionCall(left));
          i = 0;
          continue;
        }
        else if op == "{" {
          for _ in 0..i {
            left.push(self.tokens.remove(0));
          }
          self.tokens.remove(0); // remove {
          let body = self.capture_in_curly();

          if let Some(TokenType::Operator(x)) = self.tokens.first() {
            if x == ";"{
              self.tokens.remove(0);
            }
          }
          ast.push(ASTNode::StructureDefine(left, Parser::new(body).parse()));
          i = 0;
          continue;
        }
        else if op == ":" { // unsigned int wr:1; or label:
          for _ in 0..i {
            left.push(self.tokens.remove(0));
          }
          self.tokens.remove(0); // remove ':'
          if let Some(TokenType::Number(_)) = self.tokens.first(){
            let mut right = Vec::new();
            loop {
              if let Some(TokenType::Operator(o)) = self.tokens.first(){
                if o == ";" {
                  self.tokens.remove(0);
                  break;
                }
              }
              right.push(self.tokens.remove(0));
            }
            ast.push(ASTNode::BitField(left,right));
            i = 0;
            continue;
        }
          // must be label
          if let Some(TokenType::Identifier(x)) = left.first(){
            ast.push(ASTNode::Label(x.clone()));
          }

          i = 0;
          continue;
        }
      }
      else if let TokenType::Preprocessor = token {
        if let Some(TokenType::Identifier(x)) = self.tokens.get(1){
          if x == "define"{
            if let Some(TokenType::Identifier(name)) = self.tokens.get(2){
              if let Some(q) = self.tokens.get(3){
                if let TokenType::Number(_) = q{
                  let mut b = vec![];
                  b.push(self.tokens.get(3).unwrap().clone());
                  ast.push(ASTNode::PreDefineConstant(name.clone(), b));
                  self.tokens.drain(0..4);
                }
                else if let TokenType::Operator(r) = q {
                    if r == "(" {
                      let name = name.clone();
                      self.tokens.drain(0..4);
                      let args_or_expression = self.capture_in_paren();
                      if let Some(TokenType::Operator(_)) = self.tokens.first(){
                        self.tokens.remove(0);
                        let expresion = self.capture_in_paren();
                        ast.push(ASTNode::PreDefineExpressionWithParams(name,args_or_expression, expresion));
                        i = 0;
                        continue;
                      }
                      ast.push(ASTNode::PreDefineExpression(name,args_or_expression));
                      i = 0;
                      continue;
                    }
                }
              }
            }
          }else {
            todo!("Unexpected preprocessor directive {}",x);
          }
        }
        self.tokens.remove(0);
      }
      i+=1;
    }
    return ast;

  }
  fn capture_in_paren(&mut self) -> Vec<TokenType> {
    let mut braces = 1;
    let mut body = Vec::new();
    loop {
      if let Some(x) = self.tokens.first(){
        if let TokenType::Operator(op) = x{
          if op == "(" {
            braces+=1;
          }
          else if op == ")"{
            braces-=1;
            if braces == 0{
              self.tokens.remove(0); // Remove }
              return body;
            }
          }
        }
        body.push(self.tokens.remove(0));
        continue;
      }
      panic!("Missmached parens!");
    }
  }
  fn capture_until_semicolon(&mut self) -> Vec<TokenType> {
    let mut body = Vec::new();
    loop {
      if let Some(x) = self.tokens.first(){
        if let TokenType::Operator(op) = x{
          if op == ";" {
            self.tokens.remove(0);
            return  body;
          }
        }
        body.push(self.tokens.remove(0));
        continue;
      }
      panic!("Expected simicolon!");
    }

  }
  fn capture_in_curly(&mut self) -> Vec<TokenType> {
    let mut braces = 1;
    let mut body = Vec::new();
    loop {
      if let Some(x) = self.tokens.first(){
        if let TokenType::Operator(op) = x{
          if op == "{" {
            braces+=1;
          }
          else if op == "}"{
            braces-=1;
            if braces == 0{
              self.tokens.remove(0); // Remove }
              return body;
            }
          }
        }
        body.push(self.tokens.remove(0));
        continue;
      }
      panic!("Missmached curly braces!");
    }
  }
}

/*
#[cfg(test)]
mod parser_tests;






pub struct Parser {
}
  
impl Parser {
    pub fn hit(buffer : &mut Vec<Token>){
      Parser::preprocess(buffer);
  
      Parser::rpn_buffer(buffer);
    }

    pub fn hit_tree(buffer : &mut Vec<Token>,root: &mut TreeNode<Token>){
      Parser::preprocess(buffer);
  
      Parser::rpn_buffer(buffer);
      Parser::make_tree(buffer, root);
      for x in buffer {
        print!("{}", x);
      }
      println!();
      //buffer.clear();
    }
    fn preprocess(buffer : &mut Vec<Token>) {
      let mut i: usize = 0;
      let mut l: usize = buffer.len();
      let buffer_mut = buffer;
  
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
  
  
    fn make_tree(buffer : &mut Vec<Token>,root: &mut TreeNode<Token>) {
      if buffer.len() == 0 {
        panic!("Tree with length 0?")
      }
      
      let last_token = buffer.pop().unwrap();
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
      rec(buffer,root);
      
      println!("------>");
      println!("{}",root);
      println!("------<");
    }

    /// Reverse prefix! Not postfix, and not prefix
    fn rpn_buffer(buffer : &mut Vec<Token>) {
      let mut output: Vec<Token> = Vec::new();
      let mut stack: Vec<Token> = Vec::new();
      while !buffer.is_empty() {
        let e = buffer.remove(0);
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
        TokenType::Variable | TokenType::Number | TokenType::String  =>{
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
    buffer.clear();
    buffer.append(&mut output);
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

 */