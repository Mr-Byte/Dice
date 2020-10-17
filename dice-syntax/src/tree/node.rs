use crate::SyntaxNodeId;
use dice_error::span::Span;

#[derive(Debug, Clone)]
pub struct LitAnonymousFn {
    pub args: Vec<String>,
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
    pub name: String,
    pub span: Span,
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
    DiceRoll,
}

#[derive(Debug, Clone)]
pub struct SafeAccess {
    pub expression: SyntaxNodeId,
    pub field: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FieldAccess {
    pub expression: SyntaxNodeId,
    pub field: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub target: SyntaxNodeId,
    pub args: Vec<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UniversalMethodAccess {
    pub source: SyntaxNodeId,
    pub target: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Index {
    pub expression: SyntaxNodeId,
    pub index_expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Unary {
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
    DiceRoll,
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
    Is,
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
pub struct TraitImpl {
    pub trait_: String,
    // TODO: Should traits also be targets?
    pub target: String,
    pub name: Option<String>,
    pub associated_items: Vec<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub kind: VarDeclKind,
    pub is_mutable: bool,
    pub expr: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum VarDeclKind {
    Singular(String),
    Destructured(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub args: Vec<String>,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct AbstractFnDecl {
    pub name: String,
    pub args: Vec<String>,
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

#[derive(Debug, Clone)]
pub struct OpDecl {
    pub name: String,
    pub args: Vec<String>,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub name: String,
    pub span: Span,
    pub associated_items: Vec<SyntaxNodeId>,
}

#[derive(Debug, Clone)]
pub struct TraitDecl {
    pub name: String,
    pub span: Span,
    pub associated_items: Vec<SyntaxNodeId>,
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
