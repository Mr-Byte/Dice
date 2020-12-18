use crate::{error::codes::ErrorCode, source::Source, span::Span};
use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::loader::langid;
use fluent_templates::{LanguageIdentifier, Loader};
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

// TODO: Pull localization out into its own module.
fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./resources/locales",
        fallback_language: "en-US",
        // TODO: Make this configurable (most likely via Cargo features).
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

const US_ENGLISH: LanguageIdentifier = langid!("en-US");

pub type TagsMap = Vec<(&'static str, String)>;

#[derive(thiserror::Error, Clone)]

pub struct Error {
    error_code: ErrorCode,
    source_code: Option<Source>,
    span: Span,
    tags: TagsMap,
}

impl Error {
    pub const fn new(error_code: ErrorCode) -> Self {
        Self {
            error_code,
            source_code: None,
            span: Span::empty(),
            tags: TagsMap::new(),
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

    pub fn with_tags(mut self, tags: TagsMap) -> Self {
        self.tags = tags;
        self
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error[{}]", self.error_code)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO: Get a formatted error message.
        let args = self
            .tags
            .iter()
            .map(|(key, value)| (key.to_string(), FluentValue::String(std::borrow::Cow::Borrowed(value))))
            .collect::<HashMap<_, _>>();

        let error_message = LOCALES.lookup_with_args(&US_ENGLISH, self.error_code, &args);
        writeln!(f, "error[{}]: {}", self.error_code, error_message)?;

        if let Some(source) = &self.source_code {
            let position = source.line_index().position_of(self.span.start);
            let location = source
                .path()
                .map(|path| format!("{}:{}:{}", path, position.line + 1, position.column_utf16 + 1))
                .unwrap_or_else(|| format!("<Script>:{}:{}", position.line + 1, position.column_utf16 + 1));

            writeln!(f, "  --> {}", location)?;

            let lines = source
                .line_index()
                .lines(self.span)
                .map(|span| {
                    let position = source.line_index().position_of(span.start);

                    format!(
                        "{:<4} | {}",
                        position.line + 1,
                        &source.source()[span.range()].trim_end()
                    )
                })
                .collect::<Vec<_>>();

            if !lines.is_empty() {
                writeln!(f, "     |")?;
                writeln!(f, "{}", lines.join("\n"))?;
                writeln!(f, "     |")?;
            }
        }

        Ok(())
    }
}

pub trait ResultExt {
    fn with_source(self, source: impl Fn() -> Source) -> Self;
    fn with_span(self, span: Span) -> Self;
    fn with_tags(self, tags: TagsMap) -> Self;
}

impl<T> ResultExt for Result<T, Error> {
    fn with_source(self, source: impl Fn() -> Source) -> Self {
        self.map_err(|error| error.with_source(source()))
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
            tags.push((stringify!($tag), $value));
        )*

        tags
    }}
}

pub mod codes {
    pub type ErrorCode = &'static str;

    // Syntax errors
    pub static UNEXPECTED_TOKEN: ErrorCode = "E1000";
    pub static INVALID_ESCAPE_SEQUENCE: ErrorCode = "E1001";
    pub static UNTERMINATED_STRING: ErrorCode = "E1002";

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
    pub static FUNCTION_CANNOT_HAVE_DUPLICATE_ARGS: ErrorCode = "E2206";

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
