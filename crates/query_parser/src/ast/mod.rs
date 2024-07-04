use serde::{Deserialize, Serialize};
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Identifier<'a> {
    pub name: Cow<'a, str>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Literal<'a> {
    #[serde(rename = "StringLiteral")]
    String(StringLiteral<'a>),
    #[serde(rename = "NumberLiteral")]
    Number(NumberLiteral<'a>),
    #[serde(rename = "BoolLiteral")]
    Bool(BoolLiteral<'a>),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StringLiteral<'a> {
    pub value: Cow<'a, str>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BoolLiteral<'a> {
    pub raw_value: Cow<'a, str>,
    pub value: bool,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct NumberLiteral<'a> {
    pub raw_value: Cow<'a, str>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct RootNode<'a> {
    pub instructions: Vec<InstructionNode<'a>>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum InstructionNode<'a> {
    #[serde(rename = "QueryCommand")]
    Query(QueryActionNode<'a>),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct QueryActionNode<'a> {
    pub name: Option<Cow<'a, str>>,
    pub target_node: DescriptionNode<'a>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct DescriptionNode<'a> {
    pub selectors: Vec<Selector<'a>>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Selector<'a> {
    #[serde(rename = "ParentSelector")]
    Parent(ParentSelector<'a>),
    #[serde(rename = "AncestorSelector")]
    Ancestor(AncestorSelector<'a>),
    #[serde(rename = "LiteralSelector")]
    Literal(LiteralSelector<'a>),
    #[serde(rename = "RecursiveSelector")]
    Recursive(RecursiveSelector<'a>),
    #[serde(rename = "ArraySelector")]
    Array(ArraySelector<'a>),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ParentSelector<'a> {
    pub key: Cow<'a, str>,
    pub target_node: DescriptionNode<'a>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AncestorSelector<'a> {
    pub key: Cow<'a, str>,
    pub target_node: DescriptionNode<'a>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LiteralSelector<'a> {
    pub key: Cow<'a, str>,
    pub literal: Literal<'a>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RecursiveSelector<'a> {
    pub key: Cow<'a, str>,
    pub target_node: DescriptionNode<'a>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LiteralOrNode<'a> {
    Lit(Literal<'a>),
    Node(DescriptionNode<'a>),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArraySelector<'a> {
    pub key: Cow<'a, str>,
    pub value: LiteralOrNode<'a>,
}
