use dice_error::{span::Span, syntax_error::SyntaxError};
use logos::{Lexer, Logos};
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
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
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
    RangeExclusive,
    #[token("..=")]
    RangeInclusive,
    #[token("->")]
    Arrow,
    #[token("=>")]
    WideArrow,
    #[token(".")]
    Dot,
    #[token("?")]
    NullPropagate,
    #[token("!!")]
    ErrorPropagate,
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
    #[regex(r#"""#, lex_string)]
    String(String),

    #[error]
    #[regex(r"[ \t\r\n\f]+|//[^\r\n]+", logos::skip)]
    Error,
}

fn lex_string(lexer: &mut Lexer<TokenKind>) -> Result<String, StringLexError> {
    let remainder = lexer.remainder();
    let string_length = string_length(remainder)?;
    let mut result = String::new();
    let mut chars = remainder[0..string_length].chars();

    while let Some(current) = chars.next() {
        if current == '\\' {
            let next = chars.next();

            match next {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some(next) => {
                    let sequence = format!("{}{}", current, next);
                    return Err(StringLexError::UnrecognizedEscapeSequence(sequence));
                }
                None => return Err(StringLexError::UnexpectedEndOfInput),
            }
        } else {
            result.push(current);
        }
    }

    lexer.bump(string_length + 1);

    Ok(result)
}

// TODO: Can this be made cleaner?
fn string_length(remainder: &str) -> Result<usize, StringLexError> {
    let bytes = remainder.as_bytes();

    if bytes[0] == b'"' {
        return Ok(0);
    }

    let mut count = 0;
    loop {
        if count == bytes.len() {
            return Err(StringLexError::UnterminatedString);
        }

        if bytes[count] == b'\\' {
            count += 1;

            if count == bytes.len() {
                return Err(StringLexError::UnterminatedString);
            }
        } else if bytes[count] == b'"' {
            break;
        }

        count += 1;
    }

    Ok(count)
}

#[derive(thiserror::Error, Debug)]
pub enum StringLexError {
    #[error("String is not terminated with a double quote.")]
    UnterminatedString,
    #[error("Unrecognized escape sequence {0} found.")]
    UnrecognizedEscapeSequence(String),
    #[error("Unexpected end of input reached.")]
    UnexpectedEndOfInput,
}
