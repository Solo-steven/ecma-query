#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Start,
    ParenthesesLeftPunctuator, // (
    ParenthesesRightPunctuator, // )
    Identifier,
    NumberLiteral,
    StringLiteral,
    BoolLiteral,
    EOFToken
}