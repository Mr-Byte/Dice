use crate::compiler::Compiler;
use crate::compiler_stack::CompilerKind;
use crate::error::CompilerError;
use crate::visitor::NodeVisitor;
use dice_core::constants::EXPORT;
use dice_syntax::{ExportDecl, SyntaxNode};

impl NodeVisitor<&ExportDecl> for Compiler {
    fn visit(&mut self, node: &ExportDecl) -> Result<(), CompilerError> {
        if self.context()?.kind() != CompilerKind::Module {
            todo!("Error about how exports can only be used in modules.");
        }
        let export_slot = self
            .context()?
            .scope_stack()
            .local(EXPORT)
            .expect("#export should always be defined in modules.")
            .slot as u8;
        self.context()?.assembler().load_local(export_slot, node.span);
        self.visit(node.export)?;

        let field_name = match self.syntax_tree.get(node.export).expect("Node should exist.") {
            SyntaxNode::VarDecl(var_decl) => var_decl.name.clone(),
            SyntaxNode::FnDecl(fn_decl) => fn_decl.name.clone(),
            _ => todo!("Error about invalid export type."),
        };

        self.context()?.assembler().store_field(&field_name, node.span)?;

        Ok(())
    }
}
