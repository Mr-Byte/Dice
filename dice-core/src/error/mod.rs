use crate::{error::codes::ErrorCode, source::Source, span::Span};
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
    hash::BuildHasherDefault,
};
use wyhash::WyHash;

pub type TagsMap = HashMap<&'static str, String, BuildHasherDefault<WyHash>>;

#[derive(thiserror::Error, Clone)]

pub struct Error {
    error_code: ErrorCode,
    source_code: Option<Source>,
    span: Span,
    tags: TagsMap,
}

impl Error {
    pub fn new(error_code: ErrorCode) -> Self {
        Self {
            error_code,
            source_code: None,
            span: Span::empty(),
            tags: TagsMap::default(),
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn with_source(mut self, source: Source) -> Self {
        self.source_code = Some(source);
        self
    }

    pub fn with_tags(mut self, tags: TagsMap) -> Self {
        self.tags = tags;
        self
    }
}

impl Debug for Error {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Display for Error {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

pub trait ResultExt {
    fn with_source(self, source: Source) -> Self;
    fn with_span(self, span: Span) -> Self;
    fn with_tags(self, tags: TagsMap) -> Self;
}

impl<T> ResultExt for Result<T, Error> {
    fn with_source(self, source: Source) -> Self {
        self.map_err(|error| error.with_source(source))
    }

    fn with_span(self, span: Span) -> Self {
        self.map_err(|error| error.with_span(span))
    }

    fn with_tags(self, tags: TagsMap) -> Self {
        self.map_err(|error| error.with_tags(tags))
    }
}

#[macro_export]
macro_rules! error_tags {
    ($($tag:ident => $value:expr),*) => {{
        let mut tags = $crate::error::TagsMap::default();

        $(
            tags.insert(stringify!($tag), $value);
        )*

        tags
    }}
}

pub mod codes {
    pub type ErrorCode = &'static str;

    // Syntax errors
    pub static INVALID_ESCAPE_SEQUENCE: ErrorCode = "E1000";
    pub static UNTERMINATED_STRING: ErrorCode = "E1001";

    // Compiler errors
    pub static INTERNAL_COMPILER_ERROR: ErrorCode = "E2000";

    pub static TOO_MANY_UPVALUES: ErrorCode = "E2100";
    pub static TOO_MANY_CONSTANTS: ErrorCode = "E2101";

    pub static NEW_METHOD_CANNOT_HAVE_RETURN_TYPE: ErrorCode = "E2200";
    pub static NEW_METHOD_MUST_HAVE_RECEIVER: ErrorCode = "E2201";
    pub static NEW_MUST_CALL_SUPER_FROM_SUBCLASS: ErrorCode = "E2202";
    pub static NEW_RETURN_CANNOT_HAVE_EXPRESSION: ErrorCode = "E2203";
    pub static INVALID_SUPER_CALL: ErrorCode = "E2204";
    pub static METHOD_RECEIVER_CANNOT_HAVE_TYPE: ErrorCode = "E2205";
    pub static FUNCTION_CANNOT_HAVE_DUPLICATE_ARGS: ErrorCode = "E2207";

    pub static CLASS_ALREADY_DECLARED: ErrorCode = "E2300";
    pub static FUNCTION_ALREADY_DECLARE: ErrorCode = "E2301";

    pub static INVALID_ASSIGNMENT_TARGET: ErrorCode = "E2400";
    pub static VARIABLE_NOT_DECLARED: ErrorCode = "E2401";
    pub static VARIABLE_NOT_INITIALIZED: ErrorCode = "E2402";
    pub static CANNOT_REASSIGN_IMMUTABLE_VARIABLE: ErrorCode = "E2403";

    pub static INVALID_RETURN_USAGE: ErrorCode = "E2500";
    pub static INVALID_BREAK_USAGE: ErrorCode = "E2501";
    pub static INVALID_CONTINUE_USAGE: ErrorCode = "E2502";
    pub static INVALID_ERROR_PROPAGATE_USAGE: ErrorCode = "E2503";

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
