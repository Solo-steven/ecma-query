use crate::codegen::BabelCodegen;
use query_parser::ast::RootNode;
pub mod codegen;

pub fn generate_babel_visitor(ast: &RootNode) -> String {
    let mut codegen = BabelCodegen::new();
    let js_file_in_text = codegen.codegen(ast);
    js_file_in_text
}
