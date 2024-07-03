use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier<'a> {
    pub name: Cow<'a, str>
}
#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    String(StringLiteral<'a>),
    Number(NumberLiteral<'a>),
    Bool(BoolLiteral<'a>),
}
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral<'a> {
    pub value: Cow<'a, str>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct BoolLiteral<'a> {
    pub raw_value: Cow<'a, str>,
    pub value: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral<'a> {
    pub raw_value: Cow<'a, str>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct RootNode<'a> {
   pub instructions: Vec<InstructionNode<'a>>
}
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionNode<'a> {
    Query(QueryActionNode<'a>),
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryActionNode<'a> {
   pub target_node: DescriptionNode<'a>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DescriptionNode<'a> {
    pub selectors: Vec<Selector<'a>>
}
#[derive(Debug, Clone, PartialEq)]
pub enum Selector<'a> {
    Parent(ParentSelector<'a>),
    Ancestor(AncestorSelector<'a>),
    Literal(LiteralSelector<'a>),
    Recursive(RecursiveSelector<'a>),
    Array(ArraySelector<'a>)
}
#[derive(Debug, Clone, PartialEq)]
pub struct ParentSelector<'a> {
    pub key: Cow<'a, str>,
    pub target_node: DescriptionNode<'a>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AncestorSelector<'a> {
    pub key: Cow<'a, str>,
    pub target_node: DescriptionNode<'a>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralSelector<'a> {
    pub key: Cow<'a, str>,
    pub literal: Literal<'a>
}
#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveSelector<'a> {
    pub key: Cow<'a, str>,
    pub target_node: DescriptionNode<'a>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralOrNode<'a> {
    Lit(Literal<'a>),
    Node(DescriptionNode<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArraySelector<'a> {
    pub key: Cow<'a, str>,
    pub value: LiteralOrNode<'a>
}
