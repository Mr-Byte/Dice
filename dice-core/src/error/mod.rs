pub mod codes;
pub mod context;
pub mod fmt;
pub mod tag;
pub mod trace;

mod localization;

use self::{codes::IO_ERROR, trace::ErrorTrace};
use crate::{
    error::{
        codes::ErrorCode,
        fmt::{ErrorFormatter, HumanReadableErrorFormatter},
        localization::Locale,
    },
    source::Source,
    span::Span,
    tags,
};
use context::ErrorContext;
use std::fmt::{Debug, Display, Formatter};
use tag::Tags;

#[derive(thiserror::Error, Clone)]
pub struct Error {
    error_code: ErrorCode,
    source_code: Option<Source>,
    span: Span,
    tags: Tags,
    trace: Vec<ErrorTrace>,
    context: Vec<ErrorContext>,
}

impl Error {
    pub const fn new(error_code: ErrorCode) -> Self {
        Self {
            error_code,
            source_code: None,
            span: Span::empty(),
            tags: Tags::new(),
            trace: Vec::new(),
            context: Vec::new(),
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

    pub fn push_trace(mut self, trace: ErrorTrace) -> Self {
        self.trace.push(trace);
        self
    }

    pub fn push_context(mut self, context: ErrorContext) -> Self {
        self.context.push(context);
        self
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        HumanReadableErrorFormatter::new(false).fmt(formatter, self, &Locale::US_ENGLISH)
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        HumanReadableErrorFormatter::new(false).fmt_pretty(formatter, self, &Locale::US_ENGLISH)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(IO_ERROR).with_tags(tags! {
            message => error.to_string()
        })
    }
}

pub trait ResultExt {
    fn with_source(self, source: impl Fn() -> Source) -> Self;
    fn with_span(self, span: impl Fn() -> Span) -> Self;
    fn with_tags(self, tags: impl Fn() -> Tags) -> Self;
    fn push_trace(self, trace: impl Fn() -> ErrorTrace) -> Self;
    fn push_context(self, context: impl Fn() -> ErrorContext) -> Self;
}

impl<T> ResultExt for Result<T, Error> {
    fn with_source(self, source: impl Fn() -> Source) -> Self {
        self.map_err(|error| error.with_source(source()))
    }

    fn with_span(self, span: impl Fn() -> Span) -> Self {
        self.map_err(|error| error.with_span(span()))
    }

    fn with_tags(self, tags: impl Fn() -> Tags) -> Self {
        self.map_err(|error| error.with_tags(tags()))
    }

    fn push_trace(self, trace: impl Fn() -> ErrorTrace) -> Self {
        self.map_err(|error| error.push_trace(trace()))
    }

    fn push_context(self, context: impl Fn() -> ErrorContext) -> Self {
        self.map_err(|error| error.push_context(context()))
    }
}
