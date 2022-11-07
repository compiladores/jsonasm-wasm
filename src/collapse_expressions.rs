use crate::variable_analysis::{ AnalysisResults, Function, UnaryOp };
use crate::polyfill_ops::{ Statement, Expression, BinaryOp };

#[derive(Debug, Clone)]
pub enum Instruction {
  Loop(Vec<Instruction>),
  If {
    then: Vec<Instruction>,
    otherwise: Vec<Instruction>
  },
  Break,
  Continue,
  Const(f64),
  LocalGet(u32),
  LocalSet(u32),
  GlobalGet(u32),
  GlobalSet(u32),
  Call(u32),
  Return(),
  Drop(),
  UnOp(UnaryOp),
  BinOp(BinaryOp)
}

impl From<Function<Statement>> for Function<Instruction> {
  fn from(func: Function<Statement>) -> Self {
    let mut body = Vec::new();
    for stmt in func.body {
      body.append(&mut stmt.into());
    }
    Self { arguments: func.arguments, local_count: func.local_count, body }
  }
}

impl From<Statement> for Vec<Instruction> {
  fn from(stmt: Statement) -> Vec<Instruction> {
    let mut instrs = Vec::new();
    let mapbody = |body: Vec<Statement>| body.into_iter().map(Vec::<Instruction>::from).flatten().collect();
    let mut pushexpr = |expr: Expression| instrs.append(&mut expr.into());
    match stmt {
      Statement::Loop(body) => instrs.push(Instruction::Loop(mapbody(body))),
      Statement::If { cond, then, otherwise } => {
        pushexpr(cond);
        instrs.push(Instruction::If { then: mapbody(then), otherwise: mapbody(otherwise) })
      }
      Statement::Break => instrs.push(Instruction::Break),
      Statement::Continue => instrs.push(Instruction::Continue),
      Statement::LocalSet(index, expr) => {
        pushexpr(expr);
        instrs.push(Instruction::LocalSet(index));
      },
      Statement::GlobalSet(index, expr) => {
        pushexpr(expr);
        instrs.push(Instruction::GlobalSet(index));
      },
      Statement::Call(index, args) => {
        args.into_iter().for_each(|expr| pushexpr(expr));
        instrs.push(Instruction::Call(index));
        instrs.push(Instruction::Drop()); // Discard return value
      }
      Statement::Return(expr) => {
        pushexpr(expr);
        instrs.push(Instruction::Return());
      }
    }
    instrs
  }
}

impl From<Expression> for Vec<Instruction> {
  fn from(expr: Expression) -> Vec<Instruction> {
    let mut instrs = Vec::new();
    let mut pushexpr = |expr: Expression| instrs.append(&mut expr.into());
    match expr {
      Expression::UnaryOp { op, arg } => {
        pushexpr(*arg);
        instrs.push(Instruction::UnOp(op));
      }
      Expression::BinaryOp { lhs, op, rhs } => {
        pushexpr(*lhs);
        pushexpr(*rhs);
        instrs.push(Instruction::BinOp(op));
      }
      Expression::LocalGet(index) => instrs.push(Instruction::LocalGet(index)),
      Expression::GlobalGet(index) => instrs.push(Instruction::GlobalGet(index)),
      Expression::FunctionCall(index, args) => {
        args.into_iter().for_each(|expr| pushexpr(expr));
        instrs.push(Instruction::Call(index));
      }
      Expression::NumericLiteral(literal) => instrs.push(Instruction::Const(literal))
    }
    instrs
  }
}

impl AnalysisResults<Instruction> {
  pub fn from_statement_analysis(analysis: AnalysisResults<Statement>) -> Self {
    AnalysisResults {
      global_variables: analysis.global_variables,
      funcname_map: analysis.funcname_map,
      functions: analysis.functions.into_iter().map(|(k, v)| (k, v.into())).collect()
    }
  }
}

pub fn collapse_expressions(program: AnalysisResults<Statement>) -> AnalysisResults<Instruction> {
  AnalysisResults::from_statement_analysis(program)
}
