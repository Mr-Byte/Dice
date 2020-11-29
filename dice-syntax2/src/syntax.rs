use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

pub(crate) type SyntaxNode = rowan::SyntaxNode<Lang>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Lang;

impl rowan::Language for Lang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).expect("Invalid token value encountered.")
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().expect("Unable to convert token to u16."))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, logos::Logos, num_derive::FromPrimitive, num_derive::ToPrimitive)]
pub enum SyntaxKind {
    Root,
    InfixExpr,
    PrefixExpr,
    PostfixExpr,

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

    #[regex(r"[ \t\r\n\f]+")]
    Whitespace,
    #[regex(r"//[^\r\n]+")]
    Comment,

    #[error]
    Error,
}

fn lex_string(lexer: &mut logos::Lexer<SyntaxKind>) -> bool {
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
                    _ => return false,
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
            None => return false,
        }
    }

    lexer.bump(bump_count);

    true
}
