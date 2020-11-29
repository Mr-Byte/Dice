use crate::parser::rules::Precedence;
use crate::syntax::SyntaxKind;
use crate::Parser;

impl<'a> Parser<'a> {
    pub(super) fn parse_expr(&mut self, precedence: Precedence) {
        self.skip_trivia();

        let checkpoint = self.checkpoint();

        // NOTE: Parse either a prefix operator or literal.
        match self.peek_prefix_rule() {
            Some((parser, _)) => parser(self, false),
            None => return, // TODO: Handle errors.
        }

        self.skip_trivia();

        loop {
            match self.peek_infix_rule() {
                Some((parser, infix_precedence)) => {
                    if precedence > *infix_precedence {
                        return;
                    }

                    parser(self, false);
                    self.start_node_at(checkpoint, SyntaxKind::InfixExpr);
                    self.parse_expr(precedence.next());
                    self.finish_node();
                }
                None => return,
            }

            self.skip_trivia();
        }
    }
}
