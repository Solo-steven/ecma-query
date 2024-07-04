use query_parser::{lexer::Lexer, parser::Parser, token::TokenKind};

fn main() {
    let code = "
    (query 
        (name GetOpenElement)
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
    )";
    let mut lexer = Lexer::new(code);
    loop {
        match lexer.get_token() {
            TokenKind::EOFToken => break,
            _ => {
                println!("{:?} {:?}", lexer.get_token(), lexer.cur_value())
            }
        }
        lexer.next_token();
    }
    let mut parser = Parser::new(code);

    println!("{:?}", parser.parse());
}
