use ast::RootNode;
use lexer::Lexer;
use parser::{ParseResult, Parser};
use serde::{Deserialize, Serialize};
use span::Span;
use token::TokenKind;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod token;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenWithSpan {
    pub token: TokenKind,
    pub start_span: Span,
    pub finish_span: Span,
}
/// Tokenize the givn code
pub fn tokenize<'a>(code: &'a str) -> Vec<TokenWithSpan> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(code);
    loop {
        match lexer.get_token() {
            TokenKind::EOFToken => {
                break;
            }
            _ => {
                tokens.push(TokenWithSpan {
                    token: lexer.get_token(),
                    start_span: lexer.get_start_span(),
                    finish_span: lexer.get_finish_span(),
                });
                lexer.next_token();
            }
        }
    }
    tokens
}
/// Parse the givn code
pub fn parse<'a>(code: &'a str) -> ParseResult<RootNode<'a>> {
    let mut parser = Parser::new(code);
    parser.parse()
}
