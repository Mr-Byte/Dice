pub mod codes;
pub mod fmt;

mod localization;

use crate::{
    bytecode::Bytecode,
    error::{
        codes::ErrorCode,
        fmt::{ErrorFormatter, HumanReadableErrorFormatter},
        localization::Locale,
    },
    source::{LineColumn, Source},
    span::Span,
};
use std::fmt::{Debug, Display, Formatter};

pub type Tags = Vec<(&'static str, String)>;

#[derive(thiserror::Error, Clone)]
pub struct Error {
    error_code: ErrorCode,
    source_code: Option<Source>,
    span: Span,
    tags: Tags,
    // NOTE: Optional trace.
    trace: Vec<TraceLocation>,
}

impl Error {
    pub const fn new(error_code: ErrorCode) -> Self {
        Self {
            error_code,
            source_code: None,
            span: Span::empty(),
            tags: Tags::new(),
            trace: Vec::new(),
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

    pub fn with_tags(mut self, tags: Tags) -> Self {
        self.tags = tags;
        self
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        HumanReadableErrorFormatter.fmt(formatter, self, &Locale::US_ENGLISH)
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        HumanReadableErrorFormatter.fmt_pretty(formatter, self, &Locale::US_ENGLISH)
    }
}

#[derive(Debug, Clone)]
pub struct TraceLocation {
    pub source: Source,
    pub span: Span,
}

impl TraceLocation {
    pub fn from_bytecode(bytecode: &Bytecode, offset: u64) -> Self {
        let span = bytecode.source_map()[&offset];
        let source = bytecode.source().clone();

        Self { source, span }
    }
}

pub trait ResultExt {
    fn with_source(self, source: impl Fn() -> Source) -> Self;
    fn with_span(self, span: impl Fn() -> Span) -> Self;
    fn with_tags(self, tags: Tags) -> Self;
    fn with_stack_location(self, stack_location: impl Fn() -> TraceLocation) -> Self;
}

impl<T> ResultExt for Result<T, Error> {
    fn with_source(self, source: impl Fn() -> Source) -> Self {
        self.map_err(|error| error.with_source(source()))
    }

    fn with_span(self, span: impl Fn() -> Span) -> Self {
        self.map_err(|error| error.with_span(span()))
    }

    fn with_tags(self, tags: Tags) -> Self {
        self.map_err(|error| error.with_tags(tags))
    }

    fn with_stack_location(self, stack_location: impl Fn() -> TraceLocation) -> Self {
        self.map_err(|mut error| {
            error.trace.push(stack_location());
            error
        })
    }
}

#[macro_export]
macro_rules! error_tags {
    ($($tag:ident => $value:expr),*) => {{
        let mut tags = $crate::error::Tags::default();

        $(
            tags.push((stringify!($tag), $value));
        )*

        tags
    }}
}
