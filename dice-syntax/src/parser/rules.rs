use crate::{
    lexer::{Token, TokenKind},
    Parser, SyntaxNodeId,
};
use dice_error::{span::Span, syntax_error::SyntaxError};

pub type PrefixParser = fn(&mut Parser, can_assign: bool) -> Result<SyntaxNodeId, SyntaxError>;
pub type InfixParser =
    fn(&mut Parser, lhs: SyntaxNodeId, can_assign: bool, span: Span) -> Result<SyntaxNodeId, SyntaxError>;
pub type PostfixParser =
    fn(&mut Parser, lhs: SyntaxNodeId, can_assign: bool, span: Span) -> Result<SyntaxNodeId, SyntaxError>;

#[derive(Default)]
pub struct ParserRule {
    pub prefix: Option<PrefixParser>,
    pub infix: Option<InfixParser>,
    pub postfix: Option<PostfixParser>,
    pub infix_precedence: RulePrecedence,
    pub postfix_precedence: Option<RulePrecedence>,
}

impl ParserRule {
    fn new(
        prefix: Option<PrefixParser>,
        infix: Option<InfixParser>,
        postfix: Option<PostfixParser>,
        infix_precedence: RulePrecedence,
        postfix_precedence: Option<RulePrecedence>,
    ) -> Self {
        Self {
            prefix,
            infix,
            postfix,
            infix_precedence,
            postfix_precedence,
        }
    }

