mod jsonlang;
mod control_simplification;
mod variable_analysis;
mod collapse_expressions;
mod assign_labels;
mod codegen;
mod fixup_types;
mod polyfill_ops;
mod cordic;
use jsonlang::*;
use control_simplification::top_simplify_control_structures;
use variable_analysis::top_analyze_variables;
use collapse_expressions::collapse_expressions;
use assign_labels::assign_labels;
use codegen::emit_wasm;
use fixup_types::fix_types;
use polyfill_ops::polyfill_ops;

use std::fs;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let p: JsonLang = serde_json::from_str(&fs::read_to_string(&args[1]).expect("Couldn't read input")).unwrap();
  //print!("{:#?}", serde_json::to_string(&fix_types(top_analyze_variables(top_simplify_control_structures(p)))).unwrap());
  let wasm = emit_wasm(assign_labels(collapse_expressions(
    polyfill_ops(fix_types(top_analyze_variables(top_simplify_control_structures(p))))
  )));
  fs::write(&args[2], wasm).expect("Couldn't write to output");
}
