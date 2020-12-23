use crate::{compiler::Compiler, compiler_stack::CompilerKind, visitor::NodeVisitor};
use dice_core::{
    error::{
        codes::INVALID_EXPORT_USAGE,
        context::{Context, ContextKind, EXPORT_ONLY_ALLOWED_IN_MODULES, EXPORT_ONLY_ALLOWED_IN_TOP_LEVEL_SCOPE},
        Error,
    },
    protocol::{module::EXPORT, ProtocolSymbol},
    tags,
};
use dice_syntax::{ExportDecl, SyntaxNode, VarDecl, VarDeclKind};

impl NodeVisitor<&ExportDecl> for Compiler {
    fn visit(&mut self, node: &ExportDecl) -> Result<(), Error> {
        if !matches!(self.context()?.kind(), CompilerKind::Module) {
            return Err(Error::new(INVALID_EXPORT_USAGE)
                .push_context(
                    Context::new(EXPORT_ONLY_ALLOWED_IN_MODULES, ContextKind::Note).with_tags(tags! {
                        kind => self.context()?.kind().to_string()
                    }),
                )
                .with_source(self.source.clone())
                .with_span(node.span));
        }

        if self.context()?.scope_stack().top_mut()?.depth > 1 {
            return Err(Error::new(INVALID_EXPORT_USAGE)
                .push_context(Context::new(EXPORT_ONLY_ALLOWED_IN_TOP_LEVEL_SCOPE, ContextKind::Note))
                .with_source(self.source.clone())
                .with_span(node.span));
        }

        let export_slot = self
            .context()?
            .scope_stack()
            .local(EXPORT.get())
            .expect("#export should always be defined in modules.")
            .slot as u8;
        self.assembler()?.load_local(export_slot, node.span);
        self.visit(node.export)?;

        let field_name = match self.syntax_tree.get(node.export) {
            SyntaxNode::VarDecl(VarDecl {
                kind: VarDeclKind::Singular(name),
                ..
            }) => name.clone(),
            SyntaxNode::FnDecl(fn_decl) => fn_decl.name.identifier.clone(),
            SyntaxNode::ClassDecl(class_decl) => class_decl.name.identifier.clone(),
            SyntaxNode::LitIdent(lit_ident) => lit_ident.identifier.clone(),
            _ => unreachable!("Invalid export node type encountered."),
        };

        self.assembler()?.store_field(field_name, node.span)?;

        Ok(())
    }
}
