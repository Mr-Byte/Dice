mod token;

use crate::SyntaxError;
use std::collections::VecDeque;

pub use token::{Token, TokenKind};

pub struct Lexer {
    current: Token,
    tokens: VecDeque<Result<Token, SyntaxError>>,
}

impl Lexer {
    pub fn from_str(input: &str) -> Lexer {
        let tokens = Token::tokenize(input).collect::<VecDeque<_>>();

        Lexer {
            tokens,
            current: Token::end_of_input(),
        }
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn next(&mut self) -> Result<Token, SyntaxError> {
        self.current = self.tokens.pop_front().transpose()?.unwrap_or_else(Token::end_of_input);
        Ok(self.current.clone())
    }

    pub fn peek(&self) -> Result<Token, SyntaxError> {
        Ok(self
            .tokens
            .front()
            .cloned()
            .transpose()?
            .unwrap_or_else(Token::end_of_input))
    }

    pub fn consume(&mut self, kind: TokenKind) -> Result<Token, SyntaxError> {
        let next = self.next()?;
        if next.kind == kind {
            Ok(next)
        } else {
            todo!("Unexpected token.")
        }
    }

    pub fn consume_ident(&mut self) -> Result<(Token, String), SyntaxError> {
        let next = self.next()?;
        if let TokenKind::Identifier(ref ident) = next.kind {
            let ident = ident.clone();
            Ok((next, ident))
        } else {
            todo!("Unexpected token.")
        }
    }

    pub fn consume_string(&mut self) -> Result<(Token, String), SyntaxError> {
        let next = self.next()?;
        if let TokenKind::String(ref string) = next.kind {
            let string = string.to_owned();
            Ok((next, string))
        } else {
            todo!("Unexpected token.")
        }
    }

    pub fn consume_one_of(&mut self, kinds: &[TokenKind]) -> Result<Token, SyntaxError> {
        let next = self.next()?;
        if kinds.contains(&next.kind) {
            Ok(next)
        } else {
            todo!("Unexpected token.")
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    macro_rules! assert_next_token {
        ($tokens:expr, $token:pat) => {
            matches!($tokens.next(), Some(Ok($crate::lexer::Token { kind: $token, .. })))
        };
    }

    #[test]
    fn tokenize_delimiters() {
        let delimiters = "( ) { } [ ] ; : ,";
        let mut tokens = Token::tokenize(delimiters);

        assert_next_token!(tokens, TokenKind::LeftParen);
        assert_next_token!(tokens, TokenKind::RightParen);
        assert_next_token!(tokens, TokenKind::LeftCurly);
        assert_next_token!(tokens, TokenKind::RightCurly);
        assert_next_token!(tokens, TokenKind::LeftSquare);
        assert_next_token!(tokens, TokenKind::RightSquare);
        assert_next_token!(tokens, TokenKind::Semicolon);
        assert_next_token!(tokens, TokenKind::Colon);
        assert_next_token!(tokens, TokenKind::Comma);
    }

    #[test]
    fn tokenize_operators() {
        let delimiters = ".. ..= -> => . ?? % - + * / ! != == > >= < <= = d && ||";
        let mut tokens = Token::tokenize(delimiters);

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
        let delimiters = r#"1 -1 +1 1.0 -1.0 +1.0 abc _abc _123 "abc" "abc\"abc""#;
        let mut tokens = Token::tokenize(delimiters);

        assert_next_token!(tokens, TokenKind::Integer(_));
        assert_next_token!(tokens, TokenKind::Integer(_));
        assert_next_token!(tokens, TokenKind::Integer(_));
        assert_next_token!(tokens, TokenKind::Float(_));
        assert_next_token!(tokens, TokenKind::Float(_));
        assert_next_token!(tokens, TokenKind::Float(_));
        assert_next_token!(tokens, TokenKind::Identifier(_));
        assert_next_token!(tokens, TokenKind::Identifier(_));
        assert_next_token!(tokens, TokenKind::Identifier(_));
        assert_next_token!(tokens, TokenKind::String(_));
        assert_next_token!(tokens, TokenKind::String(_));
    }

    #[test]
    fn tokenize_keywords() {
        let delimiters = "
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
        ";
        let mut tokens = Token::tokenize(delimiters);

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
        let delimiters = r#"❤ @ \ ^"#;
        let mut tokens = Token::tokenize(delimiters);

        assert_next_token!(tokens, TokenKind::Error);
        assert_next_token!(tokens, TokenKind::Error);
        assert_next_token!(tokens, TokenKind::Error);
        assert_next_token!(tokens, TokenKind::Error);
    }

    #[test]
    fn tokenize_comment_yields_no_tokens() {
        let delimiters = r#"// test"#;
        let mut tokens = Token::tokenize(delimiters);

        assert!(tokens.next().is_none());
    }

    #[test]
    fn tokenize_token_followed_by_comment_yields_one_token() {
        let delimiters = r#"12 // test"#;
        let mut tokens = Token::tokenize(delimiters);

        assert_next_token!(tokens, TokenKind::Integer(_));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn tokenize_token_followed_by_comment_followed_by_token_on_newline_yields_two_tokens() {
        let delimiters = r#"12 // test\n14"#;
        let mut tokens = Token::tokenize(delimiters);

        assert_next_token!(tokens, TokenKind::Integer(_));
        assert_next_token!(tokens, TokenKind::Integer(_));
        assert!(tokens.next().is_none());
    }
}
