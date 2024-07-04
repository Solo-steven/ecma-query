use crate::token::TokenKind;

use super::Lexer;

impl <'a> Lexer<'a> {
    pub (super) fn read_idenfier(&mut self){
        loop {
            match self.get_char() {
                None => break,
                Some(ch) => {
                    match ch {
                        'A'..='Z' | 'a'..='z' | '-' | '_' => {
                            self.eat_char();
                            continue;
                        }
                        ' ' | '\t' | '\n' => break,
                        '(' | ')' => break,
                        _ => {
                            panic!("[ERROR]: Unexpect char {:?}", ch);
                        }
                    }
                }
            }
        }
        self.finish_token(TokenKind::Identifier);
    }
    pub (super) fn read_string(&mut self, end_char: char) {
        self.eat_char();
        loop {
            match self.get_char() {
                None => {
                    panic!()
                },
                Some(ch) => {
                    if ch == end_char {
                        self.finish_token_with_eat(TokenKind::StringLiteral);
                        break;
                    }
                    self.eat_char();
                    continue;
                }
            }
        }
    }
}