use std::collections::HashMap;
use crate::jsonlang::{ BinaryOp, self };
use crate::control_simplification::{ SimplifiedStatement, SimplifiedTopStatement };
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Statement {
  Loop(Vec<Statement>),
  If {
    cond: Expression,
    then: Vec<Statement>,
    otherwise: Vec<Statement>
  },
  Break,
  Continue,
  LocalSet(u32, Expression),
  GlobalSet(u32, Expression),
  Call(u32, Vec<Expression>),
  Return(Expression),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnaryOp {
  NumericNegation,
  LogicNegation,
  BitwiseNegation,
  FloatToInt,
  IntToFloat,
  Sqrt,
  Floor
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Expression {
  UnaryOp {
    op: UnaryOp,
    arg: Box<Expression>
  },
  BinaryOp { 
    lhs: Box<Expression>,
    op: BinaryOp,
    rhs: Box<Expression>
  },
  LocalGet(u32),
  GlobalGet(u32),
  FunctionCall(u32, Vec<Expression>),
  NumericLiteral (f64)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariableStore {
  index: u32,
  stores: Vec<HashMap<String, u32>>
}

impl VariableStore {
  pub fn new() -> Self {
    VariableStore { index: 0, stores: vec![HashMap::new()] }
  }
  pub fn declare(&mut self, name: String) -> u32 {
    let index = self.index;
    self.stores.last_mut().unwrap().insert(name, index);
    self.index += 1;
    index
  }
  pub fn has(&self, name: &String) -> bool {
    for store in self.stores.iter().rev() {
      if store.contains_key(name) {
        return true
      }
    }
    false
  }
  pub fn declare_or_get(&mut self, name: String) -> u32 {
    if self.has(&name) {
      self.get_id(name)
    } else {
      self.declare(name)
    }
  }
  pub fn enter_block(&mut self) {
    self.stores.push(HashMap::new())
  }
  pub fn exit_block(&mut self) {
    self.stores.pop();
  }
  pub fn get_id(&self, name: String) -> u32 {
    for store in self.stores.iter().rev() {
      if store.contains_key(&name) {
        return *store.get(&name).unwrap()
      }
    }
    panic!("Variable not found")
  }
  pub fn count(&self) -> u32 {
    return self.index
  }
  pub fn list_variables(&self) -> Vec<(&String, &u32)> {
    self.stores.iter().flat_map(|s: &HashMap<String, u32>| s.iter()).collect()
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function<T> {
  pub arguments: u32,
  pub local_count: u32,
  pub body: Vec<T>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnalysisResults<T> {
  pub global_variables: VariableStore,
  pub funcname_map: HashMap<String, u32>,
  pub functions: HashMap<u32, Function<T>>
}

impl<T> AnalysisResults<T> {
  pub fn allocate_index(&mut self, name: String) {
    if self.funcname_map.contains_key(&name) {
      panic!("Duplicate function definition found")
    }
    let index = self.funcname_map.len() as u32;
    self.funcname_map.insert(name.clone(), index);
  }
}

impl AnalysisResults<Statement> {
  pub fn new() -> Self {
    return AnalysisResults {
      global_variables: VariableStore::new(),
      funcname_map: HashMap::new(),
      functions: HashMap::new()
    }
  }
  pub fn analyze_top(&mut self, program: Vec<SimplifiedTopStatement>) {
    let mut top_statements: Vec<SimplifiedStatement> = Vec::new();
    for statement in program.iter() {
      match statement {
        SimplifiedTopStatement::Statement(_) => (),
        SimplifiedTopStatement::DeclarationStatement { name, .. } => self.allocate_index(name.clone())
      }
    }
    for statement in program {
      match statement {
        SimplifiedTopStatement::Statement(stmt) => {
          top_statements.push(stmt)
        },
        SimplifiedTopStatement::DeclarationStatement { name, args, content } => {
          self.analyze_function(name, args, content)
        }
      }
    }
    self.allocate_index("#main".to_string());
    self.analyze_function("#main".to_string(), Vec::new(), SimplifiedStatement::Block(top_statements))
  }
  fn analyze_function(&mut self, name: String, args: Vec<String>, content: SimplifiedStatement) {
    let index = *self.funcname_map.get(&name).unwrap();
    let mut locals = VariableStore::new();
    let arguments = args.len() as u32;
    for arg in args {
      locals.declare(arg);
    }
    let body = self.analyze_variables(content, &mut locals);
    self.functions.insert(index, Function {
      arguments,
      local_count: locals.count() - arguments,
      body
    });
  }
  fn analyze_variables(&mut self, stmt: SimplifiedStatement, locals: &mut VariableStore) -> Vec<Statement> {
    let mut processed = Vec::new();
    match stmt {
      SimplifiedStatement::Loop(content) => processed.push(Statement::Loop(self.analyze_variables(*content, locals))),
      SimplifiedStatement::If { cond, then, otherwise } => processed.push(Statement::If {
        cond: self.translate_expression(cond, locals),
        then: self.analyze_variables(*then, locals),
        otherwise: self.analyze_variables(*otherwise, locals)
      }),
      SimplifiedStatement::Break => processed.push(Statement::Break),
      SimplifiedStatement::Continue => processed.push(Statement::Continue),
      SimplifiedStatement::Declare(name, expr) => {
        let index = locals.declare(name);
        processed.push(Statement::LocalSet(index, self.translate_expression(expr, locals)))
      }
      SimplifiedStatement::Set(name, expr) => {
        processed.push(if locals.has(&name) {
          Statement::LocalSet(locals.get_id(name), self.translate_expression(expr, locals))
        } else {
          Statement::GlobalSet(self.global_variables.declare_or_get(name), self.translate_expression(expr, locals))
        })
      }
      SimplifiedStatement::Call(name, args) => {
        let index = *self.funcname_map.get(&name).expect("Unknown function called");
        let mut translated_args = Vec::new();
        for arg in args {
          translated_args.push(self.translate_expression(arg, locals))
        }
        processed.push(Statement::Call(index, translated_args))
      }
      SimplifiedStatement::Return(expr) => processed.push(Statement::Return(self.translate_expression(expr, locals))),
      SimplifiedStatement::Block(block) => {
        locals.enter_block();
        for stmt in block {
          processed.append(&mut self.analyze_variables(stmt, locals));
        }
        locals.exit_block();
      }
    };
    processed
  }
  fn translate_expression(&mut self, expr: jsonlang::Expression, locals: &mut VariableStore) -> Expression {
    match expr {
      jsonlang::Expression::UnaryOp { op, arg } => Expression::UnaryOp {
        op: match op {
          jsonlang::UnaryOp::BitwiseNegation => UnaryOp::BitwiseNegation,
          jsonlang::UnaryOp::LogicNegation => UnaryOp::LogicNegation,
          jsonlang::UnaryOp::NumericNegation => UnaryOp::NumericNegation
        },
        arg: Box::new(self.translate_expression(*arg, locals)) },
      jsonlang::Expression::BinaryOp { lhs, op, rhs } => Expression::BinaryOp {
        lhs: Box::new(self.translate_expression(*lhs, locals)),
        op,
        rhs: Box::new(self.translate_expression(*rhs, locals))
      },
      jsonlang::Expression::VariableAccess(name) => {
        if locals.has(&name) {
          Expression::LocalGet(locals.get_id(name))
        } else {
          Expression::GlobalGet(self.global_variables.get_id(name))
        }
      },
      jsonlang::Expression::FunctionCall { name, args } => {
        let index = *self.funcname_map.get(&name).expect("Unknown function called");
        let mut translated_args = Vec::new();
        for arg in args {
          translated_args.push(self.translate_expression(arg, locals))
        }
        Expression::FunctionCall(index, translated_args)
      }
      jsonlang::Expression::NumericLiteral(n) => Expression::NumericLiteral(n)
    }
  }
}

pub fn top_analyze_variables(program: Vec<SimplifiedTopStatement>) -> AnalysisResults<Statement> {
  let mut results = AnalysisResults::new();
  results.analyze_top(program);
  return results
}
