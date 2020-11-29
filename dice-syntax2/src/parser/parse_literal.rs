use crate::syntax::SyntaxKind;
use crate::Parser;

impl<'a> Parser<'a> {
    pub(super) fn parse_literal(&mut self, _: bool) {
        if let Some(SyntaxKind::Float) | Some(SyntaxKind::Integer) | Some(SyntaxKind::Identifier) = self.peek() {
            self.bump()
        }
    }
}
