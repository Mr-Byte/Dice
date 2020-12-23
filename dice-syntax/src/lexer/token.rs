use dice_core::{
    error::{
        codes::{INVALID_ESCAPE_SEQUENCE, UNTERMINATED_STRING},
        Error, ResultExt as _,
    },
    source::Source,
    span::Span,
};
use logos::{Lexer, Logos};
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    iter::Iterator,
    rc::Rc,
};

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub span: Span,
    pub slice: &'a str,
}

impl<'a> Token<'a> {
    pub fn tokenize(input: &'a Source) -> TokenIter<'a> {
        TokenIter::new(input)
    }

    pub const fn end_of_input(span: Span) -> Token<'a> {
        Self {
            kind: TokenKind::EndOfInput,
            span,
            slice: "",
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

pub struct TokenIter<'a> {
    source: &'a Source,
    lexer: logos::Lexer<'a, TokenKind>,
}

impl<'a> TokenIter<'a> {
    fn new(source: &'a Source) -> Self {
        let lexer = TokenKind::lexer(source.source());
        Self { lexer, source }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let span = Span::new(self.lexer.span());
        let slice = self.lexer.slice();
        let result = self
            .lexer
            .extras
            .error()
            .map_or_else(|| Ok(Token { kind, span, slice }), Err)
            .with_span(|| span)
            .with_source(|| self.source.clone());

        Some(result)
    }
}

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
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
    #[token("in")]
    In,
    #[token("op")]
    Operator,
    #[token("class")]
    Class,
    #[token("is")]
    Is,
    #[token("import")]
    Import,
    #[token("as")]
    As,
    #[token("from")]
    From,
    #[token("export")]
    Export,
    #[token("super")]
    Super,
    #[regex("await|async|yield|do|const|match|enum|trait|type|try|when")]
    Reserved,

    // Literals,
    #[regex("(d[_a-zA-Z][_a-zA-Z0-9]*)|([_a-ce-zA-Z][_a-zA-Z0-9]*)")]
    Identifier,
    #[regex("[0-9]+")]
    Integer,
    #[regex(r"[0-9]+\.[0-9]+")]
    Float,
    #[regex(r#"""#, lex_string)]
    String,

    #[error]
    #[regex(r"[ \t\r\n\f]+|//[^\r\n]+", logos::skip)]
    Error,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::EndOfInput => write!(f, "EOI"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftCurly => write!(f, "{{"),
            TokenKind::RightCurly => write!(f, "}}"),
            TokenKind::LeftSquare => write!(f, "["),
            TokenKind::RightSquare => write!(f, "]"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::RangeExclusive => write!(f, ".."),
            TokenKind::RangeInclusive => write!(f, "..="),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::WideArrow => write!(f, "=>"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::QuestionMark => write!(f, "?"),
            TokenKind::ErrorPropagate => write!(f, "!!"),
            TokenKind::Coalesce => write!(f, "??"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Remainder => write!(f, "%"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Not => write!(f, "!"),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::Equal => write!(f, "=="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::Assign => write!(f, "="),
            TokenKind::MulAssign => write!(f, "*="),
            TokenKind::DivAssign => write!(f, "/="),
            TokenKind::AddAssign => write!(f, "+="),
            TokenKind::SubAssign => write!(f, "-="),
            TokenKind::DiceRoll => write!(f, "d"),
            TokenKind::LazyAnd => write!(f, "&&"),
            TokenKind::Pipeline => write!(f, "|>"),
            TokenKind::Object => write!(f, "#"),
            TokenKind::False => write!(f, "false"),
            TokenKind::True => write!(f, "true"),
            TokenKind::Null => write!(f, "null"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Loop => write!(f, "loop"),
            TokenKind::For => write!(f, "for"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Function => write!(f, "fn"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Mut => write!(f, "mut"),
            TokenKind::In => write!(f, "in"),
            TokenKind::Operator => write!(f, "op"),
            TokenKind::Class => write!(f, "class"),
            TokenKind::Is => write!(f, "is"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::As => write!(f, "as"),
            TokenKind::From => write!(f, "from"),
            TokenKind::Export => write!(f, "export"),
            TokenKind::Super => write!(f, "super"),
            TokenKind::Reserved => write!(f, "reserved"),
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::Integer => write!(f, "integer"),
            TokenKind::Float => write!(f, "float"),
            TokenKind::String => write!(f, "string"),
            TokenKind::Error => write!(f, "error"),
        }
    }
}

#[derive(Default, Clone)]
pub struct LexerResult(Rc<RefCell<Option<Error>>>);

impl LexerResult {
    fn error(&self) -> Option<Error> {
        self.0.borrow_mut().take()
    }
}

fn lex_string(lexer: &mut Lexer<TokenKind>) -> bool {
    let remainder = lexer.remainder();
    let mut result = String::new();
    let mut chars = remainder.chars();
    let mut bump_count = 0;

    loop {
        match chars.next() {
            Some('\\') => {
                let next = chars.next();

                match next {
                    Some('"') => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    Some(next) => {
                        *lexer.extras.0.borrow_mut() =
                            Some(Error::new(INVALID_ESCAPE_SEQUENCE).with_tags(dice_core::tags! {
                                sequence => format!("\\{}", next)
                            }));
                        return false;
                    }
                    None => {
                        *lexer.extras.0.borrow_mut() = Some(Error::new(UNTERMINATED_STRING));
                        return false;
                    }
                }

                bump_count += '\\'.len_utf8();
                bump_count += next.unwrap().len_utf8();
            }
            Some('"') => {
                bump_count += '"'.len_utf8();
                break;
            }
            Some(current) => {
                bump_count += current.len_utf8();
                result.push(current);
            }
            None => {
                *lexer.extras.0.borrow_mut() = Some(Error::new(UNTERMINATED_STRING));
                return false;
            }
        }
    }

    lexer.bump(bump_count);

    true
}
