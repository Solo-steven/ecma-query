use query_parser::ast::RootNode;
use crate::codegen::BabelCodegen;
use std::fs::File;
use std::io::Write;
pub mod codegen;

pub fn generate_babel_visitor(path: &str, ast: &RootNode) {
    let mut codegen = BabelCodegen::new();
    let js_file_in_text = codegen.codegen(ast);

    let mut file = File::create(path).unwrap();
    write!(file, "{}", js_file_in_text).unwrap();
}