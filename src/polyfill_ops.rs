use std::vec;

use crate::jsonlang;
use crate::variable_analysis::{ AnalysisResults, Function, UnaryOp, self };
use crate::cordic::cordic_polyfill::generate_exp_polyfills;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BinaryOp {
  Addition,
  Substraction,
  Multiplication,
  Division,
  IntAddition,
  IntSubstraction,
  IntMultiplication,
  IntDivision,
  BitwiseAnd,
  BitwiseOr,
  RightShift,
  LeftShift,
  Lesser,
  LessEq,
  Greater,
  GreaterEq,
  Equal,
  NotEqual
}

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

fn float_to_int(arg: Expression) -> Expression {
  Expression::UnaryOp { op: UnaryOp::FloatToInt, arg: Box::new(arg) }
}
fn int_to_float(arg: Expression) -> Expression {
  Expression::UnaryOp { op: UnaryOp::IntToFloat, arg: Box::new(arg) }
}

impl AnalysisResults<Statement> {
  fn get_polyfill_index(&mut self, name: &str) -> u32 {
    if self.funcname_map.contains_key(name) {
      *self.funcname_map.get(name).unwrap()
    } else {
      let index = self.funcname_map.len() as u32;
      self.funcname_map.insert(name.to_string(), index);
      index
    }
  }
  fn polyfill_binop(&mut self, op: jsonlang::BinaryOp, lhs: variable_analysis::Expression, rhs: variable_analysis::Expression) -> Expression {
    let mut box_and_poly = |expr| Box::new(self.add_polyfills_to_expression(expr));
    let mut leave_unchanged = |lhs, op, rhs| Expression::BinaryOp { 
      lhs: box_and_poly(lhs), op: op, rhs: box_and_poly(rhs)
    };
    let zero_if_zero = |expr| Box::new(int_to_float(Expression::BinaryOp {
      lhs: expr,
      op: BinaryOp::NotEqual,
      rhs: Box::new(Expression::NumericLiteral(0.0))
    }));
    match op {
      jsonlang::BinaryOp::Addition => leave_unchanged(lhs, BinaryOp::Addition, rhs),
      jsonlang::BinaryOp::Substraction => leave_unchanged(lhs, BinaryOp::Substraction, rhs),
      jsonlang::BinaryOp::Multiplication => leave_unchanged(lhs, BinaryOp::Multiplication, rhs),
      jsonlang::BinaryOp::Division => leave_unchanged(lhs, BinaryOp::Division, rhs),
      jsonlang::BinaryOp::Exponentiation => Expression::FunctionCall(self.get_polyfill_index("#pow"),
        vec![self.add_polyfills_to_expression(lhs), self.add_polyfills_to_expression(rhs)]),
      jsonlang::BinaryOp::Modulo => todo!("¬impl"),
      jsonlang::BinaryOp::BitwiseAnd => leave_unchanged(lhs, BinaryOp::BitwiseAnd, rhs),
      jsonlang::BinaryOp::BitwiseOr => leave_unchanged(lhs, BinaryOp::BitwiseOr, rhs),
      jsonlang::BinaryOp::RightShift => leave_unchanged(lhs, BinaryOp::RightShift, rhs),
      jsonlang::BinaryOp::LeftShift => leave_unchanged(lhs, BinaryOp::LeftShift, rhs),
      jsonlang::BinaryOp::Lesser => leave_unchanged(lhs, BinaryOp::Lesser, rhs),
      jsonlang::BinaryOp::LessEq => leave_unchanged(lhs, BinaryOp::LessEq, rhs),
      jsonlang::BinaryOp::Greater => leave_unchanged(lhs, BinaryOp::Greater, rhs),
      jsonlang::BinaryOp::GreaterEq => leave_unchanged(lhs, BinaryOp::GreaterEq, rhs),
      jsonlang::BinaryOp::Equal => leave_unchanged(lhs, BinaryOp::Equal, rhs),
      jsonlang::BinaryOp::NotEqual => leave_unchanged(lhs, BinaryOp::NotEqual, rhs),
      jsonlang::BinaryOp::LogicalAnd => Expression::BinaryOp {
        lhs: zero_if_zero(box_and_poly(lhs)),
        op: BinaryOp::Multiplication,
        rhs: box_and_poly(rhs)
      },
      jsonlang::BinaryOp::LogicalOr => Expression::FunctionCall(self.get_polyfill_index("#logic_or"),
        vec![self.add_polyfills_to_expression(lhs), self.add_polyfills_to_expression(rhs)])
    }
  }

