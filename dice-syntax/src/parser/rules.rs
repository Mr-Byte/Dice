use crate::lexer::{Token, TokenKind};
use crate::{Parser, SyntaxNodeId};
use dice_error::span::Span;
use dice_error::syntax_error::SyntaxError;

pub type PrefixParser = fn(&mut Parser, can_assign: bool) -> Result<SyntaxNodeId, SyntaxError>;
pub type InfixParser =
    fn(&mut Parser, lhs: SyntaxNodeId, can_assign: bool, span: Span) -> Result<SyntaxNodeId, SyntaxError>;

#[derive(Default)]
pub struct ParserRule {
    pub prefix: Option<PrefixParser>,
    pub infix: Option<InfixParser>,
    pub precedence: RulePrecedence,
}

impl ParserRule {
    fn new(prefix: Option<PrefixParser>, infix: Option<InfixParser>, precedence: RulePrecedence) -> Self {
        Self {
            prefix,
            infix,
            precedence,
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
            TokenKind::Integer(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::Float(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::String(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::Null => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::False => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::True => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::Identifier(_) => ParserRule::new(Some(Parser::variable), None, RulePrecedence::Primary),

            // If expression
            TokenKind::If => ParserRule::new(Some(Parser::if_expression), None, RulePrecedence::None),

            // Objects
            TokenKind::Object => ParserRule::new(Some(Parser::object), None, RulePrecedence::Primary),
            TokenKind::LeftSquare => {
                ParserRule::new(Some(Parser::list), Some(Parser::index_access), RulePrecedence::Call)
            }
            TokenKind::Dot => ParserRule::new(None, Some(Parser::field_access), RulePrecedence::Call),
            TokenKind::SafeDot => ParserRule::new(None, Some(Parser::safe_field_access), RulePrecedence::Call),
            TokenKind::UniversalMethodAccess => {
                ParserRule::new(None, Some(Parser::universal_method_access), RulePrecedence::Call)
            }

            // Grouping
            TokenKind::LeftParen => {
                ParserRule::new(Some(Parser::grouping), Some(Parser::fn_call), RulePrecedence::Call)
            }

            // Block expressions
            TokenKind::LeftCurly => ParserRule::new(Some(Parser::block_expression), None, RulePrecedence::None),

            // Operators
            TokenKind::Pipeline => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Pipeline),
            TokenKind::Coalesce => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Coalesce),
            TokenKind::ExclusiveRange => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Range),
            TokenKind::InclusiveRange => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Range),
            TokenKind::LazyAnd => ParserRule::new(None, Some(Parser::binary), RulePrecedence::And),
            TokenKind::Pipe => ParserRule::new(Some(Parser::anonymous_fn), Some(Parser::binary), RulePrecedence::Or),
            TokenKind::Equal => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::NotEqual => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::Greater => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::GreaterEqual => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::Less => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::LessEqual => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::Is => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::Star => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Factor),
            TokenKind::Slash => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Factor),
            TokenKind::Remainder => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Factor),
            TokenKind::Plus => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Term),
            TokenKind::Minus => ParserRule::new(Some(Parser::unary), Some(Parser::binary), RulePrecedence::Term),
            TokenKind::DiceRoll => ParserRule::new(Some(Parser::unary), Some(Parser::binary), RulePrecedence::DiceRoll),
            TokenKind::Not => ParserRule::new(Some(Parser::unary), None, RulePrecedence::Unary),

            // Setup reserved keywords and sequence with a parser that returns a friendly error.

            // End of input
            TokenKind::EndOfInput => ParserRule::new(None, None, RulePrecedence::None),
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
            RulePrecedence::Unary => RulePrecedence::Call,
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
