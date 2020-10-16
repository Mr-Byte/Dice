use crate::{compiler::Compiler, compiler_stack::CompilerKind, visitor::NodeVisitor};
use dice_core::protocol::module::EXPORT;
use dice_error::compiler_error::CompilerError;
use dice_syntax::{ExportDecl, SyntaxNode, VarDecl, VarDeclKind};

impl NodeVisitor<&ExportDecl> for Compiler {
    fn visit(&mut self, node: &ExportDecl) -> Result<(), CompilerError> {
        if self.context()?.kind() != CompilerKind::Module {
            todo!("Error about how exports can only be used in modules.");
        }

        if self.context()?.scope_stack().top_mut()?.depth != 0 {
            todo!("Error about how exports can only be used as top-level declarations.");
        }

        let export_slot = self
            .context()?
            .scope_stack()
            .local(EXPORT)
            .expect("#export should always be defined in modules.")
            .slot as u8;
        self.context()?.assembler().load_local(export_slot, node.span);
        self.visit(node.export)?;

        let field_name = match self.syntax_tree.get(node.export) {
            SyntaxNode::VarDecl(VarDecl {
                kind: VarDeclKind::Singular(name),
                ..
            }) => name.clone(),
            SyntaxNode::FnDecl(fn_decl) => fn_decl.name.clone(),
            SyntaxNode::ClassDecl(class_decl) => class_decl.name.clone(),
            SyntaxNode::LitIdent(lit_ident) => lit_ident.name.clone(),
            _ => todo!("Error about invalid export type."),
        };

        self.context()?.assembler().store_field(&field_name, node.span)?;

        Ok(())
    }
}
