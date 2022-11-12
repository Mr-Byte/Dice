use logos::Logos;

use super::lexer_result::LexerResult;

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[logos(extras = LexerResult)]
pub enum DiceTokenKind {
    #[regex("[0-9]+")]
    Integer,
    #[regex(r"[0-9]+\.[0-9]+")]
    Float,
    #[regex(r"d")]
    DiceRoll,
    #[regex(r"kh")]
    KeepHighest,
    #[regex(r"kl")]
    KeepLowest,

    #[error]
    #[regex(r"[ \t\r\n\f]+|//[^\r\n]+", logos::skip)]
    Error,
}
