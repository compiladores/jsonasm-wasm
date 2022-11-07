use core::panic;

use crate::variable_analysis::{ AnalysisResults, Function, UnaryOp };
use crate::polyfill_ops::{ Statement, Expression };

use crate::control_simplification::top_simplify_control_structures;
use crate::variable_analysis::top_analyze_variables;
use crate::fixup_types::fix_types;
use crate::polyfill_ops::polyfill_ops;

pub fn generate_exp_polyfills(base_index: u32) -> (Function<Statement>, Function<Statement>, Function<Statement>, Function<Statement>) {
  let p = serde_json::from_str(include_str!("cordic.lang.json")).unwrap();
  let store: AnalysisResults<Statement> = polyfill_ops(fix_types(top_analyze_variables(top_simplify_control_structures(p))));
  println!("{:#?}", store);
  let pow_index = *store.funcname_map.get("pow").unwrap();
  let lut_index = *store.funcname_map.get("cordic_lut").unwrap();
  let ln_index = *store.funcname_map.get("ln_cordic").unwrap();
  let exp_index = *store.funcname_map.get("exp_cordic").unwrap();
  let floor_index = *store.funcname_map.get("floor").unwrap();
  (
    replace_indexes(base_index, lut_index, ln_index, exp_index, floor_index, store.functions.get(&pow_index).unwrap()),
    store.functions.get(&lut_index).unwrap().clone(),
    replace_indexes(base_index, lut_index, ln_index, exp_index, floor_index, store.functions.get(&ln_index).unwrap()),
    replace_indexes(base_index, lut_index, ln_index, exp_index, floor_index, store.functions.get(&exp_index).unwrap())
  )
}

pub fn replace_indexes(base_index: u32, lut_index: u32, ln_index: u32, exp_index: u32, floor_index: u32, func: &Function<Statement>) -> Function<Statement> {
  let repl_ind = |stmt| replace_indexes_stmt(base_index, lut_index, ln_index, exp_index, floor_index, stmt);
  Function {
    arguments: func.arguments,
    local_count: func.local_count,
    body: func.body.clone().into_iter().map(repl_ind).collect()
  }
}

pub fn replace_indexes_stmt(base_index: u32, lut_index: u32, ln_index: u32, exp_index: u32, floor_index: u32, stmt: Statement) -> Statement {
  let repl = |stmt| replace_indexes_stmt(base_index, lut_index, ln_index, exp_index, floor_index, stmt);
  let map_vec = |vec: Vec<Statement>| vec.into_iter().map(repl).collect();
  let repl_expr = |expr| replace_indexes_expr(base_index, lut_index, ln_index, exp_index, floor_index, expr);
  match stmt {
    Statement::Loop(s) => Statement::Loop(map_vec(s)),
    Statement::If { cond, then, otherwise } => Statement::If {
      cond: repl_expr(cond),
      then: map_vec(then), otherwise: map_vec(otherwise)
    },
    Statement::LocalSet(index, expr) => Statement::LocalSet(index, repl_expr(expr)),
    Statement::GlobalSet(_, _) => panic!("shouldn't be in file"),
    Statement::Call(_, _) => panic!("shouldn't be in file"),
    Statement::Return(expr) => Statement::Return(repl_expr(expr)),
    _ => stmt
  }
}

pub fn replace_indexes_expr(base_index: u32, lut_index: u32, ln_index: u32, exp_index: u32, floor_index: u32, expr: Expression) -> Expression {
  let repl_expr = |e| replace_indexes_expr(base_index, lut_index, ln_index, exp_index, floor_index, e);
  let repl_box = |e: Box<Expression>| Box::new(repl_expr(*e));
  match expr {
    Expression::UnaryOp { op, arg } => Expression::UnaryOp { op, arg: repl_box(arg) },
    Expression::BinaryOp { lhs, op, rhs } => Expression::BinaryOp {
      lhs: repl_box(lhs), op, rhs: repl_box(rhs)
    },
    Expression::FunctionCall(index, args) => if index == floor_index {
      Expression::UnaryOp { op: UnaryOp::Floor, arg: Box::new(repl_expr(args.get(0).expect("invalid floor call").clone())) }
    } else {
      let fixed_index = if index == lut_index { 
        base_index + 1
      } else if index == ln_index {
        base_index + 2
      } else if index == exp_index {
        base_index + 3
      } else {
        panic!("Unexpected call");
      };
      Expression::FunctionCall(fixed_index, args.into_iter().map(repl_expr).collect())
    }
    Expression::GlobalGet(_) => panic!("shouldn't be in file"),
    _ => expr
  }
}