  fn add_polyfills_to_expression(&mut self, expr: variable_analysis::Expression) -> Expression {
    match expr {
      variable_analysis::Expression::UnaryOp { op, arg } => Expression::UnaryOp { op: op, arg: Box::new(self.add_polyfills_to_expression(*arg)) },
      variable_analysis::Expression::BinaryOp { lhs, op, rhs } => self.polyfill_binop(op, *lhs, *rhs),
      variable_analysis::Expression::LocalGet(index) => Expression::LocalGet(index),
      variable_analysis::Expression::GlobalGet(index) => Expression::GlobalGet(index),
      variable_analysis::Expression::FunctionCall(index, args) => Expression::FunctionCall(index,
        args.into_iter().map(|e| self.add_polyfills_to_expression(e)).collect()),
      variable_analysis::Expression::NumericLiteral(n) => Expression::NumericLiteral(n)
    }
  }

  fn add_polyfills_to_statement(&mut self, stmt: variable_analysis::Statement) -> Statement {
    let fix_vec = |s: &mut Self, list: Vec<variable_analysis::Statement>| list.into_iter()
      .map(|e| s.add_polyfills_to_statement(e)).collect();
    match stmt {
      variable_analysis::Statement::Loop(body) => Statement::Loop(fix_vec(self, body)),
      variable_analysis::Statement::If { cond, then, otherwise } => Statement::If {
        cond: self.add_polyfills_to_expression(cond),
        then: fix_vec(self, then),
        otherwise: fix_vec(self, otherwise)
      },
      variable_analysis::Statement::LocalSet(index, expr) => Statement::LocalSet(index, self.add_polyfills_to_expression(expr)),
      variable_analysis::Statement::GlobalSet(index, expr) => Statement::GlobalSet(index, self.add_polyfills_to_expression(expr)),
      variable_analysis::Statement::Call(index, exprs) => Statement::Call(index,
        exprs.into_iter().map(|e| self.add_polyfills_to_expression(e)).collect()),
      variable_analysis::Statement::Return(expr) => Statement::Return(self.add_polyfills_to_expression(expr)),
      variable_analysis::Statement::Break => Statement::Break,
      variable_analysis::Statement::Continue => Statement::Continue
    }
  }
  pub fn polyfill_ops(program: AnalysisResults<variable_analysis::Statement>) -> Self {
    let mut res = AnalysisResults {
      global_variables: program.global_variables,
      funcname_map: program.funcname_map,
      functions: std::collections::HashMap::new()
    };
    res.functions = program.functions.into_iter().map(|(index, func)| {
      (index, Function {
        arguments: func.arguments,
        local_count: func.local_count,
        body: func.body.into_iter().map(|stmt| res.add_polyfills_to_statement(stmt)).collect()
      })
    }).collect();

    if let Some(index) = res.funcname_map.get("#logic_or") {
      res.functions.insert(*index, Function {
        arguments: 2, local_count: 0,
        body: vec![Statement::If {
          cond: float_to_int(Expression::LocalGet(0)),
          then: vec![Statement::Return(Expression::LocalGet(0))],
          otherwise: vec![Statement::Return(Expression::LocalGet(1))],
        }]
      });
    }
    if res.funcname_map.contains_key("#pow") {
      let index = *res.funcname_map.get("#pow").unwrap();
      let (cordic_pow, lut_table, cordic_ln, cordic_exp) = generate_exp_polyfills(index);
      res.funcname_map.insert("#cordic_lut".to_string(), index + 1);
      res.funcname_map.insert("#ln_cordic".to_string(), index + 2);
      res.funcname_map.insert("#exp_cordic".to_string(), index + 3);
      res.functions.insert(index, cordic_pow);
      res.functions.insert(index + 1, lut_table);
      res.functions.insert(index + 2, cordic_ln);
      res.functions.insert(index + 3, cordic_exp);
    }
    res
  }
}

pub fn polyfill_ops(program: AnalysisResults<variable_analysis::Statement>) -> AnalysisResults<Statement> {
  AnalysisResults::polyfill_ops(program)
}
