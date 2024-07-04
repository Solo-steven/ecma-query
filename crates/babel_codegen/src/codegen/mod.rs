use query_parser::ast::RootNode;
use std::collections::{HashMap, HashSet};
mod file_generator;
mod visitor;

pub struct BabelCodegen {
    // babel_code: String,
    /// when there is a node descripion, we will need to create a function
    /// for validate that node with schema
    /// - key: function name for that node description.
    /// - value: js function accept a node and return a bool.
    cache_functions: HashMap<String, String>,
    /// type of target node in any query mean our visitor need to have
    /// that type of node in visitor object
    /// - key: type of ast node
    /// - value: all the function needs to call
    queried_ast_nodes: HashMap<String, HashSet<String>>,
}

impl<'a> BabelCodegen {
    pub fn new() -> Self {
        Self {
            // babel_code: String::new(),
            cache_functions: Default::default(),
            queried_ast_nodes: Default::default(),
        }
    }
    pub fn codegen(&mut self, ast: &RootNode) -> String {
        self.visit_root_node(ast);
        self.generate_file_js()
    }
}
