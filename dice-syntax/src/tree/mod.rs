use dice_error::span::Span;
use id_arena::{Arena, Id};
pub use node::*;
use std::rc::Rc;

mod node;

pub type SyntaxNodeId = Id<SyntaxNode>;

pub struct SyntaxTree {
    root: SyntaxNodeId,
    nodes: Rc<Arena<SyntaxNode>>,
}

impl SyntaxTree {
    pub(crate) fn new(root: SyntaxNodeId, nodes: Arena<SyntaxNode>) -> Self {
        Self {
            root,
            nodes: Rc::new(nodes),
        }
    }

    pub fn root(&self) -> SyntaxNodeId {
        self.root
    }

    pub fn get(&self, id: SyntaxNodeId) -> &SyntaxNode {
        self.nodes.get(id).expect("Node should always exist.")
    }

    pub fn child(&self, id: SyntaxNodeId) -> SyntaxTree {
        self.nodes
            .get(id)
            .map(|_| Self {
                root: id,
                nodes: self.nodes.clone(),
            })
            .expect("Node should always exist.")
    }
}

#[derive(Debug, Clone)]
pub enum SyntaxNode {
    // Literals
    LitIdent(LitIdent),
    LitNull(LitNull),
    LitUnit(LitUnit),
    LitInt(LitInt),
    LitFloat(LitFloat),
    LitString(LitString),
    LitBool(LitBool),
    LitList(LitList),
    LitObject(LitObject),
    LitAnonymousFn(LitAnonymousFn),

    // Member access
    FieldAccess(FieldAccess),
    Index(Index),
    UniversalMethodAccess(UniversalMethodAccess),

    // Operators
    Unary(Unary),
    Binary(Binary),
    NullPropagate(NullPropagate),
    Assignment(Assignment),
    TraitImpl(TraitImpl),

    // Declarations
    VarDecl(VarDecl),
    FnDecl(FnDecl),
    AbstractFnDecl(AbstractFnDecl),
    OpDecl(OpDecl),
    ClassDecl(ClassDecl),
    TraitDecl(TraitDecl),
    ImportDecl(ImportDecl),
    ExportDecl(ExportDecl),

    // Control flow
    IfExpression(IfExpression),
    Loop(Loop),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    Block(Block),
    Break(Break),
    Return(Return),
    Continue(Continue),
    FunctionCall(FunctionCall),
}

impl SyntaxNode {
    pub fn span(&self) -> Span {
        match self {
            SyntaxNode::LitIdent(LitIdent { span, .. }) => *span,
            SyntaxNode::LitNull(LitNull { span, .. }) => *span,
            SyntaxNode::LitUnit(LitUnit { span, .. }) => *span,
            SyntaxNode::LitInt(LitInt { span, .. }) => *span,
            SyntaxNode::LitFloat(LitFloat { span, .. }) => *span,
            SyntaxNode::LitString(LitString { span, .. }) => *span,
            SyntaxNode::LitBool(LitBool { span, .. }) => *span,
            SyntaxNode::LitList(LitList { span, .. }) => *span,
            SyntaxNode::LitObject(LitObject { span, .. }) => *span,
            SyntaxNode::LitAnonymousFn(LitAnonymousFn { span, .. }) => *span,
            SyntaxNode::FieldAccess(FieldAccess { span, .. }) => *span,
            SyntaxNode::Index(Index { span, .. }) => *span,
            SyntaxNode::Unary(Unary { span, .. }) => *span,
            SyntaxNode::Binary(Binary { span, .. }) => *span,
            SyntaxNode::NullPropagate(NullPropagate { span, .. }) => *span,
            SyntaxNode::Assignment(Assignment { span, .. }) => *span,
            SyntaxNode::TraitImpl(TraitImpl { span, .. }) => *span,
            SyntaxNode::VarDecl(VarDecl { span, .. }) => *span,
            SyntaxNode::FnDecl(FnDecl { span, .. }) => *span,
            SyntaxNode::AbstractFnDecl(AbstractFnDecl { span, .. }) => *span,
            SyntaxNode::OpDecl(OpDecl { span, .. }) => *span,
            SyntaxNode::ClassDecl(ClassDecl { span, .. }) => *span,
            SyntaxNode::TraitDecl(TraitDecl { span, .. }) => *span,
            SyntaxNode::ImportDecl(ImportDecl { span, .. }) => *span,
            SyntaxNode::ExportDecl(ExportDecl { span, .. }) => *span,
            SyntaxNode::IfExpression(IfExpression { span, .. }) => *span,
            SyntaxNode::WhileLoop(WhileLoop { span, .. }) => *span,
            SyntaxNode::ForLoop(ForLoop { span, .. }) => *span,
            SyntaxNode::Loop(Loop { span, .. }) => *span,
            SyntaxNode::Block(Block { span, .. }) => *span,
            SyntaxNode::Break(Break { span, .. }) => *span,
            SyntaxNode::Return(Return { span, .. }) => *span,
            SyntaxNode::Continue(Continue { span, .. }) => *span,
            SyntaxNode::FunctionCall(FunctionCall { span, .. }) => *span,
            SyntaxNode::UniversalMethodAccess(UniversalMethodAccess { span, .. }) => *span,
        }
    }
}
