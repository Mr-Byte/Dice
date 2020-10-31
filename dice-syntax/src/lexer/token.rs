use dice_error::syntax_error::LexerError;
use dice_error::{span::Span, syntax_error::SyntaxError};
use logos::{Lexer, Logos};
use std::cell::RefCell;
use std::rc::Rc;
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
    pub fn tokenize(input: &str) -> impl Iterator<Item = Result<Token, SyntaxError>> + '_ {
        let lexer = TokenKind::lexer(input);
        let error = lexer.extras.clone();

        lexer.spanned().map(move |(kind, span)| {
            let span: Span = span.into();
            error.error().map_or_else(
                || Ok(Token { kind, span }),
                |err| Err(SyntaxError::LexerError(err, span)),
            )
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
#[logos(extras = LexerResult)]
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
    QuestionMark,
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
    // Keywords
    #[token(r"#")]
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
    #[token("fn")]
    Function,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("trait")]
    Trait,
    #[token("in")]
    In,
    #[token("op")]
    Operator,
    #[token("class")]
    Class,
    #[token("is")]
    Is,
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
    #[regex("await|async|yield|do|const|match|enum|type")]
    Reserved,

    // Literals,
    #[regex("(d[_a-zA-Z][_a-zA-Z0-9]*)|([_a-ce-zA-Z][_a-zA-Z0-9]*)", |lex| lex.slice().to_owned())]
    Identifier(String),
    #[regex("[0-9]+", parse_int)]
    Integer(i64),
    #[regex(r"[0-9]+\.[0-9]+", parse_float)]
    Float(f64),
    #[regex(r#"""#, lex_string)]
    String(String),

    // TODO: Propagate error for unexpected tokens.
    #[error]
    #[regex(r"[ \t\r\n\f]+|//[^\r\n]+", logos::skip)]
    Error,
}

#[derive(Debug, Default, Clone)]
pub struct LexerResult(Rc<RefCell<Option<LexerError>>>);

impl LexerResult {
    fn error(&self) -> Option<LexerError> {
        self.0.borrow().clone()
    }
}

fn parse_int(lexer: &mut Lexer<TokenKind>) -> Option<i64> {
    match lexer.slice().parse() {
        Ok(int) => Some(int),
        Err(err) => {
            *lexer.extras.0.borrow_mut() = Some(LexerError::from(err));
            None
        }
    }
}

fn parse_float(lexer: &mut Lexer<TokenKind>) -> Option<f64> {
    match lexer.slice().parse() {
        Ok(float) => Some(float),
        Err(err) => {
            *lexer.extras.0.borrow_mut() = Some(LexerError::from(err));
            None
        }
    }
}

fn lex_string(lexer: &mut Lexer<TokenKind>) -> Option<String> {
    let remainder = lexer.remainder();
    let mut result = String::new();
    let mut chars = remainder.chars();
    let mut bump_count = 0;

    while let Some(current) = chars.next() {
        match current {
            '\\' => {
                let next = chars.next();

                match next {
                    Some('"') => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    Some(next) => {
                        let sequence = format!("{}{}", current, next);
                        *lexer.extras.0.borrow_mut() = Some(LexerError::UnrecognizedEscapeSequence(sequence));
                        return None;
                    }
                    None => {
                        *lexer.extras.0.borrow_mut() = Some(LexerError::UnterminatedString);
                        return None;
                    }
                }

                bump_count += current.len_utf8();
                bump_count += next.unwrap().len_utf8();
            }
            '"' => {
                bump_count += current.len_utf8();
                break;
            }
            _ => {
                bump_count += current.len_utf8();
                result.push(current);
            }
        }
    }

    if bump_count >= remainder.len() {
        *lexer.extras.0.borrow_mut() = Some(LexerError::UnterminatedString);
        return None;
    }

    lexer.bump(bump_count);

    Some(result)
}
