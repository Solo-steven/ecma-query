use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    Start,
    ParenthesesLeftPunctuator,  // (
    ParenthesesRightPunctuator, // )
    Identifier,
    NumberLiteral,
    StringLiteral,
    BoolLiteral,
    EOFToken,
}
