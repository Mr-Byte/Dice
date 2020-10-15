use dice_error::{span::Span, syntax_error::SyntaxError};
use logos::Logos;
use std::{
    fmt::{Display, Formatter},
    iter::Iterator,
};

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
        TokenKind::lexer(input).spanned().map(move |(kind, span)| Token {
            kind,
            span: span.into(),
        })
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub const fn end_of_input() -> Token {
        Self {
            kind: TokenKind::EndOfInput,
            span: Span::new(0..0),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl Into<SyntaxError> for Token {
    fn into(self) -> SyntaxError {
        SyntaxError::UnexpectedToken(self.to_string(), self.span)
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum TokenKind {
    // End of input.
    EndOfInput,
    // Delimiters
    #[token("?(")]
    SafeCall,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[token("?[")]
    SafeIndex,
    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("|")]
    Pipe,
    #[token("#")]
    Hash,
    // Operators
    #[token("..")]
    ExclusiveRange,
    #[token("..=")]
    InclusiveRange,
    #[token("->")]
    Arrow,
    #[token("=>")]
    WideArrow,
    #[token(".")]
    Dot,
    #[token("?.")]
    SafeDot,
    #[token("??")]
    Coalesce,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("%")]
    Remainder,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("!")]
    Not,
    #[token("!=")]
    NotEqual,
    #[token("==")]
    Equal,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token("=")]
    Assign,
    #[token("*=")]
    MulAssign,
    #[token("/=")]
    DivAssign,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubAssign,
    #[token("d")]
    DiceRoll,
    #[token("&&")]
    LazyAnd,
    #[token("|>")]
    Pipeline,
    #[token("::")]
    UniversalMethodAccess,
    // Keywords
    #[token("object")]
    Object,
    #[token("false")]
    False,
    #[token("true")]
    True,
    #[token("null")]
    Null,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("do")]
    Do,
    #[token("loop")]
    Loop,
    #[token("for")]
    For,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,
    #[token("yield")]
    Yield,
    #[token("fn")]
    Function,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("const")]
    Const,
    #[token("match")]
    Match,
    #[token("trait")]
    Trait,
    #[token("in")]
    In,
    #[token("op")]
    Operator,
    #[token("class")]
    Class,
    #[token("type")]
    Type,
    #[token("typeof")]
    TypeOf,
    #[token("is")]
    Is,
    #[token("enum")]
    Enum,
    #[token("impl")]
    Impl,
    #[token("import")]
    Import,
    #[token("as")]
    As,
    #[token("from")]
    From,
    #[token("export")]
    Export,

    // Literals,
    #[regex("(d[_a-zA-Z][_a-zA-Z0-9]*)|([_a-ce-zA-Z][_a-zA-Z0-9]*)", |lex| lex.slice().to_owned())]
    Identifier(String),
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),
    #[regex(r#""((?:[^"\\]|\\.)*)""#, |lex| lex.slice().to_owned())]
    String(String),

    #[error]
    #[regex(r"[ \t\r\n\f]+|//[^\r\n]+", logos::skip)]
    Error,
}
