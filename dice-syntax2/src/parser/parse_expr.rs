use crate::syntax::SyntaxKind;
use crate::Parser;

impl<'a> Parser<'a> {
    pub(super) fn parse_expr(&mut self, minimum_binding: u8) {
        self.skip_trivia();
        let checkpoint = self.checkpoint();

        let next = self.peek();

        if next.map_or(false, is_literal) {
            self.bump();
        } else {
            // TODO: Handle prefix operators here.
        }

        self.skip_trivia();

        loop {
            if let Some(precedence) = self.peek().and_then(InfixPrecedence::try_from_kind) {
                if precedence.left_binding < minimum_binding {
                    return;
                }

                self.bump();
                self.start_node_at(checkpoint, SyntaxKind::InfixExpr);
                self.parse_expr(precedence.right_binding);
                self.finish_node();
            } else {
                return;
                // TODO: This is an error.
            }
        }
    }
}

fn is_literal(kind: SyntaxKind) -> bool {
    kind == SyntaxKind::Integer
        || kind == SyntaxKind::Float
        || kind == SyntaxKind::Identifier
        || kind == SyntaxKind::String
}

struct InfixPrecedence {
    left_binding: u8,
    right_binding: u8,
}

impl InfixPrecedence {
    const fn try_from_kind(kind: SyntaxKind) -> Option<Self> {
        match kind {
            SyntaxKind::Plus | SyntaxKind::Minus => Some(Self::new(1, 2)),
            SyntaxKind::Star | SyntaxKind::Slash | SyntaxKind::Remainder => Some(Self::new(3, 4)),
            _ => None,
        }
    }

    const fn new(left: u8, right: u8) -> Self {
        Self {
            left_binding: left,
            right_binding: right,
        }
    }
}
