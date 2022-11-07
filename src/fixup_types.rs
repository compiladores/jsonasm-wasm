use crate::variable_analysis::{ AnalysisResults, Function, Statement, Expression, UnaryOp };
use crate::jsonlang::{ BinaryOp };

fn demand_int(expr: Expression) -> Expression {
  match expr {
    Expression::UnaryOp { op: UnaryOp::NumericNegation, arg } => into_int(Expression::UnaryOp {
      op: UnaryOp::NumericNegation, arg: Box::new(demand_float(*arg))
    }),
    Expression::UnaryOp { op: UnaryOp::BitwiseNegation, arg } => Expression::UnaryOp {
      op: UnaryOp::BitwiseNegation, arg: Box::new(demand_int(*arg))
    },
    Expression::UnaryOp { op: UnaryOp::LogicNegation, arg } => Expression::UnaryOp {
      op: UnaryOp::LogicNegation, arg: Box::new(demand_int(*arg))
    },
    Expression::UnaryOp { op: UnaryOp::FloatToInt, arg } => Expression::UnaryOp {
      op: UnaryOp::FloatToInt, arg: Box::new(demand_float(*arg))
    },
    Expression::UnaryOp { op: UnaryOp::IntToFloat, arg } => into_int(Expression::UnaryOp {
      op: UnaryOp::IntToFloat, arg: Box::new(demand_int(*arg))
    }),
    Expression::UnaryOp { op: UnaryOp::Sqrt, arg } => into_int(Expression::UnaryOp {
      op: UnaryOp::NumericNegation, arg: Box::new(demand_float(*arg))
    }),
    Expression::UnaryOp { op: UnaryOp::Floor, arg } => into_int(Expression::UnaryOp {
      op: UnaryOp::Floor, arg: Box::new(demand_float(*arg))
    }),
    Expression::BinaryOp { op: BinaryOp::Lesser, .. } => floatop(expr),
    Expression::BinaryOp { op: BinaryOp::LessEq, .. } => floatop(expr),
    Expression::BinaryOp { op: BinaryOp::Greater, .. } => floatop(expr),
    Expression::BinaryOp { op: BinaryOp::GreaterEq, .. } => floatop(expr),
    Expression::BinaryOp { op: BinaryOp::Equal, .. } => floatop(expr),
    Expression::BinaryOp { op: BinaryOp::NotEqual, .. } => floatop(expr),
    Expression::BinaryOp { op: BinaryOp::BitwiseAnd, .. } => binop_into_int(expr),
    Expression::BinaryOp { op: BinaryOp::BitwiseOr, .. } => binop_into_int(expr),
    Expression::BinaryOp { op: BinaryOp::RightShift, .. } => binop_into_int(expr),
    Expression::BinaryOp { op: BinaryOp::LeftShift, .. } => binop_into_int(expr),
    Expression::BinaryOp { lhs, op, rhs } => into_int(Expression::BinaryOp {
      lhs: Box::new(demand_float(*lhs)),
      op: op,
      rhs: Box::new(demand_float(*rhs))
    }),
    Expression::LocalGet(_) => into_int(expr),
    Expression::GlobalGet(_) => into_int(expr),
    Expression::FunctionCall(index, exprs) => into_int(Expression::FunctionCall(index, exprs.into_iter().map(demand_float).collect())),
    Expression::NumericLiteral(_) => into_int(expr)
  }
}

fn binop_into_float(expr: Expression) -> Expression {
  if let Expression::BinaryOp { lhs, op, rhs } = expr {
    return into_float(Expression::BinaryOp { 
      lhs: Box::new(demand_int(*lhs)), op, rhs: Box::new(demand_int(*rhs))
    })
  }
  panic!("Expected binary operation")
}

fn binop_into_int(expr: Expression) -> Expression {
  if let Expression::BinaryOp { lhs, op, rhs } = expr {
    return Expression::BinaryOp { 
      lhs: Box::new(demand_int(*lhs)), op, rhs: Box::new(demand_int(*rhs))
    }
  }
  panic!("Expected binary operation")
}

fn floatop(expr: Expression) -> Expression {
  if let Expression::BinaryOp { lhs, op, rhs } = expr {
    return Expression::BinaryOp { 
      lhs: Box::new(demand_float(*lhs)), op, rhs: Box::new(demand_float(*rhs))
    }
  }
  panic!("Expected binary operation")
}

pub fn into_float(expr: Expression) -> Expression {
  Expression::UnaryOp { op: UnaryOp::IntToFloat, arg: Box::new(expr) }
}

pub fn into_int(expr: Expression) -> Expression {
  Expression::UnaryOp { op: UnaryOp::FloatToInt, arg: Box::new(expr) }
}

