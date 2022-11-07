use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Statement{
  If {
    #[serde(rename = "if")]
    branches: Vec<IfConditions>,
    #[serde(rename = "else")]
    else_branch: Option<Box<Statement>>
  },
  While {
    #[serde(rename = "while")]
    condition: Box<Expression>,
    #[serde(rename = "do")]
    do_block: Box<Statement>
  },
  StatementList(Vec<Statement>),
  Iterator {
    iterator: String,
    from: Box<Expression>,
    to: Box<Expression>,
    step: Option<Box<Expression>>,
    #[serde(rename = "do")]
    do_block: Box<Statement>
  },
  Until {
    #[serde(rename = "do")]
    do_block: Box<Statement>,
    until: Box<Expression>
  },
  Declare { declare: String, value: Box<Expression> },
  Set { set: String, value: Box<Expression> },
  Call { 
    #[serde(rename = "call")]
    name: String,
    args: Vec<Expression>
  },
  Return {
    #[serde(rename = "return")]
    return_value: Box<Expression>
  },
  Other(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IfConditions {
  pub cond: Expression,
  pub then: Box<Statement>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnaryOp {
  #[serde(rename = "-")]
  NumericNegation,
  #[serde(rename = "!")]
  LogicNegation,
  #[serde(rename = "~")]
  BitwiseNegation
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BinaryOp {
  #[serde(rename = "+")]
  Addition,
  #[serde(rename = "-")]
  Substraction,
  #[serde(rename = "*")]
  Multiplication,
  #[serde(rename = "/")]
  Division,
  #[serde(rename = "^")]
  Exponentiation,
  #[serde(rename = "%")]
  Modulo,
  #[serde(rename = "&")]
  BitwiseAnd,
  #[serde(rename = "|")]
  BitwiseOr,
  #[serde(rename = ">>")]
  RightShift,
  #[serde(rename = "<<")]
  LeftShift,
  #[serde(rename = "<")]
  Lesser,
  #[serde(rename = "<=")]
  LessEq,
  #[serde(rename = ">")]
  Greater,
  #[serde(rename = ">=")]
  GreaterEq,
  #[serde(rename = "==")]
  Equal,
  #[serde(rename = "~=")]
  NotEqual,
  #[serde(rename = "and")]
  LogicalAnd,
  #[serde(rename = "or")]
  LogicalOr
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Expression {
  UnaryOp {
    #[serde(rename = "unop")]
    op: UnaryOp,
    arg: Box<Expression>
  },
  BinaryOp { 
    #[serde(rename = "argl")]
    lhs: Box<Expression>,
    #[serde(rename = "binop")]
    op: BinaryOp,
    #[serde(rename = "argr")]
    rhs: Box<Expression>
  },
  VariableAccess (String),
  FunctionCall {
    #[serde(rename = "call")]
    name: String,
    args: Vec<Expression>
  },
  NumericLiteral (f64)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeclarationStatement {
  pub function: String,
  pub args: Vec<String>,
  pub block: Statement
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TopStatement {
  Statement(Statement),
  DeclarationStatement(DeclarationStatement)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct JsonLang {
  pub statements: Vec<TopStatement>
}
