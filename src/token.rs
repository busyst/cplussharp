use core::fmt;
#[derive(Debug, Clone)]
pub enum TokenType {
  Keyword(String),
  Preprocessor,
  Identifier(String),
  Operator(String),
  Number(String),
  String(String),
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
            TokenType::Keyword(a) => write!(f, "{}",a),
            TokenType::Preprocessor => write!(f, "#"),
            TokenType::Identifier(a) => write!(f, "{}",a),
            TokenType::Operator(a) => write!(f, "{}",a),
            TokenType::Number(a) => write!(f, "{}",a),
            TokenType::String(a) => write!(f, "{}",a),
        }
    }
}