fn demand_float(expr: Expression) -> Expression {
  match expr {
    Expression::UnaryOp { op: UnaryOp::NumericNegation, arg } => Expression::UnaryOp {
      op: UnaryOp::NumericNegation, arg: Box::new(demand_float(*arg))
    },
    Expression::UnaryOp { op: UnaryOp::BitwiseNegation, arg } => into_float(Expression::UnaryOp {
      op: UnaryOp::BitwiseNegation, arg: Box::new(demand_int(*arg))
    }),
    Expression::UnaryOp { op: UnaryOp::LogicNegation, arg } => into_float(Expression::UnaryOp {
      op: UnaryOp::LogicNegation, arg: Box::new(demand_int(*arg))
    }),
    Expression::UnaryOp { op: UnaryOp::FloatToInt, arg } => into_float(Expression::UnaryOp {
      op: UnaryOp::FloatToInt, arg: Box::new(demand_int(*arg))
    }),
    Expression::UnaryOp { op: UnaryOp::IntToFloat, arg } => Expression::UnaryOp {
      op: UnaryOp::IntToFloat, arg: Box::new(demand_int(*arg))
    },
    Expression::UnaryOp { op: UnaryOp::Sqrt, arg } => Expression::UnaryOp {
      op: UnaryOp::Sqrt, arg: Box::new(demand_float(*arg))
    },
    Expression::UnaryOp { op: UnaryOp::Floor, arg } => Expression::UnaryOp {
      op: UnaryOp::Floor, arg: Box::new(demand_float(*arg))
    },
    Expression::BinaryOp { op: BinaryOp::Exponentiation, .. } => floatop(expr),
    Expression::BinaryOp { op: BinaryOp::Modulo, .. } => floatop(expr),
    Expression::BinaryOp { op: BinaryOp::BitwiseAnd, .. } => binop_into_float(expr),
    Expression::BinaryOp { op: BinaryOp::BitwiseOr, .. } => binop_into_float(expr),
    Expression::BinaryOp { op: BinaryOp::RightShift, .. } => binop_into_float(expr),
    Expression::BinaryOp { op: BinaryOp::LeftShift, .. } => binop_into_float(expr),
    Expression::BinaryOp { op: BinaryOp::LogicalAnd, .. } => floatop(expr),
    Expression::BinaryOp { op: BinaryOp::LogicalOr, .. } => floatop(expr),
    Expression::BinaryOp { op: BinaryOp::Lesser, .. } => into_float(floatop(expr)),
    Expression::BinaryOp { op: BinaryOp::LessEq, .. } => into_float(floatop(expr)),
    Expression::BinaryOp { op: BinaryOp::Greater, .. } => into_float(floatop(expr)),
    Expression::BinaryOp { op: BinaryOp::GreaterEq, .. } => into_float(floatop(expr)),
    Expression::BinaryOp { op: BinaryOp::Equal, .. } => into_float(floatop(expr)),
    Expression::BinaryOp { op: BinaryOp::NotEqual, .. } => into_float(floatop(expr)),
    Expression::BinaryOp { lhs, op, rhs } => Expression::BinaryOp {
      lhs: Box::new(demand_float(*lhs)),
      op: op,
      rhs: Box::new(demand_float(*rhs))
    },
    Expression::LocalGet(_) => expr,
    Expression::GlobalGet(_) => expr,
    Expression::FunctionCall(index, exprs) => Expression::FunctionCall(index, exprs.into_iter().map(demand_float).collect()),
    Expression::NumericLiteral(_) => expr
  }
}

fn fix_statement(stmt: Statement) -> Statement {
  let fix_vec = |list: Vec<Statement>| list.into_iter().map(fix_statement).collect();
  match stmt {
    Statement::Loop(body) => Statement::Loop(fix_vec(body)),
    Statement::If { cond, then, otherwise } => Statement::If {
      cond: demand_int(cond),
      then: fix_vec(then),
      otherwise: fix_vec(otherwise)
    },
    Statement::LocalSet(index, expr) => Statement::LocalSet(index, demand_float(expr)),
    Statement::GlobalSet(index, expr) => Statement::GlobalSet(index, demand_float(expr)),
    Statement::Call(index, exprs) => Statement::Call(index, exprs.into_iter().map(demand_float).collect()),
    Statement::Return(expr) => Statement::Return(demand_float(expr)),
    Statement::Break => Statement::Break,
    Statement::Continue => Statement::Continue
  }
}

pub fn fix_types(program: AnalysisResults<Statement>) -> AnalysisResults<Statement> {
  AnalysisResults {
    global_variables: program.global_variables,
    funcname_map: program.funcname_map,
    functions: program.functions.into_iter().map(|(index, func)| {
      (index, Function {
        arguments: func.arguments,
        local_count: func.local_count,
        body: func.body.into_iter().map(|stmt| fix_statement(stmt)).collect()
      })
    }).collect()
  }
}