use babel_codegen::generate_babel_visitor;
use query_parser::parse;

fn main() {
    let ast = parse(
        "
    (query 
        (node 
            (type 'JSXOpenElement')
            (attributes array (node 
                (type 'JSXAttribute')
                (name (node 
                    (type 'JSXIdentifier')
                    (name 'className')
                ))
            ))
        )
    )",
    )
    .unwrap();
    generate_babel_visitor(&ast);
}
