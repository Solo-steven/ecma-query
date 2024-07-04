use query_parser::{lexer::Lexer, parser::Parser, token::TokenKind};

fn main() {
    let code_2 = "
    (let openElement = GetOpenElement)
    

    ";
    let code = "
    (query 
        (name, 'GetOpenElement')
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
                println!("{:?}", lexer.get_token())
            }
        }
        lexer.next_token();
    }
    let mut parser = Parser::new(code);

    println!("{:?}", parser.parse());
}

