use crate::token::TokenType;
#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
  // Preprocessor, Name , definition
  PreDefineConstant(String,TokenType),
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
  
}
use std::{collections::HashMap, fmt};
#[derive(Debug, PartialEq, Clone)]
pub struct Argument{
  a_type: String, 
  name: String, 
}

impl Argument {
    pub fn new(a_type: String, name: String) -> Self {
        Self { a_type, name }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum BSTNode{
  // return type and name, args, body
  FunctionDeclaration(Argument,Vec<Argument>,Vec<BSTNode>),
  // func name, 
  FunctionCall(String,Vec<Vec<String>>),
  // whole expression
  Operation(Vec<String>),
  // whole expression
  Return(Vec<String>),
  // expression, body, else
  If(Vec<String>,Vec<BSTNode>,Vec<BSTNode>),
  // whole expression
  Loop(Vec<BSTNode>),
  Break,
  Continue,
}
#[derive(Debug, PartialEq, Clone)]
pub enum VariableType{
  Byte,UByte,
  Short,UShort,
  Int,UInt,
  Long,ULong,
  Void,
  Pointer(Box<VariableType>),
  Struct(String,Vec<VariableType>),
}
impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
          ASTNode::PreDefineExpression(_,_) =>{Ok(())}
          ASTNode::PreDefineExpressionWithParams(_,_,_) =>{Ok(())}

          ASTNode::PreDefineConstant(name, token) => {
            write!(f, "#define {} {token}", name,)?;
            Ok(())
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
            for node in declaration {
              write!(f, "{} ", node)?;
            }
            writeln!(f, "{{")?;

            for i in body {
              if let ASTNode::StructureDefine(_, _) = i{
                writeln!(f, "{} ", i)?;
                continue;
              }
              writeln!(f, "   {} ", i)?;
            }
            write!(f, "}}")
          }
          ASTNode::FunctionCall(tokens) => {
            for token in tokens {
              write!(f, "{} ", token)?;
            }
            Ok(())
          }
          ASTNode::Operation(op, left, right) => {
            for token in left {
              write!(f, "{}", token)?;
            }
            write!(f, "{}",op)?;
            for token in right {
              write!(f, "{}", token)?;
            }
            write!(f, "")
          }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
  tokens: Vec<TokenType>,
}

impl Parser {
  pub fn is_string_vtype(a:&str) -> Option<VariableType>{
    match a {
      "sbyte" =>Some(VariableType::Byte),
      "byte" =>Some(VariableType::UByte),
      "short" =>Some(VariableType::Short),
      "ushort" =>Some(VariableType::UShort),
      "int" =>Some(VariableType::Int),
      "uint" =>Some(VariableType::UInt),
      "long" =>Some(VariableType::Long),
      "ulong" =>Some(VariableType::ULong),

      "void" =>Some(VariableType::Void),
      _ => None
    }
  }
  
