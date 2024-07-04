mod selector;

use crate::ast::*;
use crate::lexer::ds::LexerCahce;
use crate::lexer::Lexer;
use crate::token::TokenKind;
use std::borrow::Cow;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

pub type ParseResult<T> = Result<T, String>;

pub type DefaultOkType = ();

macro_rules! unexpect_token_error {
    ($self: expr, $token: expr) => {
        return Err(format!(
            "[Error]: Got Unexpect Token {:?}, expect token {:?}",
            $self.get_token(),
            $token
        ));
    };
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            lexer: Lexer::new(code),
        }
    }
    fn get_token(&mut self) -> TokenKind {
        self.lexer.get_token()
    }
    fn next_token(&mut self) {
        self.lexer.next_token();
    }
    fn lookahead(&mut self, step: usize) -> LexerCahce {
        self.lexer.lookahead(step)
    }
    fn expect_token(&mut self, token: TokenKind) -> ParseResult<DefaultOkType> {
        if self.get_token() == token {
            self.next_token();
            return Ok(());
        }
        unexpect_token_error!(self, token);
    }
    fn expect_token_with_value(&mut self, token: TokenKind) -> ParseResult<Cow<'a, str>> {
        let value = self.lexer.cur_value();
        self.expect_token(token)?;
        Ok(value)
    }
    fn expect_context_keyword(&mut self, value: &'static str) -> ParseResult<DefaultOkType> {
        let token_value = self.lexer.cur_value();
        self.expect_token(TokenKind::Identifier)?;
        if token_value.as_ref() == value {
            return Ok(());
        }
        unexpect_token_error!(self, "");
    }
    fn match_token(&mut self, token: TokenKind) -> bool {
        self.get_token() == token
    }
    fn match_lookahead_context_keyword(&mut self, value: &'static str, step: usize) -> bool {
        let cache = self.lookahead(step);
        let start_offset = cache.start_span.offset;
        let finish_offset = cache.finish_span.offset;
        let lookahead_value = self
            .lexer
            .get_value_from_offset(start_offset, finish_offset);
        value == lookahead_value.as_ref()
    }

    /// Parser entry
    pub fn parse(&mut self) -> ParseResult<RootNode<'a>> {
        self.parse_root()
    }
    fn parse_root(&mut self) -> ParseResult<RootNode<'a>> {
        let mut instructions = Vec::new();
        loop {
            if self.match_token(TokenKind::ParenthesesLeftPunctuator) {
                instructions.push(self.parse_instruction()?);
            } else {
                break;
            }
        }
        Ok(RootNode { instructions })
    }
    fn parse_instruction(&mut self) -> ParseResult<InstructionNode<'a>> {
        self.expect_token(TokenKind::ParenthesesLeftPunctuator)?;
        let inst = match self.lexer.cur_value().as_ref() {
            "query" => {
                self.next_token();
                let name = self.parse_query_optional_name()?;
                let node = self.parse_description_node()?;
                Ok(InstructionNode::Query(QueryActionNode {
                    target_node: node,
                    name,
                }))
            }
            _ => panic!(),
        };
        self.expect_token(TokenKind::ParenthesesRightPunctuator)?;
        inst
    }
    fn parse_query_optional_name(&mut self) -> ParseResult<Option<Cow<'a, str>>> {
        println!("BEFROE {}", self.lexer.cur_value());
        if self.match_lookahead_context_keyword("name", 1) {
            println!("After {}", self.lexer.cur_value());
            self.expect_token(TokenKind::ParenthesesLeftPunctuator)?;
            println!("{:?} {:?}", self.get_token(), self.lexer.cur_value());
            self.next_token(); // eat name id
            println!("{:?}", self.get_token());
            let name = self.expect_token_with_value(TokenKind::Identifier)?;
            self.expect_token(TokenKind::ParenthesesRightPunctuator)?;
            Ok(Some(name))
        } else {
            Ok(None)
        }
    }
    fn parse_description_node(&mut self) -> ParseResult<DescriptionNode<'a>> {
        self.expect_token(TokenKind::ParenthesesLeftPunctuator)?;
        // parse key
        self.expect_context_keyword("node")?;
        // parse selector
        let mut selectors = Vec::new();
        loop {
            if self.match_token(TokenKind::ParenthesesLeftPunctuator) {
                selectors.push(self.parse_selector()?);
            } else {
                break;
            }
        }
        self.expect_token(TokenKind::ParenthesesRightPunctuator)?;
        Ok(DescriptionNode { selectors })
    }
    fn parse_literal(&mut self) -> ParseResult<Literal<'a>> {
        match self.get_token() {
            TokenKind::BoolLiteral => {
                let raw_value = self.lexer.cur_value();
                match raw_value.as_ref() {
                    "true" => {
                        self.next_token();
                        Ok(Literal::Bool(BoolLiteral {
                            raw_value,
                            value: true,
                        }))
                    }
                    "false" => {
                        self.next_token();
                        Ok(Literal::Bool(BoolLiteral {
                            raw_value,
                            value: false,
                        }))
                    }
                    _ => unreachable!(),
                }
            }
            TokenKind::StringLiteral => {
                let value = self.lexer.cur_value();
                self.next_token();
                Ok(Literal::String(StringLiteral { value }))
            }
            TokenKind::NumberLiteral => {
                todo!();
            }
            _ => {
                unexpect_token_error!(self, "");
            }
        }
    }
    fn parse_identifier(&mut self) -> ParseResult<Identifier<'a>> {
        let name = self.expect_token_with_value(TokenKind::Identifier)?;
        Ok(Identifier { name })
    }
}
