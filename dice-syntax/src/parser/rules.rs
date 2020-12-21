use crate::{lexer::TokenKind, Parser, SyntaxNodeId};
use dice_core::{error::Error, span::Span};
use std::collections::HashMap;

pub type ParseResult = Result<SyntaxNodeId, Error>;
pub type PrefixParser<'a> = fn(&mut Parser<'a>, bool) -> ParseResult;
pub type InfixParser<'a> = fn(&mut Parser<'a>, SyntaxNodeId, bool, Span) -> ParseResult;
pub type PostfixParser<'a> = fn(&mut Parser<'a>, SyntaxNodeId, bool, Span) -> ParseResult;

pub struct ParserRules<'a> {
    rules: HashMap<TokenKind, Rule<'a>>,
    prefix_tokens: Vec<TokenKind>,
}

impl<'a> ParserRules<'a> {
    pub fn new() -> Self {
        let mut rules = HashMap::default();

        // Empty rules
        // NOTE: When these rules are encountered, they indicate the end of the current expression/start of a new expression.
        rules.insert(TokenKind::RightSquare, Rule::new());
        rules.insert(TokenKind::RightParen, Rule::new());
        rules.insert(TokenKind::RightCurly, Rule::new());
        rules.insert(TokenKind::Comma, Rule::new());
        rules.insert(TokenKind::Colon, Rule::new());
        rules.insert(TokenKind::Assign, Rule::new());
        rules.insert(TokenKind::MulAssign, Rule::new());
        rules.insert(TokenKind::DivAssign, Rule::new());
        rules.insert(TokenKind::AddAssign, Rule::new());
        rules.insert(TokenKind::SubAssign, Rule::new());
        rules.insert(TokenKind::While, Rule::new());
        rules.insert(TokenKind::Loop, Rule::new());
        rules.insert(TokenKind::For, Rule::new());
        rules.insert(TokenKind::Let, Rule::new());
        rules.insert(TokenKind::Function, Rule::new());
        rules.insert(TokenKind::Operator, Rule::new());
        rules.insert(TokenKind::Class, Rule::new());
        rules.insert(TokenKind::Import, Rule::new());
        rules.insert(TokenKind::Export, Rule::new());
        rules.insert(TokenKind::Return, Rule::new());
        rules.insert(TokenKind::Break, Rule::new());
        rules.insert(TokenKind::Continue, Rule::new());

        // Literals
        rules.insert(
            TokenKind::Integer,
            Rule::new().with_prefix(Parser::literal, Precedence::Primary),
        );
        rules.insert(
            TokenKind::Float,
            Rule::new().with_prefix(Parser::literal, Precedence::Primary),
        );
        rules.insert(
            TokenKind::String,
            Rule::new().with_prefix(Parser::literal, Precedence::Primary),
        );
        rules.insert(
            TokenKind::Null,
            Rule::new().with_prefix(Parser::literal, Precedence::Primary),
        );
        rules.insert(
            TokenKind::False,
            Rule::new().with_prefix(Parser::literal, Precedence::Primary),
        );
        rules.insert(
            TokenKind::True,
            Rule::new().with_prefix(Parser::literal, Precedence::Primary),
        );
        rules.insert(
            TokenKind::Identifier,
            Rule::new().with_prefix(Parser::variable, Precedence::Primary),
        );

        rules.insert(
            TokenKind::If,
            Rule::new().with_prefix(Parser::if_expression, Precedence::None),
        );

        // Objects
        rules.insert(
            TokenKind::Object,
            Rule::new().with_prefix(Parser::object, Precedence::Primary),
        );

        rules.insert(
            TokenKind::LeftSquare,
            Rule::new()
                .with_prefix(Parser::list, Precedence::Primary)
                .with_infix(Parser::index_access, Precedence::Call),
        );
        rules.insert(
            TokenKind::Dot,
            Rule::new().with_infix(Parser::field_access, Precedence::Call),
        );
        rules.insert(
            TokenKind::Super,
            Rule::new().with_prefix(Parser::super_access, Precedence::Call),
        );

        // Grouping
        rules.insert(
            TokenKind::LeftParen,
            Rule::new()
                .with_prefix(Parser::grouping, Precedence::Primary)
                .with_infix(Parser::fn_call, Precedence::Call),
        );
        rules.insert(
            TokenKind::LeftCurly,
            Rule::new().with_prefix(Parser::block_expression, Precedence::None),
        );

        // Operators
        rules.insert(
            TokenKind::Pipeline,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Pipeline),
        );
        rules.insert(
            TokenKind::Coalesce,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Coalesce),
        );
        rules.insert(
            TokenKind::RangeExclusive,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Range),
        );
        rules.insert(
            TokenKind::RangeInclusive,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Range),
        );
        rules.insert(
            TokenKind::LazyAnd,
            Rule::new().with_infix(Parser::binary_operator, Precedence::And),
        );
        rules.insert(
            TokenKind::Pipe,
            Rule::new()
                .with_prefix(Parser::anonymous_fn, Precedence::Primary)
                .with_infix(Parser::binary_operator, Precedence::Or),
        );
        rules.insert(
            TokenKind::Equal,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Comparison),
        );
        rules.insert(
            TokenKind::NotEqual,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Comparison),
        );
        rules.insert(
            TokenKind::Greater,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Comparison),
        );
        rules.insert(
            TokenKind::GreaterEqual,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Comparison),
        );
        rules.insert(
            TokenKind::Less,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Comparison),
        );
        rules.insert(
            TokenKind::LessEqual,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Comparison),
        );
        rules.insert(
            TokenKind::Is,
            Rule::new().with_infix(Parser::is_operator, Precedence::Comparison),
        );
        rules.insert(
            TokenKind::Star,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Factor),
        );
        rules.insert(
            TokenKind::Slash,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Factor),
        );
        rules.insert(
            TokenKind::Remainder,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Factor),
        );
        rules.insert(
            TokenKind::Plus,
            Rule::new().with_infix(Parser::binary_operator, Precedence::Term),
        );
        rules.insert(
            TokenKind::Minus,
            Rule::new()
                .with_prefix(Parser::prefix_operator, Precedence::Unary)
                .with_infix(Parser::binary_operator, Precedence::Term),
        );
        rules.insert(
            TokenKind::DiceRoll,
            Rule::new()
                .with_prefix(Parser::prefix_operator, Precedence::Unary)
                .with_infix(Parser::binary_operator, Precedence::DiceRoll),
        );
        rules.insert(
            TokenKind::Not,
            Rule::new().with_prefix(Parser::prefix_operator, Precedence::Unary),
        );
        rules.insert(
            TokenKind::QuestionMark,
            Rule::new().with_postfix(Parser::null_propagate, Precedence::Propagate),
        );
        rules.insert(
            TokenKind::ErrorPropagate,
            Rule::new().with_postfix(Parser::error_propagate, Precedence::Propagate),
        );

        // TODO: Setup reserved keywords and sequence with a parser that returns a friendly error.
        // End of input
        rules.insert(TokenKind::EndOfInput, Rule::new());

        let prefix_tokens = rules
            .iter()
            .filter_map(|(key, value)| value.prefix.map(|_| *key))
            .collect::<Vec<_>>();

        Self { rules, prefix_tokens }
    }

    pub fn for_token(&self, kind: TokenKind) -> Result<&Rule<'a>, Error> {
        self.rules
            .get(&kind)
            .ok_or_else(|| unreachable!("Unreachable token scenario reached."))
    }

    pub fn prefix_tokens(&self) -> &[TokenKind] {
        &self.prefix_tokens
    }
}

#[derive(Default)]
pub struct Rule<'a> {
    pub prefix: Option<(PrefixParser<'a>, Precedence)>,
    pub infix: Option<(InfixParser<'a>, Precedence)>,
    pub postfix: Option<(PostfixParser<'a>, Precedence)>,
}

impl<'a> Rule<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    fn with_prefix(mut self, prefix: PrefixParser<'a>, precedence: Precedence) -> Self {
        self.prefix = Some((prefix, precedence));
        self
    }

    fn with_infix(mut self, infix: InfixParser<'a>, precedence: Precedence) -> Self {
        self.infix = Some((infix, precedence));
        self
    }

    fn with_postfix(mut self, postfix: PostfixParser<'a>, precedence: Precedence) -> Self {
        self.postfix = Some((postfix, precedence));
        self
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
    pub const fn increment(self) -> Self {
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
            Precedence::Primary => Precedence::Primary,
        }
    }
}

impl Default for Precedence {
    fn default() -> Self {
        Self::None
    }
}