  pub fn new(tokens: Vec<TokenType>) -> Self {
    Self { tokens }
  }
  pub fn parse(&mut self) -> Vec<BSTNode>{
    let mut a = self.first_step();
    let x = Parser::second_step(&mut a);
    return x;
  }
  fn second_step(blocks:&mut Vec<ASTNode>) -> Vec<BSTNode>{
    let _constand_expressions: HashMap<String, Vec<TokenType>> = Parser::get_defines(blocks);

    let mut instr_nodes: Vec<BSTNode> = Vec::new();
    while blocks.len()!=0{
      let x = &blocks.remove(0);
      println!("{}",x);

      match x {
        ASTNode::StructureDefine(left, body) => {
          if let Some(TokenType::Keyword(kw)) = left.first() {
            if let Some(_) = Parser::is_string_vtype(kw){ // int function() or int i = ...
              if let Some(TokenType::Identifier(name)) = left.get(1){
                let n = Argument::new(kw.clone(), name.clone());
                if let Some(TokenType::Operator(op)) = left.get(2) {
                  if op == "("{ // Func creation int a(...){
                    let len = left.len();
                    let vec = Parser::get_args(left,3,len);
                    let mut x = body.clone();
                    let f_body = Parser::second_step(&mut x);
                    instr_nodes.push(BSTNode::FunctionDeclaration(n, vec, f_body));
                    continue;
                  }
                  else if op == "=" { // int a = ...
                    panic!("Wtf?");
                  }
                }
              }
              panic!("Wtf?");
            }
            else{ // if, while, do,
              match kw as &str {
                "if" => {
                  let mut expr = vec![];
                  Parser::get_expression(left,1,left.len(),&mut expr);
                  let mut bbody = body.clone();
                  let body: Vec<BSTNode> = Parser::second_step(&mut bbody);
                  let mut ielse = vec![];
                  if let Some(ASTNode::StructureDefine(x, y)) = blocks.first(){
                    if let Some(TokenType::Keyword(prob)) = x.first() {
                      if prob == "else"{
                
                        let mut ebody = y.clone();
                        let body: Vec<BSTNode> = Parser::second_step(&mut ebody);
                        blocks.remove(0);
                        ielse = body;
                      }
                    }
                  }
                  instr_nodes.push(BSTNode::If(expr, body,ielse))
                },
                "else" => panic!("unmached else"),
                "while" => {
                  let mut expr = vec![];
                  Parser::get_expression(left,1,left.len(),&mut expr);
                  let mut bbody = body.clone();
                  let mut body: Vec<BSTNode> = Parser::second_step(&mut bbody);
                  body.insert(0,BSTNode::If(expr, vec![],vec![BSTNode::Break]));
                  instr_nodes.push(BSTNode::Loop(body));
                },
                _ => todo!("{}",kw)
        
              }
            }
          }
        }
        ASTNode::FunctionCall(left) => {
          if let Some(TokenType::Identifier(name)) = left.first(){ // a(...)
            if left.len() == 3 {
              instr_nodes.push(BSTNode::FunctionCall(name.clone(), vec![]));
              continue;
            }
            let len = left.len()-1;

            let mut arg = vec![];
            let mut last  = 2;
            for i in 2..len {
              if let TokenType::Operator(op) = &left[i]{
                if op == ","{
                  let mut buffer = vec![];
                  let mut x = Parser::get_expression(&left,last,i,&mut buffer);
                  last = i+1;
                  arg.push(buffer.clone());
                  buffer.clear();
                  continue;
                }
              }
            }
            instr_nodes.push(BSTNode::FunctionCall(name.clone(), arg));
          }
          else if let Some(TokenType::Keyword(name)) = left.first(){ // return .. etc
            let c = name as &str;
            let mut expr = vec![];
            Parser::get_expression(left,1,left.len(),&mut expr);
            match c {
              "return" => instr_nodes.push(BSTNode::Return(expr)),
              "break" => instr_nodes.push(BSTNode::Break),
              "continue" => instr_nodes.push(BSTNode::Continue),
              _ => panic!()
            }
          }
        }
        ASTNode::Operation(operator,left,right) => { // int a = ..;
          let mut expr = Vec::new();
          Parser::get_expression(left, 0, left.len(), &mut expr);
          if let TokenType::Operator(op) = operator {
            expr.push(op.clone());
          }
          Parser::get_expression(right, 0, right.len(), &mut expr);
          
          instr_nodes.push(BSTNode::Operation(expr));
        }
      _ => panic!(),
      }
    }


    /*let mut keywords: HashMap<String, VariableType> = HashMap::new();
    for x in blocks{
      println!("{}",x);
      match x {
        ASTNode::StructureDefine(left, right) =>{
          if let Some(TokenType::Keyword(TYPE)) = left.first() {
            if TYPE == "struct" {
              if let Some(TokenType::Identifier(x)) = left.get(1) {
                let name = x.clone();
                let mut types = Vec::new();
                for a in right {
                  if let ASTNode::FunctionCall(x) = a {
                    if x.len() != 2{
                      todo!("Add functionality for struct")
                    }
                    if let TokenType::Keyword(typ) = &x[0]{
                      if let TokenType::Identifier(_) = &x[1]{
                        let t = Parser::string_to_vtype(&typ);
                        types.push(t);
                      }
                    }
                    else if let TokenType::Identifier(typ) = &x[0]{
                      if let TokenType::Identifier(_) = &x[1]{
                        types.push(VariableType::Struct(typ.clone(),Vec::new()));
                      }
                    }
                    else {
                      panic!("Expected name!");
                    }
                  }
                }

                keywords.insert(name.clone(), VariableType::Struct(name,types));
              }
              todo!()
            }
            else if TYPE == "enum"{
              todo!()
            }
            else if TYPE == "class"{
              todo!()
            }
            else if TYPE == "interface"{
              todo!()
            }
          }
        }
        _ =>{}
      }
    }
    for x in &keywords {
      println!("{}",x.0)
    }*/
    
    return instr_nodes;
  }
  fn get_expression(toks: &Vec<TokenType>,start_index: usize,end_index: usize,expr: &mut Vec<String>){
    let mut i = start_index;
    while i<end_index {
      match toks.get(i) {
        Some(TokenType::Keyword(typ)) => {
          let mut k_type = typ.clone();
          if i+1<end_index{
            if let Some(TokenType::Operator(x)) = toks.get(i+1){
              if x == "*" {
                k_type.push_str(x);
                i+=1;
              }
            }
          }
          if i>0 &&!expr.is_empty()&& expr.last().unwrap() == "("{
            if let Some(TokenType::Operator(cp)) = toks.get(i+1) {
              if cp == ")" {
                expr.pop();
                k_type.insert(0, '(');
                k_type.push( ')');
                i+=1;  
              }
            }
          }
          expr.push(k_type);
        }
        Some(TokenType::Identifier(typ)) => expr.push(typ.clone()),
        Some(TokenType::Operator(op)) => expr.push(op.clone()),
        Some(TokenType::Number(op)) => expr.push(op.clone()),
        Some(TokenType::String(op)) => expr.push(op.clone()),
        _ => (),
    }
      i+=1;
    }
  }
  fn get_type(toks: &Vec<TokenType>,start_index: usize,end_index: usize) -> String{
    let mut vtype = String::new();
    for i in start_index..end_index {
      if let Some(TokenType::Keyword(typ)) = toks.get(i){
        vtype.push_str(typ); 
      }else if let Some(TokenType::Identifier(typ)) = toks.get(i) {
        vtype.push_str(typ);
      }else if let Some(TokenType::Operator(op)) = toks.get(i) {
        if op == "*"{
          vtype.push_str(op);
          continue;
        }
        panic!("huh?")
      }
    }
    return vtype;
  }
  fn get_defines(blocks: &mut Vec<ASTNode>) -> HashMap<String, Vec<TokenType>>{
    let mut constand_expressions: HashMap<String, Vec<TokenType>> = HashMap::new();
    let mut i = 0;
    while let Some(x) = blocks.get(i) {
      if let ASTNode::PreDefineConstant(n, tt) = x{
        constand_expressions.insert(n.clone(), vec![tt.clone()]);
        blocks.remove(i);
        continue;
      }
      else if let ASTNode::PreDefineExpression(n, tt) = x{
        constand_expressions.insert(n.clone(), tt.clone());
        blocks.remove(i);
        continue;
      } 
      else if let ASTNode::PreDefineExpressionWithParams(_, _, _) = x{
        todo!();
      }
      i+=1;
    }
    return  constand_expressions;
  }
  fn get_args(toks: &Vec<TokenType>,start_index: usize,end_index: usize) -> Vec<Argument>{
    let mut vec = Vec::new();
    let mut last_index = start_index;
    for i in start_index..end_index {
      if let TokenType::Operator(x) = &toks[i] {
        if x == ")" || x == ","{
          if i == last_index{
            break;
          }
          if let TokenType::Identifier(last) =  &toks[i-1] {
            let name = last.clone();
            let mut a_type = String::new();
            for j in last_index..(i-1) {
              if let TokenType::Keyword(kw) = &toks[j] {
                a_type.push_str(kw);
              }
              else if let TokenType::Identifier(kw) = &toks[j] {
                a_type.push_str(kw);
              }
              else if let TokenType::Operator(op) = &toks[j] {
                a_type.push_str(op);
              }
            }
            vec.push(Argument::new(a_type, name));
            last_index = i+1;
          }
          else {
            panic!("Expected name.")
          }
          
        }     
      }
    }
    return vec;
  }
  
