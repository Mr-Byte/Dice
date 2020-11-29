mod parse_bin_op;
mod parse_expr;
mod parse_literal;
mod rules;

use crate::lexer::Lexer;
use crate::parser::rules::{ParserFn, Precedence, Rule};
use crate::syntax::{Lang, SyntaxKind, SyntaxNode};
use rowan::{GreenNode, GreenNodeBuilder, Language};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    rules: HashMap<SyntaxKind, Rule<'a>, BuildHasherDefault<wyhash::WyHash>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser::initialize_rules(Self {
            lexer: Lexer::new(input),
            rules: Default::default(),
            builder: GreenNodeBuilder::new(),
        })
    }

    pub fn parse(mut self) -> ParseResult {
        self.start_node(SyntaxKind::Root);
        self.parse_expr(Precedence::None);
        self.finish_node();

        ParseResult {
            green_node: self.builder.finish(),
        }
    }

    fn skip_trivia(&mut self) {
        while let Some(SyntaxKind::Whitespace) | Some(SyntaxKind::Comment) = self.peek() {
            self.bump()
        }
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.pop();

        self.builder.token(Lang::kind_to_raw(kind), text.into());
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek()
    }

    fn peek_prefix_rule(&mut self) -> Option<&(ParserFn<'a>, Precedence)> {
        self.peek()
            .and_then(move |kind| self.rules.get(&kind))
            .and_then(|rule| rule.prefix())
    }

    fn peek_infix_rule(&mut self) -> Option<&(ParserFn<'a>, Precedence)> {
        self.peek()
            .and_then(move |kind| self.rules.get(&kind))
            .and_then(|rule| rule.infix())
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(Lang::kind_to_raw(kind));
    }

    fn start_node_at(&mut self, checkpoint: rowan::Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, Lang::kind_to_raw(kind))
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn checkpoint(&mut self) -> rowan::Checkpoint {
        self.builder.checkpoint()
    }
}

pub struct ParseResult {
    green_node: GreenNode,
}

impl ParseResult {
    pub fn format_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        format!("{:#?}", syntax_node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = Parser::new(input).parse();
        expected_tree.assert_eq(&parse.format_tree());
    }

    #[test]
    fn parse_empty() {
        check(
            "",
            expect![[r#"
                Root@0..0
            "#]],
        )
    }

    #[test]
    fn parse_float() {
        check(
            "5.0",
            expect![[r#"
                Root@0..3
                  Float@0..3 "5.0"
            "#]],
        )
    }

    #[test]
    fn parse_integer() {
        check(
            "5",
            expect![[r#"
                Root@0..1
                  Integer@0..1 "5"
            "#]],
        )
    }

    #[test]
    fn parse_ident() {
        check(
            "abc",
            expect![[r#"
                Root@0..3
                  Identifier@0..3 "abc"
            "#]],
        )
    }

    #[test]
    fn parse_multiplication() {
        check(
            "5 * 5",
            expect![[r#"
                Root@0..5
                  InfixExpr@0..5
                    Integer@0..1 "5"
                    Whitespace@1..2 " "
                    Star@2..3 "*"
                    Whitespace@3..4 " "
                    Integer@4..5 "5"
            "#]],
        )
    }

    #[test]
    fn parse_addition() {
        check(
            "5 + 5",
            expect![[r#"
                Root@0..5
                  InfixExpr@0..5
                    Integer@0..1 "5"
                    Whitespace@1..2 " "
                    Plus@2..3 "+"
                    Whitespace@3..4 " "
                    Integer@4..5 "5"
            "#]],
        )
    }

    #[test]
    fn parse_subtraction() {
        check(
            "5 - 5",
            expect![[r#"
                Root@0..5
                  InfixExpr@0..5
                    Integer@0..1 "5"
                    Whitespace@1..2 " "
                    Minus@2..3 "-"
                    Whitespace@3..4 " "
                    Integer@4..5 "5"
            "#]],
        )
    }

    #[test]
    fn parse_precedence() {
        check(
            "5 + 5 * 5",
            expect![[r#"
                Root@0..9
                  InfixExpr@0..9
                    Integer@0..1 "5"
                    Whitespace@1..2 " "
                    Plus@2..3 "+"
                    Whitespace@3..4 " "
                    InfixExpr@4..9
                      Integer@4..5 "5"
                      Whitespace@5..6 " "
                      Star@6..7 "*"
                      Whitespace@7..8 " "
                      Integer@8..9 "5"
            "#]],
        )
    }
}
