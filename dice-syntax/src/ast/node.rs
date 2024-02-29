use std::fmt::{Display, Formatter};

use dice_core::span::Span;

use crate::SyntaxNodeId;

#[derive(Debug, Clone)]
pub struct LitAnonymousFn {
    pub args: Vec<FnArg>,
    pub return_: Option<TypeAnnotation>,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitList {
    pub items: Vec<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitObject {
    pub items: Vec<(String, SyntaxNodeId)>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitIdent {
    pub identifier: String,
    pub span: Span,
}

impl LitIdent {
    pub fn synthesize(identifier: impl Into<String>, span: Span) -> Self {
        Self {
            identifier: identifier.into(),
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LitUnit {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitNull {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitInt {
    pub value: i64,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitFloat {
    pub value: f64,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitString {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitBool {
    pub value: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub struct NullPropagate {
    pub expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ErrorPropagate {
    pub expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FieldAccess {
    pub expression: SyntaxNodeId,
    pub field: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct SuperAccess {
    pub field: String,
    pub super_class: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub target: SyntaxNodeId,
    pub args: Vec<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct SuperCall {
    pub args: Vec<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Index {
    pub expression: SyntaxNodeId,
    pub index_expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Is {
    pub value: SyntaxNodeId,
    pub type_: TypeAnnotation,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Prefix {
    pub operator: UnaryOperator,
    pub expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub operator: BinaryOperator,
    pub lhs_expression: SyntaxNodeId,
    pub rhs_expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOperator {
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    Equals,
    NotEquals,
    LogicalAnd,
    LogicalOr,
    RangeInclusive,
    RangeExclusive,
    Coalesce,
    Pipeline,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub operator: AssignmentOperator,
    pub lhs_expression: SyntaxNodeId,
    pub rhs_expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Copy, Clone)]
pub enum AssignmentOperator {
    Assignment,
    MulAssignment,
    DivAssignment,
    AddAssignment,
    SubAssignment,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub kind: VarDeclKind,
    pub is_mutable: bool,
    pub expr: SyntaxNodeId,
    pub type_: Option<TypeAnnotation>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum VarDeclKind {
    Singular(String),
    Destructured(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: LitIdent,
    pub args: Vec<FnArg>,
    pub return_: Option<TypeAnnotation>,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct OpDecl {
    pub operator: OverloadedOperator,
    pub args: Vec<FnArg>,
    pub return_: Option<TypeAnnotation>,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnArg {
    pub name: String,
    pub type_: Option<TypeAnnotation>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub name: LitIdent,
    pub is_nullable: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ExportDecl {
    pub export: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ImportDecl {
    pub module_import: Option<String>,
    pub item_imports: Vec<String>,
    pub relative_path: String,
    pub span: Span,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OverloadedOperator {
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    Equals,
    NotEquals,
    RangeInclusive,
    RangeExclusive,
}

impl Display for OverloadedOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OverloadedOperator::Multiply => write!(f, "*"),
            OverloadedOperator::Divide => write!(f, "/"),
            OverloadedOperator::Remainder => write!(f, "%"),
            OverloadedOperator::Add => write!(f, "+"),
            OverloadedOperator::Subtract => write!(f, "-"),
            OverloadedOperator::GreaterThan => write!(f, ">"),
            OverloadedOperator::LessThan => write!(f, "<"),
            OverloadedOperator::GreaterThanEquals => write!(f, ">="),
            OverloadedOperator::LessThanEquals => write!(f, "<="),
            OverloadedOperator::Equals => write!(f, "=="),
            OverloadedOperator::NotEquals => write!(f, "!="),
            OverloadedOperator::RangeInclusive => write!(f, ".."),
            OverloadedOperator::RangeExclusive => write!(f, "..="),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub name: LitIdent,
    pub span: Span,
    pub associated_items: Vec<SyntaxNodeId>,
    pub base: Option<SyntaxNodeId>,
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: SyntaxNodeId,
    pub primary: SyntaxNodeId,
    pub secondary: Option<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: SyntaxNodeId,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub variable: String,
    pub source: SyntaxNodeId,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Loop {
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub expressions: Vec<SyntaxNodeId>,
    pub trailing_expression: Option<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Break {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Continue {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub result: Option<SyntaxNodeId>,
    pub span: Span,
}