    pub fn for_token(token: &Token) -> Result<ParserRule, SyntaxError> {
        let rule = match token.kind {
            // Empty rules
            TokenKind::RightSquare => ParserRule::default(),
            TokenKind::RightParen => ParserRule::default(),
            TokenKind::RightCurly => ParserRule::default(),
            TokenKind::Semicolon => ParserRule::default(),
            TokenKind::Comma => ParserRule::default(),
            TokenKind::Colon => ParserRule::default(),
            TokenKind::Assign => ParserRule::default(),
            TokenKind::MulAssign => ParserRule::default(),
            TokenKind::DivAssign => ParserRule::default(),
            TokenKind::AddAssign => ParserRule::default(),
            TokenKind::SubAssign => ParserRule::default(),

            // Literals
            TokenKind::Integer(_) => ParserRule::new(Some(Parser::literal), None, None, RulePrecedence::Primary, None),
            TokenKind::Float(_) => ParserRule::new(Some(Parser::literal), None, None, RulePrecedence::Primary, None),
            TokenKind::String(_) => ParserRule::new(Some(Parser::literal), None, None, RulePrecedence::Primary, None),
            TokenKind::Null => ParserRule::new(Some(Parser::literal), None, None, RulePrecedence::Primary, None),
            TokenKind::False => ParserRule::new(Some(Parser::literal), None, None, RulePrecedence::Primary, None),
            TokenKind::True => ParserRule::new(Some(Parser::literal), None, None, RulePrecedence::Primary, None),
            TokenKind::Identifier(_) => {
                ParserRule::new(Some(Parser::variable), None, None, RulePrecedence::Primary, None)
            }

            // If expression
            TokenKind::If => ParserRule::new(Some(Parser::if_expression), None, None, RulePrecedence::None, None),

            // Objects
            TokenKind::Object => ParserRule::new(Some(Parser::object), None, None, RulePrecedence::Primary, None),
            TokenKind::LeftSquare => ParserRule::new(
                Some(Parser::list),
                Some(Parser::index_access),
                None,
                RulePrecedence::Call,
                None,
            ),
            TokenKind::Dot => ParserRule::new(None, Some(Parser::field_access), None, RulePrecedence::Call, None),

            // Grouping
            TokenKind::LeftParen => ParserRule::new(
                Some(Parser::grouping),
                Some(Parser::fn_call),
                None,
                RulePrecedence::Call,
                None,
            ),

            // Block expressions
            TokenKind::LeftCurly => {
                ParserRule::new(Some(Parser::block_expression), None, None, RulePrecedence::None, None)
            }

            // Operators
            TokenKind::Pipeline => ParserRule::new(
                None,
                Some(Parser::binary_operator),
                None,
                RulePrecedence::Pipeline,
                None,
            ),
            TokenKind::Coalesce => ParserRule::new(
                None,
                Some(Parser::binary_operator),
                None,
                RulePrecedence::Coalesce,
                None,
            ),
            TokenKind::RangeExclusive => {
                ParserRule::new(None, Some(Parser::binary_operator), None, RulePrecedence::Range, None)
            }
            TokenKind::RangeInclusive => {
                ParserRule::new(None, Some(Parser::binary_operator), None, RulePrecedence::Range, None)
            }
            TokenKind::LazyAnd => ParserRule::new(None, Some(Parser::binary_operator), None, RulePrecedence::And, None),
            TokenKind::Pipe => ParserRule::new(
                Some(Parser::anonymous_fn),
                Some(Parser::binary_operator),
                None,
                RulePrecedence::Or,
                None,
            ),
            TokenKind::Equal => ParserRule::new(
                None,
                Some(Parser::binary_operator),
                None,
                RulePrecedence::Comparison,
                None,
            ),
            TokenKind::NotEqual => ParserRule::new(
                None,
                Some(Parser::binary_operator),
                None,
                RulePrecedence::Comparison,
                None,
            ),
            TokenKind::Greater => ParserRule::new(
                None,
                Some(Parser::binary_operator),
                None,
                RulePrecedence::Comparison,
                None,
            ),
            TokenKind::GreaterEqual => ParserRule::new(
                None,
                Some(Parser::binary_operator),
                None,
                RulePrecedence::Comparison,
                None,
            ),
            TokenKind::Less => ParserRule::new(
                None,
                Some(Parser::binary_operator),
                None,
                RulePrecedence::Comparison,
                None,
            ),
            TokenKind::LessEqual => ParserRule::new(
                None,
                Some(Parser::binary_operator),
                None,
                RulePrecedence::Comparison,
                None,
            ),
            TokenKind::Is => ParserRule::new(
                None,
                Some(Parser::parse_is_operator),
                None,
                RulePrecedence::Comparison,
                None,
            ),
            TokenKind::Star => ParserRule::new(None, Some(Parser::binary_operator), None, RulePrecedence::Factor, None),
            TokenKind::Slash => {
                ParserRule::new(None, Some(Parser::binary_operator), None, RulePrecedence::Factor, None)
            }
            TokenKind::Remainder => {
                ParserRule::new(None, Some(Parser::binary_operator), None, RulePrecedence::Factor, None)
            }
            TokenKind::Plus => ParserRule::new(None, Some(Parser::binary_operator), None, RulePrecedence::Term, None),
            TokenKind::Minus => ParserRule::new(
                Some(Parser::unary_operator),
                Some(Parser::binary_operator),
                None,
                RulePrecedence::Term,
                None,
            ),
            TokenKind::DiceRoll => ParserRule::new(
                Some(Parser::unary_operator),
                Some(Parser::binary_operator),
                None,
                RulePrecedence::DiceRoll,
                None,
            ),
            TokenKind::Not => ParserRule::new(Some(Parser::unary_operator), None, None, RulePrecedence::Unary, None),

            // Postfix operators
            TokenKind::QuestionMark => ParserRule::new(
                None,
                None,
                Some(Parser::null_propagate),
                RulePrecedence::None,
                Some(RulePrecedence::Propagate),
            ),
            TokenKind::ErrorPropagate => ParserRule::new(
                None,
                None,
                Some(Parser::error_propagate),
                RulePrecedence::None,
                Some(RulePrecedence::Propagate),
            ),

            // Setup reserved keywords and sequence with a parser that returns a friendly error.

            // End of input
            TokenKind::EndOfInput => ParserRule::new(None, None, None, RulePrecedence::None, None),
            _ => return Err(token.clone().into()),
        };

        Ok(rule)
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RulePrecedence {
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

impl RulePrecedence {
    pub const fn increment(self) -> Self {
        match self {
            RulePrecedence::None => RulePrecedence::Assignment,
            RulePrecedence::Assignment => RulePrecedence::Pipeline,
            RulePrecedence::Pipeline => RulePrecedence::Coalesce,
            RulePrecedence::Coalesce => RulePrecedence::Range,
            RulePrecedence::Range => RulePrecedence::Or,
            RulePrecedence::Or => RulePrecedence::And,
            RulePrecedence::And => RulePrecedence::Comparison,
            RulePrecedence::Comparison => RulePrecedence::Term,
            RulePrecedence::Term => RulePrecedence::Factor,
            RulePrecedence::Factor => RulePrecedence::DiceRoll,
            RulePrecedence::DiceRoll => RulePrecedence::Unary,
            RulePrecedence::Unary => RulePrecedence::Propagate,
            RulePrecedence::Propagate => RulePrecedence::Call,
            RulePrecedence::Call => RulePrecedence::Object,
            RulePrecedence::Object => RulePrecedence::Primary,
            RulePrecedence::Primary => RulePrecedence::Primary,
        }
    }
}

impl Default for RulePrecedence {
    fn default() -> Self {
        Self::None
    }
}
