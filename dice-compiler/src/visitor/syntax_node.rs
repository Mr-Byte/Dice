use super::{expr_block::BlockKind, NodeVisitor};
use crate::{compiler::Compiler, error::CompilerError};
use dice_syntax::{SyntaxNode, SyntaxNodeId};

impl NodeVisitor<SyntaxNodeId> for Compiler {
    fn visit(&mut self, node: SyntaxNodeId) -> Result<(), CompilerError> {
        let node = self
            .syntax_tree
            .get(node)
            .cloned()
            .expect("Node should never be empty.");

        match &node {
            SyntaxNode::LitIdent(literal) => self.visit(literal)?,
            SyntaxNode::LitUnit(literal) => self.visit(literal)?,
            SyntaxNode::LitNull(literal) => self.visit(literal)?,
            SyntaxNode::LitBool(literal) => self.visit(literal)?,
            SyntaxNode::LitInt(literal) => self.visit(literal)?,
            SyntaxNode::LitFloat(literal) => self.visit(literal)?,
            SyntaxNode::LitString(literal) => self.visit(literal)?,
            SyntaxNode::LitAnonymousFn(literal) => self.visit(literal)?,
            SyntaxNode::LitObject(literal) => self.visit(literal)?,
            SyntaxNode::LitList(literal) => self.visit(literal)?,
            SyntaxNode::Assignment(assignment) => self.visit(assignment)?,
            SyntaxNode::Unary(unary) => self.visit(unary)?,
            SyntaxNode::Binary(binary) => self.visit(binary)?,
            SyntaxNode::VarDecl(variable) => self.visit(variable)?,
            SyntaxNode::FnDecl(func) => self.visit(func)?,
            SyntaxNode::OpDecl(func) => self.visit(func)?,
            SyntaxNode::IfExpression(conditional) => self.visit(conditional)?,
            SyntaxNode::WhileLoop(while_loop) => self.visit(while_loop)?,
            SyntaxNode::ForLoop(_) => todo!(),
            SyntaxNode::Break(break_node) => self.visit(break_node)?,
            SyntaxNode::Continue(continue_node) => self.visit(continue_node)?,
            SyntaxNode::Block(block) => self.visit((block, BlockKind::<&str>::Block))?,
            SyntaxNode::Return(return_expr) => self.visit(return_expr)?,
            SyntaxNode::SafeAccess(safe_access) => {
                self.enter_call()?;
                self.visit(safe_access)?;
                self.exit_call()?;
            }
            SyntaxNode::FieldAccess(field_access) => {
                self.enter_call()?;
                self.visit(field_access)?;
                self.exit_call()?;
            }
            SyntaxNode::FunctionCall(fn_call) => {
                self.enter_call()?;
                self.visit(fn_call)?;
                self.exit_call()?;
            }
            SyntaxNode::Index(index) => {
                self.enter_call()?;
                self.visit(index)?;
                self.exit_call()?;
            }
        }

        Ok(())
    }
}

impl Compiler {
    fn enter_call(&mut self) -> Result<(), CompilerError> {
        let context = &mut self.context()?.scope_stack().top_mut()?.call_context;
        context.depth += 1;

        Ok(())
    }

    fn exit_call(&mut self) -> Result<(), CompilerError> {
        let context = &mut self.context()?.scope_stack().top_mut()?.call_context;
        context.depth -= 1;

        if context.depth == 0 {
            let exit_points = std::mem::take(&mut context.exit_points);
            for exit_point in exit_points.into_iter() {
                self.context()?.assembler().patch_jump(exit_point as u64);
            }
        }

        Ok(())
    }
}
