use core::panic;

use crate::jsonlang::*;

#[derive(Debug, Clone)]
pub enum SimplifiedStatement {
  Loop(Box<SimplifiedStatement>),
  If {
    cond: Expression,
    then: Box<SimplifiedStatement>,
    otherwise: Box<SimplifiedStatement>
  },
  Break,
  Continue,
  Declare(String, Expression),
  Set(String, Expression),
  Call(String, Vec<Expression>),
  Return(Expression),
  Block(Vec<SimplifiedStatement>)
}

#[derive(Debug, Clone)]
pub enum SimplifiedTopStatement {
  Statement(SimplifiedStatement),
  DeclarationStatement {
    name: String,
    args: Vec<String>,
    content: SimplifiedStatement
  }
}

pub fn top_simplify_control_structures(program: JsonLang) -> Vec<SimplifiedTopStatement> {
  let mut statements = vec![];
  for statement in program.statements {
    statements.push(match statement {
      TopStatement::Statement(stmt) => SimplifiedTopStatement::Statement(simplify_control_structures(stmt)),
      TopStatement::DeclarationStatement(decl) => SimplifiedTopStatement::DeclarationStatement {
        name: decl.function, args: decl.args, content: simplify_control_structures(decl.block)
      }
    })
  }
  statements
}

pub fn simplify_control_structures(statement: Statement) -> SimplifiedStatement {
  match statement {
    Statement::If { branches, else_branch } => {
      let mut stmt = if let Some(branch) = else_branch {
        simplify_control_structures(*branch)
      } else {
        SimplifiedStatement::Block(vec![])
      };
      for if_case in branches.into_iter().rev() {
        stmt = SimplifiedStatement::If {
          cond: if_case.cond,
          then: Box::new(simplify_control_structures(*if_case.then)),
          otherwise: Box::new(stmt)
        }
      }
      stmt
    },
    Statement::While { condition, do_block } => {
      SimplifiedStatement::Loop(Box::new(SimplifiedStatement::If {
        cond: *condition,
        then: Box::new(simplify_control_structures(*do_block)),
        otherwise: Box::new(SimplifiedStatement::Break)
      }))
    },
    Statement::StatementList(stmt_list) => {
      let mut list = vec![];
      for stmt in stmt_list {
        list.push(simplify_control_structures(stmt))
      }
      SimplifiedStatement::Block(list)
    },
    Statement::Iterator { iterator, from, to, step, do_block } => {
      SimplifiedStatement::Block(vec![
        SimplifiedStatement::Declare(iterator.clone(), *from),
        SimplifiedStatement::Loop(Box::new(SimplifiedStatement::Block(vec![
          SimplifiedStatement::If {
            cond: Expression::BinaryOp { lhs: Box::new(Expression::VariableAccess(iterator.clone())), op: BinaryOp::Greater, rhs: to },
            then: Box::new(SimplifiedStatement::Break),
            otherwise: Box::new(simplify_control_structures(*do_block))
          },
          SimplifiedStatement::Set(iterator.clone(), Expression::BinaryOp {
            lhs: Box::new(Expression::VariableAccess(iterator.clone())),
            op: BinaryOp::Addition,
            rhs: if let Some(step) = step {
              step
            } else {
              Box::new(Expression::NumericLiteral(1.0))
            }
          })
        ])))
      ])
    },
    Statement::Until { do_block, until } => {
      SimplifiedStatement::Loop(Box::new(
        SimplifiedStatement::Block(vec![
          simplify_control_structures(*do_block),
          SimplifiedStatement::If {
            cond: *until,
            then: Box::new(SimplifiedStatement::Break),
            otherwise: Box::new(SimplifiedStatement::Block(vec![]))
          }
      ])))
    },
    Statement::Other(s) => {
      if s == "break" {
        SimplifiedStatement::Break
      } else if s == "continue" {
        SimplifiedStatement::Continue
      } else {
        panic!("Invalid string element.");
      }
    }
    Statement::Declare { declare, value } => SimplifiedStatement::Declare(declare, *value),
    Statement::Set { set, value } => SimplifiedStatement::Set(set, *value),
    Statement::Call { name, args } => SimplifiedStatement::Call(name, args),
    Statement::Return { return_value } => SimplifiedStatement::Return(*return_value),
  }
}