use crate::source::Source;
use crate::span::Span;
use std::fmt::{Debug, Display, Formatter};

#[derive(thiserror::Error)]
pub struct Error {
    error_code: &'static str,
    span: Span,
    source_code: Option<Source>,
}

impl Error {
    pub const fn new(error_code: &'static str) -> Self {
        Self {
            error_code,
            span: Span::empty(),
            source_code: None,
        }
    }

    pub const fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn with_source(mut self, source: Source) -> Self {
        self.source_code = Some(source);
        self
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

pub mod codes {
    pub static INTERNAL_COMPILER_ERROR: &str = "E000";

    // Syntax errors

    // Compiler errors

    // Runtime errors
}

// TODO: Do more to pretty print errors.
// pub fn format_error(&self, error: &impl SpannedError) -> String {
//     let span = error.span();
//     let position = self.line_index.position_of(span.start);
//     let lines = self
//         .line_index
//         .lines(span)
//         .map(|span| {
//             let position = self.line_index.position_of(span.start);
//             format!("{:<4} | {}", position.line + 1, &self.source[span.range()].trim_end())
//         })
//         .collect::<Vec<_>>()
//         .join("\n");
//     let location = self
//         .path
//         .clone()
//         .map(|path| format!("{}:{}:{}", path, position.line + 1, position.column_utf16 + 1))
//         .unwrap_or_else(|| format!("<Script>:{}:{}", position.line + 1, position.column_utf16 + 1));
//
//     format!("error: {}\n  --> {}\n{}", error.message(), location, lines)
// }

// #[test]
// fn format_error() {
//     let source = Source::new("let x = 0;\nx = 1;", SourceKind::Script);
//     let error = TestError(
//         String::from("Immutable variable cannot be reassigned."),
//         Span::new(11..17),
//     );
//
//     let expected = "\
// error: Immutable variable cannot be reassigned.
//   --> <Script>:2:1
// 2    | x = 1;";
//
//     assert_eq!(expected, source.format_error(&error));
// }
//
// #[test]
// fn format_error_multi_line() {
//     let source = Source::new("let x = 0;\nx = 1;\nx = 1;\ntest", SourceKind::Script);
//     let error = TestError(
//         String::from("Immutable variable cannot be reassigned."),
//         Span::new(11..23),
//     );
//
//     let expected = "\
// error: Immutable variable cannot be reassigned.
//   --> <Script>:2:1
// 2    | x = 1;
// 3    | x = 1;";
//
//     assert_eq!(expected, source.format_error(&error));
// }
