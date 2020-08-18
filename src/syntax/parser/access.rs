use super::{error::ErrorKind, ParseResult, Parser};
use crate::{
    runtime::core::Symbol,
    syntax::{lexer::TokenKind, ParserError, SyntaxTree},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_accessor(&mut self) -> ParseResult {
        let mut expression = self.parse_literal()?;

        while self.next_token.is_any_kind(&[
            TokenKind::Dot,
            TokenKind::SafeDot,
            TokenKind::LeftParen,
            TokenKind::LeftSquare,
        ]) {
            self.next();

            expression = match &self.current_token.kind {
                TokenKind::Dot | TokenKind::SafeDot => self.parse_field_access(expression)?,
                TokenKind::LeftSquare => self.parse_index_access(expression)?,
                TokenKind::LeftParen => self.parse_function_call(expression)?,
                _ => unreachable!(),
            }
        }

        Ok(expression)
    }

    fn parse_field_access(&mut self, expression: SyntaxTree) -> ParseResult {
        let span_start = self.current_token.span();
        let operator = self.current_token.clone();
        self.next();

        if self.current_token.is_kind(TokenKind::Identifier) {
            let symbol: Symbol = self.current_token.slice().into();
            let span_end = self.current_token.span();

            let result = match operator.kind {
                TokenKind::Dot => SyntaxTree::FieldAccess(Box::new(expression), symbol, span_start + span_end),
                TokenKind::SafeDot => SyntaxTree::SafeAccess(Box::new(expression), symbol, span_start + span_end),
                _ => unreachable!(),
            };

            Ok(result)
        } else {
            Err(ParserError::new(
                ErrorKind::UnexpectedToken {
                    expected: vec![TokenKind::Identifier],
                    found: self.current_token.kind,
                },
                Some(self.current_token.span()),
            ))
        }
    }

    fn parse_index_access(&mut self, expression: SyntaxTree) -> ParseResult {
        let index_expression = self.parse_expression()?;

        if !self.next_token.is_kind(TokenKind::RightSquare) {
            return Err(ParserError::unexpected_token(
                self.next_token.kind,
                &[TokenKind::RightSquare],
                Some(self.next_token.span()),
            ));
        }

        self.next();

        let span_start = expression.span();
        let span_end = self.current_token.span();

        Ok(SyntaxTree::Index(
            Box::new(expression),
            Box::new(index_expression),
            span_start + span_end,
        ))
    }

    fn parse_function_call(&mut self, expression: SyntaxTree) -> ParseResult {
        let mut args = Vec::new();

        while !self.next_token.is_kind(TokenKind::RightParen) {
            args.push(self.parse_expression()?);

            if self.next_token.is_kind(TokenKind::Comma) {
                self.next();
            } else if !self.next_token.is_kind(TokenKind::RightParen) {
                return Err(ParserError::unexpected_token(
                    self.next_token.kind,
                    &[TokenKind::Comma, TokenKind::RightParen],
                    Some(self.next_token.span()),
                ));
            }
        }

        self.next();
        let span_start = expression.span();
        let span_end = self.current_token.span();

        Ok(SyntaxTree::FunctionCall(
            Box::new(expression),
            args,
            span_start + span_end,
        ))
    }
}
