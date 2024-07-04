use super::BabelCodegen;

impl BabelCodegen {
    pub(super) fn generate_file_js(&mut self) -> String {
        let mut js_file = String::new();
        for (_, fun) in &self.cache_functions {
            js_file.push_str(fun);
            js_file.push('\n');
        }
        let mut js_visitor_object = String::from("{\n");
        for (ast_type, funcs) in &self.queried_ast_nodes {
            js_visitor_object.push_str(format!("\t\t\t{}(path) {{\n", ast_type).as_str());
            js_visitor_object.push_str("\t\t\t\tconst node = path.node;\n");
            for fun_name in funcs {
                js_visitor_object
                    .push_str(format!("\t\t\t\tif({}(node)) {{}}\n", fun_name).as_str());
            }
            js_visitor_object.push_str("\t\t\t}\n");
        }
        js_visitor_object.push_str("\t\t}\n");
        js_file.push_str(
            format!(
                "export default function() {{\n\treturn {{\n\t\tvisitor: {}\t}}\n}}\n",
                js_visitor_object
            )
            .as_str()
            .trim(),
        );
        js_file
    }
}
