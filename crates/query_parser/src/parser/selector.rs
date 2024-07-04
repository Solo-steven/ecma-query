use super::{ParseResult, Parser};
use crate::ast::*;
use crate::token::TokenKind;
use std::borrow::Cow;

impl<'a> Parser<'a> {
    pub(super) fn parse_selector(&mut self) -> ParseResult<Selector<'a>> {
        self.expect_token(TokenKind::ParenthesesLeftPunctuator)?;
        let result = if let TokenKind::Identifier = self.get_token() {
            match self.lexer.cur_value().as_ref() {
                "parent" => {
                    self.next_token();
                    Ok(Selector::Parent(self.parse_values_of_parent_selector()?))
                }
                "ancestor" => {
                    self.next_token();
                    Ok(Selector::Ancestor(
                        self.parse_values_of_ancestor_selector()?,
                    ))
                }
                _ => {
                    let key = self.lexer.cur_value();
                    self.next_token();
                    match self.get_token() {
                        TokenKind::NumberLiteral
                        | TokenKind::BoolLiteral
                        | TokenKind::StringLiteral => Ok(Selector::Literal(LiteralSelector {
                            key,
                            literal: self.parse_literal()?,
                        })),
                        TokenKind::ParenthesesLeftPunctuator => Ok(Selector::Recursive(
                            self.parse_values_of_recursive_node(key)?,
                        )),
                        TokenKind::Identifier => match self.lexer.cur_value().as_ref() {
                            "array" => {
                                self.next_token();
                                Ok(Selector::Array(self.parse_values_of_array_selector(key)?))
                            }
                            _ => Err(String::from("[ERROR]: Unexpect")),
                        },
                        _ => Err(String::from("[ERROR]: Unexpect")),
                    }
                }
            }
        } else {
            // should
            Err(String::from("[ERROR]: unfinsh"))
        };
        self.expect_token(TokenKind::ParenthesesRightPunctuator)?;
        result
    }
    fn parse_values_of_parent_selector(&mut self) -> ParseResult<ParentSelector<'a>> {
        let key = self.expect_token_with_value(TokenKind::Identifier)?;
        let node = self.parse_description_node()?;
        Ok(ParentSelector {
            key,
            target_node: node,
        })
    }
    fn parse_values_of_ancestor_selector(&mut self) -> ParseResult<AncestorSelector<'a>> {
        let key = self.expect_token_with_value(TokenKind::Identifier)?;
        let node = self.parse_description_node()?;
        Ok(AncestorSelector {
            key,
            target_node: node,
        })
    }
    fn parse_values_of_recursive_node(
        &mut self,
        key: Cow<'a, str>,
    ) -> ParseResult<RecursiveSelector<'a>> {
        let node = self.parse_description_node()?;
        Ok(RecursiveSelector {
            key,
            target_node: node,
        })
    }
    fn parse_values_of_array_selector(
        &mut self,
        key: Cow<'a, str>,
    ) -> ParseResult<ArraySelector<'a>> {
        match self.get_token() {
            TokenKind::ParenthesesLeftPunctuator => {
                let node = self.parse_description_node()?;
                Ok(ArraySelector {
                    key,
                    value: LiteralOrNode::Node(node),
                })
            }
            _ => {
                let literal = self.parse_literal()?;
                Ok(ArraySelector {
                    key,
                    value: LiteralOrNode::Lit(literal),
                })
            }
        }
    }
}
