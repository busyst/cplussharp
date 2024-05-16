use core::fmt;

use crate::tree::TreeNode;

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
  Keyword,
  Variable,
  UnOperator,
  Operator,
  Number,
  String,
  LParen,
  RParen,
  LBrace,
  RBrace,
  LSquareBrace,
  RSquareBrace,
  Semicolon,
  Colon,


  // Parser token
  Pointer
}
impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    fn ne(&self, other: &Self) -> bool {
        std::mem::discriminant(self) != std::mem::discriminant(other)
    }
}

impl Eq for TokenType {}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Keyword => write!(f, "Keyword"),
            TokenType::Variable => write!(f, "Variable"),
            TokenType::UnOperator => write!(f, "UnOperator"),
            TokenType::Operator => write!(f, "BiOperator"),
            TokenType::Number => write!(f, "Number"),
            TokenType::String => write!(f, "String"),
            TokenType::LParen => write!(f, "Left Paren"),
            TokenType::RParen => write!(f, "Right Paren"),
            TokenType::LBrace => write!(f, "Left Brace"),
            TokenType::RBrace => write!(f, "Right Brace"),
            TokenType::LSquareBrace => write!(f, "Left Square Brace"),
            TokenType::RSquareBrace => write!(f, "Right Square Brace"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::Pointer => write!(f,"Pointer")
        }
    }
}

#[derive(Debug,Clone)]
pub struct Token{
  token_type : TokenType,
  val : String,
}
impl Token {
    pub fn new(token_type: TokenType, val: String) -> Self {
        Self {
        token_type,
        val,
        }
    }

    pub fn to_tree_node(&self) -> TreeNode<&Token> {
        TreeNode::new(self)
    }
    pub fn to_tree_node_clone(&self) -> TreeNode<Token> {
        TreeNode::new(self.clone())
    }
    pub fn val_mut(&mut self) -> &mut String{
        &mut self.val
    }

    pub fn val(&self) -> &str {
        &self.val
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
    
    pub fn set_token_type(&mut self, token_type: TokenType) {
        self.token_type = token_type;
    }
    
}
impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {})", self.token_type, self.val)
  }
}