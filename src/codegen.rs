use std::collections::HashMap;

use crate::variable_analysis::{AnalysisResults, Function, UnaryOp };
use crate::assign_labels::Instruction;
use crate::polyfill_ops::{ BinaryOp };

struct WASMEmitter {
  indent: u32,
  emitted: String
}

impl WASMEmitter {
  pub fn new() -> Self {
    WASMEmitter {
      indent: 0,
      emitted: String::new()
    }
  }
  fn emit_str(&mut self, str: &str) {
    self.emitted.push_str(str);
  }
  fn emit_line(&mut self, str: &str) {
    self.emitted.push('\n');
    for _ in 0..self.indent { self.emitted.push(' '); }
    self.emit_str(str);
  }
  pub fn emit_program(program: AnalysisResults<Instruction>) -> String {
    let mut emitter = Self::new();
    let mut funclist = Vec::new();
    let mut functions = program.functions;
    for i in 0..functions.len() {
      funclist.push(functions.remove(&(i as u32)).unwrap())
    }
    emitter.emit_line("(module");
    emitter.indent += 2;
    emitter.emit_types(&funclist);
    emitter.emit_functions(&funclist);
    emitter.emit_globals(program.global_variables.count());
    emitter.emit_global_exports(program.global_variables.list_variables());
    emitter.emit_func_exports(&program.funcname_map);
    emitter.indent -= 2;
    emitter.emit_line(")");
    emitter.emitted
  }
  fn emit_globals(&mut self, count: u32) {
    for _ in 0..count {
      self.emit_line("(global (mut f64) (f64.const 0))");
    }
  }
  fn emit_global_exports(&mut self, globals: Vec<(&String, &u32)>) {
    for (name, index) in globals {
      self.emit_line(&format!(r#"(export "{}" (global {}))"#, name, index));
    }
  }
  fn emit_func_exports(&mut self, funcnames: &HashMap<String, u32>) {
    for (name, index) in funcnames {
      self.emit_line(&format!(r#"(export "{}" (func {}))"#, name, index));
    }
  }
  fn emit_types(&mut self, funclist: &Vec<Function<Instruction>>) {
    for func in funclist.iter() {
      self.emit_line("(type (func");
      self.emit_func_type(func);
      self.emit_str("))");
    }
  }
  fn emit_func_type(&mut self, func: &Function<Instruction>) {
    if func.arguments != 0 {
      self.emit_str(" (param");
      for _ in 0..func.arguments {
        self.emit_str(" f64")
      }
      self.emit_str(")");
    }
    self.emit_str(" (result f64)");
  }
  fn emit_functions(&mut self, funclist: &Vec<Function<Instruction>>) {
    let mut index = 0;
    for func in funclist {
      self.emit_function(func, index);
      index += 1;
    }
  }
  fn emit_locals(&mut self, func: &Function<Instruction>) {
    if func.local_count != 0 {
      self.emit_str("(local");
      for _ in 0..func.local_count {
        self.emit_str(" f64")
      }
      self.emit_str(")");
    }
  }
  fn emit_function(&mut self, func: &Function<Instruction>, type_index: u32) {
    self.emit_line(&format!("(func (type {})", type_index));
    self.emit_func_type(func);
    self.emit_locals(func);
    self.indent += 2;
    for instr in func.body.iter() {
      self.emit_instruction(instr);
    }
    self.emit_instruction(&Instruction::Const(0.0));
    self.emit_instruction(&Instruction::Return()); // Force value return if none applies
    self.indent -= 2;
    self.emit_line(")");
  }
  fn emit_instruction(&mut self, instr: &Instruction) {
    let emit_body = |s: &mut Self, body: &Vec<Instruction>| {
      s.indent += 2;
      body.iter().for_each(|ins| s.emit_instruction(ins));
      s.indent -= 2;
    };
    match instr {
      Instruction::Loop(body) => {
        self.emit_line("loop");
        emit_body(self, body);
        self.emit_line("end");
      }
      Instruction::Block(body) => {
        self.emit_line("block");
        emit_body(self, body);
        self.emit_line("end");
      }
      Instruction::If { then, otherwise } => {
        self.emit_line("if");
        emit_body(self, then);
        self.emit_line("else");
        emit_body(self, otherwise);
        self.emit_line("end");
      }
      Instruction::Branch(index) => self.emit_line(&format!("br {}", index)),
      Instruction::Const(literal) => self.emit_line(&format!("f64.const {}", literal)),
      Instruction::LocalGet(index) => self.emit_line(&format!("local.get {}", index)),
      Instruction::LocalSet(index) => self.emit_line(&format!("local.set {}", index)),
      Instruction::GlobalGet(index) => self.emit_line(&format!("global.get {}", index)),
      Instruction::GlobalSet(index) => self.emit_line(&format!("global.set {}", index)),
      Instruction::Call(index) => self.emit_line(&format!("call {}", index)),
      Instruction::Return() => self.emit_line("return"),
      Instruction::Drop() => self.emit_line("drop"),
      Instruction::UnOp(UnaryOp::NumericNegation) => self.emit_line("f64.neg"),
      Instruction::UnOp(UnaryOp::LogicNegation) => self.emit_line("i32.eqz"),
      Instruction::UnOp(UnaryOp::BitwiseNegation) => {
        self.emit_line("i32.const 2147483647");
        self.emit_line("i32.xor");
      },
      Instruction::UnOp(UnaryOp::IntToFloat) => self.emit_line("f64.convert_i32_u"),
      Instruction::UnOp(UnaryOp::FloatToInt) => self.emit_line("i32.trunc_f64_u"),
      Instruction::UnOp(UnaryOp::Sqrt) => self.emit_line("f64.sqrt"),
      Instruction::UnOp(UnaryOp::Floor) => self.emit_line("f64.floor"),
      Instruction::BinOp(BinaryOp::Addition) => self.emit_line("f64.add"),
      Instruction::BinOp(BinaryOp::Substraction) => self.emit_line("f64.sub"),
      Instruction::BinOp(BinaryOp::Multiplication) => self.emit_line("f64.mul"),
      Instruction::BinOp(BinaryOp::Division) => self.emit_line("f64.div"),
      Instruction::BinOp(BinaryOp::IntAddition) => self.emit_line("i32.add"),
      Instruction::BinOp(BinaryOp::IntSubstraction) => self.emit_line("i32.sub"),
      Instruction::BinOp(BinaryOp::IntMultiplication) => self.emit_line("i32.mul"),
      Instruction::BinOp(BinaryOp::IntDivision) => self.emit_line("i32.div"),
      Instruction::BinOp(BinaryOp::Lesser) => self.emit_line("f64.lt"),
      Instruction::BinOp(BinaryOp::LessEq) => self.emit_line("f64.le"),
      Instruction::BinOp(BinaryOp::Greater) => self.emit_line("f64.gt"),
      Instruction::BinOp(BinaryOp::GreaterEq) => self.emit_line("f64.ge"),
      Instruction::BinOp(BinaryOp::NotEqual) => self.emit_line("f64.ne"),
      Instruction::BinOp(BinaryOp::Equal) => self.emit_line("f64.eq"),
      Instruction::BinOp(BinaryOp::BitwiseAnd) => self.emit_line("i32.and"),
      Instruction::BinOp(BinaryOp::BitwiseOr) => self.emit_line("i32.or"),
      Instruction::BinOp(BinaryOp::LeftShift) => self.emit_line("i32.shl"),
      Instruction::BinOp(BinaryOp::RightShift) => self.emit_line("i32.shr_u"),
      Instruction::BinOp(BinaryOp::Modulo) => self.emit_line("i32.rem_u"),
    }
  }
}

pub fn emit_wasm(program: AnalysisResults<Instruction>) -> String {
  WASMEmitter::emit_program(program)
}