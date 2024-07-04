use crate::span::Span;
use crate::token::TokenKind;
/// ! Data structure for Lexer
use std::str::CharIndices;

pub struct LexerCursor<'a> {
    pub(super) cur_char: Option<char>,
    pub(super) source: &'a str,
    pub(super) iter: CharIndices<'a>,
    pub(super) offset: usize,
    pub(super) cur_line: usize,
    pub(super) cur_line_start: usize,
}
impl<'a> LexerCursor<'a> {
    pub fn new(
        source: &'a str,
        cur_char: Option<char>,
        iter: CharIndices<'a>,
        offset: usize,
    ) -> Self {
        Self {
            cur_char,
            source,
            iter,
            offset,
            cur_line: 0,
            cur_line_start: offset,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct LexerCahce {
    pub token: TokenKind,
    pub start_span: Span,
    pub finish_span: Span,
}
impl LexerCahce {
    pub fn new(token: TokenKind) -> Self {
        Self {
            token,
            start_span: Span::new_empty(),
            finish_span: Span::new_empty(),
        }
    }
}