  fn first_step(&mut self) ->  Vec<ASTNode>{ // Divide into blocks
    let mut ast: Vec<ASTNode> = Vec::new();

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
        if op ==  "("{
          parens+=1;
        }
        else if op ==  ")"{
          parens-=1;
        }
        
        if parens==0{
          if let Some(ins) =  self.operator(i, op.clone()){
            ast.push(ins);
            i = 0;
            continue;
          }
        }
      }
      else if let TokenType::Preprocessor = token {
        ast.push(self.preprocessor_instruction());
        i = 0;
        continue;
      }
      
      i+=1;
    }
    return ast;
  }
  fn preprocessor_instruction(&mut self) -> ASTNode{
    if let Some(tt) = self.tokens.get(1){
      if let TokenType::Identifier(x) = tt{
        if x == "define"{ // #define
          if let Some(TokenType::Identifier(name)) = self.tokens.get(2){ // #define name
            if let Some(q) = self.tokens.get(3){
              if let TokenType::Number(_) = q{ // #define name 999
                let b = self.tokens.get(3).unwrap().clone();
                let name = name.clone();
                self.tokens.drain(0..4);
                return ASTNode::PreDefineConstant(name, b);
  
              }
              else if let TokenType::Operator(r) = q { 
                if r == "(" { // #define name (
                  let name = name.clone();
                  self.tokens.drain(0..4);
                  let args_or_expression = self.capture_in_paren();
                  if let Some(TokenType::Operator(_)) = self.tokens.first(){// #define name (...) (
                    self.tokens.remove(0);
                    let expresion = self.capture_in_paren();
                    return ASTNode::PreDefineExpressionWithParams(name,args_or_expression, expresion);
                  }
                  return ASTNode::PreDefineExpression(name,args_or_expression);
                }
              }
              panic!("What? Define doensnt work like that");
            }
          }
        }
        else {
          todo!("Unexpected preprocessor directive! ({})",x);
        }
      }

    }
    todo!("Unexpected preprocessor directive!");
  }
  fn operator(&mut self, i:usize, op:String) -> Option<ASTNode>{
    if op.ends_with('='){ // definetly assigment a = ...; or creation uint a = ...; 
      let left = self.capture_to(i);
      let operation = self.tokens.remove(0);
      let right = self.capture_until_semicolon();
      return Some(ASTNode::Operation(operation, left, right));
    }
    else if op == ";" { // probably funcion call a(); or single operator r.a++; or return a; or int a;
      let mut left = self.capture_to(i);
      self.tokens.remove(0);
      return Some(ASTNode::FunctionCall(left));
    }
    else if op == "{" { // enum a{,while (x){, struct a{, struct {}A;,
      let mut left = self.capture_to(i);
      self.tokens.remove(0); // remove {
      let body = self.capture_in_curly();

      if let Some(TokenType::Operator(x)) = self.tokens.first() {
        if x == ";"{
          self.tokens.remove(0);
        }
      }
      if let Some(TokenType::Identifier(_)) = self.tokens.get(0) {
        if let Some(TokenType::Operator(y)) = self.tokens.get(1) {
          if y == ";"{
            left.push(self.tokens.remove(0));
            self.tokens.remove(0);
          }
        }
      }
      return Some(ASTNode::StructureDefine(left, Parser::new(body).first_step()));
    }
    else if op == ":" { // unsigned int wr:1; or label:
      let left = self.capture_to(i);
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
        return Some(ASTNode::BitField(left,right));
    }
      // must be label
      if let Some(TokenType::Identifier(x)) = left.first(){
        return Some(ASTNode::Label(x.clone()));
      }
    }
    
    return None;
  }
  
  fn capture_to(&mut self,i : usize) -> Vec<TokenType>{
    let mut body = Vec::new();
    for _ in 0..i {
      if let Some(TokenType::Keyword(op)) = self.tokens.first(){
        if op == "unsigned"{
          if let TokenType::Keyword(sop) = self.tokens.get_mut(1).unwrap(){
            sop.insert(0, 'u');
          }
          self.tokens.remove(0);
        }
      }
      body.push(self.tokens.remove(0));
    }
    return body;
  }
  fn capture_in_paren(&mut self) -> Vec<TokenType> {
    let mut braces = 1;
    let mut body = Vec::new();
    loop {
      if let Some(TokenType::Keyword(op)) = self.tokens.first(){
        if op == "unsigned"{
          if let TokenType::Keyword(sop) = self.tokens.get_mut(1).unwrap(){
            sop.insert(0, 'u');
          }
          self.tokens.remove(0);
        }
      }
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
      if let Some(TokenType::Keyword(op)) = self.tokens.first(){
        if op == "unsigned"{
          if let TokenType::Keyword(sop) = self.tokens.get_mut(1).unwrap(){
            sop.insert(0, 'u');
          }
          self.tokens.remove(0);
        }
      }
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
      if let Some(TokenType::Keyword(op)) = self.tokens.first(){
        if op == "unsigned"{
          if let TokenType::Keyword(sop) = self.tokens.get_mut(1).unwrap(){
            sop.insert(0, 'u');
          }
          self.tokens.remove(0);
        }
      }
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