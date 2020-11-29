use crate::syntax::SyntaxKind;
use crate::Parser;

pub type ParserFn<'a> = fn(&mut Parser<'a>, can_assign: bool);

impl<'a> Parser<'a> {
    pub(super) fn initialize_rules(mut self) -> Self {
        self.add_rule(
            SyntaxKind::Identifier,
            Some((Self::parse_literal, Precedence::Primary)),
            None,
            None,
        );
        self.add_rule(
            SyntaxKind::Integer,
            Some((Self::parse_literal, Precedence::Primary)),
            None,
            None,
        );
        self.add_rule(
            SyntaxKind::Float,
            Some((Self::parse_literal, Precedence::Primary)),
            None,
            None,
        );
        self.add_rule(
            SyntaxKind::Star,
            None,
            Some((Self::parse_bin_op, Precedence::Factor)),
            None,
        );
        self.add_rule(
            SyntaxKind::Slash,
            None,
            Some((Self::parse_bin_op, Precedence::Factor)),
            None,
        );
        self.add_rule(
            SyntaxKind::Remainder,
            None,
            Some((Self::parse_bin_op, Precedence::Factor)),
            None,
        );
        self.add_rule(
            SyntaxKind::Plus,
            None,
            Some((Self::parse_bin_op, Precedence::Term)),
            None,
        );
        self.add_rule(
            SyntaxKind::Minus,
            None,
            Some((Self::parse_bin_op, Precedence::Term)),
            None,
        );

        self
    }

    fn add_rule(
        &mut self,
        kind: SyntaxKind,
        prefix: Option<(ParserFn<'a>, Precedence)>,
        infix: Option<(ParserFn<'a>, Precedence)>,
        postfix: Option<(ParserFn<'a>, Precedence)>,
    ) {
        let rule = Rule { prefix, infix, postfix };

        self.rules.insert(kind, rule);
    }
}

pub(super) struct Rule<'a> {
    prefix: Option<(ParserFn<'a>, Precedence)>,
    infix: Option<(ParserFn<'a>, Precedence)>,
    postfix: Option<(ParserFn<'a>, Precedence)>,
}

impl<'a> Rule<'a> {
    pub fn prefix(&self) -> Option<&(ParserFn<'a>, Precedence)> {
        self.prefix.as_ref()
    }

    pub fn infix(&self) -> Option<&(ParserFn<'a>, Precedence)> {
        self.infix.as_ref()
    }

    pub fn postfix(&self) -> Option<&(ParserFn<'a>, Precedence)> {
        self.postfix.as_ref()
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Precedence {
    None,
    Assignment,
    Pipeline,
    Coalesce,
    Range,
    Or,
    And,
    Comparison,
    Term,
    Factor,
    DiceRoll,
    Unary,
    Propagate,
    Call,
    Object,
    Primary,
}

impl Precedence {
    pub fn next(self) -> Self {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Pipeline,
            Precedence::Pipeline => Precedence::Coalesce,
            Precedence::Coalesce => Precedence::Range,
            Precedence::Range => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::DiceRoll,
            Precedence::DiceRoll => Precedence::Unary,
            Precedence::Unary => Precedence::Propagate,
            Precedence::Propagate => Precedence::Call,
            Precedence::Call => Precedence::Object,
            Precedence::Object => Precedence::Primary,
            Precedence::Primary => unreachable!("Precedence beyond primary should be unreachable."),
        }
    }
}
