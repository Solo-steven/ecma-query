pub mod ds;
mod literal;
use std::borrow::Cow;
use std::mem::replace;

use crate::{span::Span, token::TokenKind};
use ds::{LexerCahce, LexerCursor};

pub struct Lexer<'a> {
    cursor: LexerCursor<'a>,
    cache: LexerCahce,
    lookahead_buffer: Vec<LexerCahce>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut iter = source.char_indices();
        match iter.next() {
            Some((index, ch)) => {
                let mut lexer = Lexer {
                    cursor: LexerCursor::new(source, Some(ch), iter, index),
                    cache: LexerCahce::new(TokenKind::Start),
                    lookahead_buffer: Vec::with_capacity(10),
                };
                lexer.next_token();
                lexer
            }
            None => Lexer {
                cursor: LexerCursor::new(source, None, iter, 0),
                cache: LexerCahce::new(TokenKind::EOFToken),
                lookahead_buffer: Vec::new(),
            },
        }
    }
    fn get_char(&mut self) -> Option<char> {
        self.cursor.cur_char.clone()
    }
    fn eat_char(&mut self) {
        match self.cursor.iter.next() {
            Some((index, char)) => {
                self.cursor.offset = index;
                self.cursor.cur_char = Some(char);
            }
            None => {
                self.cursor.cur_char = None;
            }
        }
    }
    fn eat_change_line_char(&mut self) {
        self.eat_char();
        self.cursor.cur_line += 1;
        self.cursor.cur_line_start = self.cursor.offset;
    }
    fn start_token(&mut self) {
        self.cache.start_span = Span {
            offset: self.cursor.offset,
            line: self.cursor.cur_line,
            col: self.cursor.offset - self.cursor.cur_line_start,
        }
    }
    fn finish_token(&mut self, token: TokenKind) {
        self.cache.token = token;
        self.cache.finish_span = Span {
            offset: self.cursor.offset,
            line: self.cursor.cur_line,
            col: self.cursor.offset - self.cursor.cur_line_start,
        }
    }
    fn finish_token_with_eat(&mut self, token: TokenKind) {
        self.eat_char();
        self.cache.token = token;
        self.cache.finish_span = Span {
            offset: self.cursor.offset,
            line: self.cursor.cur_line,
            col: self.cursor.offset - self.cursor.cur_line_start,
        }
    }
    pub fn get_token(&self) -> TokenKind {
        self.cache.token.clone()
    }
    pub fn next_token(&mut self) {
        if let Some(next_token_cache) = self.lookahead_buffer.pop() {
            self.cache = next_token_cache;
        } else {
            self.scan();
        }
    }
    pub fn lookahead(&mut self, step: usize) -> LexerCahce {
        if self.lookahead_buffer.len() < step {
            let current_cache = self.cache.clone();
            let step_need_to_move = step - self.lookahead_buffer.len();
            for _ in 0..step_need_to_move {
                self.next_token();
                let next_token_cache = self.cache.clone();
                self.lookahead_buffer.push(next_token_cache);
            }
            let target_cache = replace(&mut self.cache, current_cache);
            target_cache
        } else {
            self.lookahead_buffer[step].clone()
        }
    }
    pub fn get_start_span(&self) -> Span {
        self.cache.start_span.clone()
    }
    pub fn get_finish_span(&self) -> Span {
        self.cache.finish_span.clone()
    }
    pub fn get_value_from_offset(&self, start_offset: usize, finish_offset: usize) -> Cow<'a, str> {
        Cow::Borrowed(&self.cursor.source[start_offset..finish_offset])
    }
    pub fn cur_value(&self) -> Cow<'a, str> {
        Cow::Borrowed(
            &self.cursor.source[self.cache.start_span.offset..self.cache.finish_span.offset],
        )
    }
    fn skip_change_line_and_space(&mut self) {
        loop {
            if let Some(ch) = self.get_char() {
                match ch {
                    ' ' | '\t' => {
                        self.eat_char();
                        continue;
                    }
                    '\n' => {
                        self.eat_change_line_char();
                        continue;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
    }
    fn scan(&mut self) {
        self.skip_change_line_and_space();
        self.start_token();
        match self.get_char() {
            None => self.finish_token(TokenKind::EOFToken),
            Some(ch) => match ch {
                '(' => self.finish_token_with_eat(TokenKind::ParenthesesLeftPunctuator),
                ')' => self.finish_token_with_eat(TokenKind::ParenthesesRightPunctuator),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => todo!(),
                '\'' | '\"' => self.read_string(ch),
                _ => self.read_idenfier(),
            },
        };
    }
}
