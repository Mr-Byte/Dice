mod token;

use crate::lexer::token::TokenIter;
use dice_core::{
    error::{codes::UNEXPECTED_TOKEN, Error},
    error_tags,
    source::Source,
};
use std::iter::Peekable;
pub use token::{Token, TokenKind};

pub struct Lexer<'a> {
    current: Token<'a>,
    tokens: Peekable<TokenIter<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn from_source(input: &'a Source) -> Lexer<'a> {
        let tokens = Token::tokenize(input).peekable();

        Lexer {
            tokens,
            current: Token::end_of_input(),
        }
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn next(&mut self) -> Result<&Token<'a>, Error> {
        self.current = self.tokens.next().transpose()?.unwrap_or_else(Token::end_of_input);
        Ok(&self.current)
    }

    pub fn peek(&mut self) -> Result<Token<'a>, Error> {
        self.tokens.peek().cloned().unwrap_or_else(|| Ok(Token::end_of_input()))
    }

    pub fn consume(&mut self, kind: TokenKind) -> Result<&Token, Error> {
        let next = self.next()?;
        if next.kind == kind {
            Ok(next)
        } else {
            Err(Error::new(UNEXPECTED_TOKEN).with_span(next.span).with_tags(error_tags! {
                expected => kind.to_string(),
                actual => next.kind.to_string()
            }))
        }
    }

    pub fn consume_ident(&mut self) -> Result<(&Token, String), Error> {
        let next = self.next()?;
        if let TokenKind::Identifier = next.kind {
            let ident = next.slice.to_owned();

            Ok((next, ident))
        } else {
            Err(Error::new(UNEXPECTED_TOKEN).with_span(next.span).with_tags(error_tags! {
                // TODO: Revamp tokens to be easier to list the kind of.
                actual => next.kind.to_string()
            }))
        }
    }

    pub fn consume_string(&mut self) -> Result<(&Token, String), Error> {
        let next = self.next()?;
        if let TokenKind::String = next.kind {
            let string = next.slice.to_owned();

            Ok((next, string))
        } else {
            Err(Error::new(UNEXPECTED_TOKEN).with_span(next.span).with_tags(error_tags! {
                actual => next.kind.to_string()
            }))
        }
    }

    pub fn consume_one_of(&mut self, kinds: &[TokenKind]) -> Result<&Token, Error> {
        let next = self.next()?;
        if kinds.contains(&next.kind) {
            Ok(next)
        } else {
            Err(Error::new(UNEXPECTED_TOKEN).with_span(next.span).with_tags(error_tags! {
                expected => kinds.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "),
                actual => next.kind.to_string()
            }))
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use dice_core::source::SourceKind;

    macro_rules! assert_next_token {
        ($tokens:expr, $token:pat) => {
            matches!($tokens.next(), Some(Ok($crate::lexer::Token { kind: $token, .. })))
        };
    }

    #[test]
    fn tokenize_delimiters() {
        let delimiters = Source::new("( ) { } [ ] : ,", SourceKind::Script);
        let mut tokens = Token::tokenize(&delimiters);

        assert_next_token!(tokens, TokenKind::LeftParen);
        assert_next_token!(tokens, TokenKind::RightParen);
        assert_next_token!(tokens, TokenKind::LeftCurly);
        assert_next_token!(tokens, TokenKind::RightCurly);
        assert_next_token!(tokens, TokenKind::LeftSquare);
        assert_next_token!(tokens, TokenKind::RightSquare);
        assert_next_token!(tokens, TokenKind::Colon);
        assert_next_token!(tokens, TokenKind::Comma);
    }

    #[test]
    fn tokenize_operators() {
        let delimiters = Source::new(".. ..= -> => . ?? % - + * / ! != == > >= < <= = d && ||", SourceKind::Script);
        let mut tokens = Token::tokenize(&delimiters);

        assert_next_token!(tokens, TokenKind::RangeInclusive);
        assert_next_token!(tokens, TokenKind::RangeExclusive);
        assert_next_token!(tokens, TokenKind::Arrow);
        assert_next_token!(tokens, TokenKind::WideArrow);
        assert_next_token!(tokens, TokenKind::Dot);
        assert_next_token!(tokens, TokenKind::Coalesce);
        assert_next_token!(tokens, TokenKind::Minus);
        assert_next_token!(tokens, TokenKind::Remainder);
        assert_next_token!(tokens, TokenKind::Plus);
        assert_next_token!(tokens, TokenKind::Star);
        assert_next_token!(tokens, TokenKind::Slash);
        assert_next_token!(tokens, TokenKind::Not);
        assert_next_token!(tokens, TokenKind::NotEqual);
        assert_next_token!(tokens, TokenKind::Equal);
        assert_next_token!(tokens, TokenKind::Greater);
        assert_next_token!(tokens, TokenKind::GreaterEqual);
        assert_next_token!(tokens, TokenKind::Less);
        assert_next_token!(tokens, TokenKind::LessEqual);
        assert_next_token!(tokens, TokenKind::Assign);
        assert_next_token!(tokens, TokenKind::DiceRoll);
        assert_next_token!(tokens, TokenKind::LazyAnd);
        assert_next_token!(tokens, TokenKind::Pipe);
    }

    #[test]
    fn tokenize_literals() {
        let delimiters = Source::new(r#"1 -1 +1 1.0 -1.0 +1.0 abc _abc _123 "abc" "abc\"abc""#, SourceKind::Script);
        let mut tokens = Token::tokenize(&delimiters);

        assert_next_token!(tokens, TokenKind::Integer);
        assert_next_token!(tokens, TokenKind::Integer);
        assert_next_token!(tokens, TokenKind::Integer);
        assert_next_token!(tokens, TokenKind::Float);
        assert_next_token!(tokens, TokenKind::Float);
        assert_next_token!(tokens, TokenKind::Float);
        assert_next_token!(tokens, TokenKind::Identifier);
        assert_next_token!(tokens, TokenKind::Identifier);
        assert_next_token!(tokens, TokenKind::Identifier);
        assert_next_token!(tokens, TokenKind::String);
        assert_next_token!(tokens, TokenKind::String);
    }

    #[test]
    fn tokenize_keywords() {
        let delimiters = Source::new(
            "
            false
            true
            none
            if
            else
            while
            do
            loop
            for
            break
            continue
            return
            yield
            fn
            let
            const
            match
            in
            operator
            static
            class
            struct
            type
            is
            enum
            where
            import
            from
        ",
            SourceKind::Script,
        );
        let mut tokens = Token::tokenize(&delimiters);

        assert_next_token!(tokens, TokenKind::False);
        assert_next_token!(tokens, TokenKind::True);
        assert_next_token!(tokens, TokenKind::Null);
        assert_next_token!(tokens, TokenKind::If);
        assert_next_token!(tokens, TokenKind::Else);
        assert_next_token!(tokens, TokenKind::While);
        assert_next_token!(tokens, TokenKind::Reserved);
        assert_next_token!(tokens, TokenKind::Loop);
        assert_next_token!(tokens, TokenKind::For);
        assert_next_token!(tokens, TokenKind::Break);
        assert_next_token!(tokens, TokenKind::Return);
        assert_next_token!(tokens, TokenKind::Reserved);
        assert_next_token!(tokens, TokenKind::Continue);
        assert_next_token!(tokens, TokenKind::Let);
        assert_next_token!(tokens, TokenKind::Reserved);
        assert_next_token!(tokens, TokenKind::Reserved);
        assert_next_token!(tokens, TokenKind::In);
        assert_next_token!(tokens, TokenKind::Operator);
        assert_next_token!(tokens, TokenKind::Class);
        assert_next_token!(tokens, TokenKind::Reserved);
        assert_next_token!(tokens, TokenKind::Is);
        assert_next_token!(tokens, TokenKind::Reserved);
        assert_next_token!(tokens, TokenKind::Import);
        assert_next_token!(tokens, TokenKind::From);
    }

    #[test]
    fn tokenize_errors() {
        let delimiters = Source::new(r#"❤ @ \ ^"#, SourceKind::Script);
        let mut tokens = Token::tokenize(&delimiters);

        assert_next_token!(tokens, TokenKind::Error);
        assert_next_token!(tokens, TokenKind::Error);
        assert_next_token!(tokens, TokenKind::Error);
        assert_next_token!(tokens, TokenKind::Error);
    }

    #[test]
    fn tokenize_comment_yields_no_tokens() {
        let delimiters = Source::new(r#"// test"#, SourceKind::Script);
        let mut tokens = Token::tokenize(&delimiters);

        assert!(tokens.next().is_none());
    }

    #[test]
    fn tokenize_token_followed_by_comment_yields_one_token() {
        let delimiters = Source::new(r#"12 // test"#, SourceKind::Script);
        let mut tokens = Token::tokenize(&delimiters);

        assert_next_token!(tokens, TokenKind::Integer);
        assert!(tokens.next().is_none());
    }

    #[test]
    fn tokenize_token_followed_by_comment_followed_by_token_on_newline_yields_two_tokens() {
        let delimiters = Source::new(r#"12 // test\n14"#, SourceKind::Script);
        let mut tokens = Token::tokenize(&delimiters);

        assert_next_token!(tokens, TokenKind::Integer);
        assert_next_token!(tokens, TokenKind::Integer);
        assert!(tokens.next().is_none());
    }
}
