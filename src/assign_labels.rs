use crate::variable_analysis::{ AnalysisResults, Function, UnaryOp };
use crate::collapse_expressions::{ Instruction as UnlabaledInstruction };
use crate::polyfill_ops::{ BinaryOp };

#[derive(Debug, Clone)]
pub enum Instruction {
  Loop(Vec<Instruction>),
  Block(Vec<Instruction>),
  If {
    then: Vec<Instruction>,
    otherwise: Vec<Instruction>
  },
  Branch(u32),
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

fn add_labels(instr: UnlabaledInstruction, break_label: u32) -> Instruction {
  let map_one_level_deeper = |body: Vec<UnlabaledInstruction>| body.into_iter()
  .map(|inner_instr| add_labels(inner_instr, break_label + 1)).collect();
  match instr {
    UnlabaledInstruction::Loop(body) => {
      let mut body: Vec<Instruction> = body.into_iter().map(|inner_instr| add_labels(inner_instr, 0)).collect();
      body.push(Instruction::Branch(0));
      Instruction::Block(vec![
        Instruction::Loop(body)
      ])
    }
    UnlabaledInstruction::If { then, otherwise } => Instruction::If {
      then: map_one_level_deeper(then), otherwise: map_one_level_deeper(otherwise)
    },
    UnlabaledInstruction::Break => Instruction::Branch(break_label + 1),
    UnlabaledInstruction::Continue => Instruction::Branch(break_label),
    UnlabaledInstruction::Const(literal) => Instruction::Const(literal),
    UnlabaledInstruction::LocalGet(index) => Instruction::LocalGet(index),
    UnlabaledInstruction::LocalSet(index) => Instruction::LocalSet(index),
    UnlabaledInstruction::GlobalGet(index) => Instruction::GlobalGet(index),
    UnlabaledInstruction::GlobalSet(index) => Instruction::GlobalSet(index),
    UnlabaledInstruction::Call(index) => Instruction::Call(index),
    UnlabaledInstruction::Return() => Instruction::Return(),
    UnlabaledInstruction::Drop() => Instruction::Drop(),
    UnlabaledInstruction::UnOp(op) => Instruction::UnOp(op),
    UnlabaledInstruction::BinOp(op) => Instruction::BinOp(op)
  }
}

impl From<Function<UnlabaledInstruction>> for Function<Instruction> {
  fn from(func: Function<UnlabaledInstruction>) -> Self {
    let body = func.body.into_iter().map(|instr| add_labels(instr, 0)).collect();
    Self { arguments: func.arguments, local_count: func.local_count, body }
  }
}

impl AnalysisResults<Instruction> {
  pub fn from_unlabaled_analysis(analysis: AnalysisResults<UnlabaledInstruction>) -> Self {
    AnalysisResults {
      global_variables: analysis.global_variables,
      funcname_map: analysis.funcname_map,
      functions: analysis.functions.into_iter().map(|(k, v)| (k, v.into())).collect()
    }
  }
}

pub fn assign_labels(program: AnalysisResults<UnlabaledInstruction>) -> AnalysisResults<Instruction> {
  AnalysisResults::from_unlabaled_analysis(program)
}