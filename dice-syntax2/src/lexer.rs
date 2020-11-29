use crate::syntax::SyntaxKind;
use logos::Logos;
use std::iter::Peekable;

pub(crate) struct Lexer<'a> {
    inner: Peekable<LexerIter<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let inner = LexerIter::new(input).peekable();

        Self { inner }
    }

    pub fn peek(&mut self) -> Option<SyntaxKind> {
        self.inner.peek().map(|kind| kind.0)
    }

    pub fn pop(&mut self) -> (SyntaxKind, &'a str) {
        self.inner.next().expect("Lexer went beyond end.")
    }
}

struct LexerIter<'a> {
    iter: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> LexerIter<'a> {
    fn new(input: &'a str) -> Self {
        let iter = SyntaxKind::lexer(input);

        Self { iter }
    }
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = (SyntaxKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|kind| (kind, self.iter.slice()))
    }
}
