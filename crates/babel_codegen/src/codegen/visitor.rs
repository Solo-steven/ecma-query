
use std::collections::{HashMap, HashSet};
use query_parser::ast::{DescriptionNode, InstructionNode, Literal, LiteralOrNode, QueryActionNode, RootNode, Selector};
use super::BabelCodegen;

impl BabelCodegen {
    pub (super) fn visit_root_node(&mut self, node: &RootNode) {
        for inst in &node.instructions {
            self.visit_inst_node(inst);    
        }
    }
    fn visit_inst_node(&mut self, node: &InstructionNode) {
        match node {
            InstructionNode::Query(query_node) => self.visit_query_node(query_node)
        }
    }
    fn visit_query_node(&mut self, query_node: &QueryActionNode) {
        let node_type = {
            let mut ast_type = None;
            for selector in &query_node.target_node.selectors {
                if let Selector::Literal(literal) = selector {
                    if literal.key.as_ref() == "type" {
                        if let Literal::String(string_literal) = &literal.literal {
                            ast_type = Some(string_literal.value.to_string());
                        }
                    }
                }
            }
            if let Some(ast_type_str) = ast_type {
                ast_type_str
            }else {
                panic!()
            }
        };
        let func_name = self.visit_node_description(&query_node.target_node);

        if let Some(set) = self.queried_ast_nodes.get_mut(&node_type) {
            set.insert(func_name);
        }else {
            let mut set = HashSet::new();
            set.insert(func_name);
            self.queried_ast_nodes.insert(node_type, set);
        }

    }
    fn visit_node_description(&mut self, node: &DescriptionNode) -> String {
        let function_name = format!("testASTNode_{}", self.cache_functions.len());
        self.cache_functions.insert(function_name.clone(), Default::default());
        let mut js_test_expression_map = HashMap::new();
        for selector in &node.selectors {
            match selector {
                Selector::Literal(literal_selector) => {
                    let key = literal_selector.key.to_string();
                    let js_literal_expression = match &literal_selector.literal {
                        Literal::Bool(bool_literal) => {
                            format!("node.{} === {} ", key, bool_literal.raw_value)
                        }
                        Literal::String(string_literal) => {
                            format!("node.{} === {} ", key, string_literal.value)
                        }
                        Literal::Number(number_literal) => {
                            format!("node.{} === {} ", key, number_literal.raw_value)
                        }
                    };
                    js_test_expression_map.insert(key, js_literal_expression);
                }
                Selector::Recursive(node) => {
                    let function_name = self.visit_node_description(&node.target_node);
                    js_test_expression_map.insert(node.key.to_string(), format!("{}(node.{})", function_name, node.key));
                }
                Selector::Array(selector) => {
                    match &selector.value {
                        LiteralOrNode::Node(node) =>  {
                            let function_name = self.visit_node_description(&node);
                            js_test_expression_map.insert(selector.key.to_string(), format!("Array.isArray(node.{}) && node.{}.some({})", selector.key, selector.key, function_name));
                        }
                        LiteralOrNode::Lit(literal) => {
                            todo!()
                        }
                    }
                }
                _ => todo!()
            }
        }
        let mut check_all_condition = String::new();
        let mut index = 0;
        for (_key, expr) in &js_test_expression_map {
            let operator = if index != 0 { " && " } else { "" };
            check_all_condition.push_str(format!("{}{}", operator,  expr).as_str());
            index += 1;
        }
        check_all_condition.push(';');
        self.cache_functions.insert(function_name.clone(), format!("function {}(node) {{ return node && {}  }}", function_name, check_all_condition));
        function_name
    }
}