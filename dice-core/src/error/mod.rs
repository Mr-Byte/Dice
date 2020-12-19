pub mod codes;
pub mod fmt;

mod localization;

use crate::{
    error::{
        codes::ErrorCode,
        fmt::{ErrorFormatter, HumanReadableErrorFormatter},
        localization::Locale,
    },
    source::Source,
    span::Span,
};
use std::fmt::{Debug, Display, Formatter};

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
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        HumanReadableErrorFormatter.fmt(formatter, self, &Locale::US_ENGLISH)
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        HumanReadableErrorFormatter.fmt_pretty(formatter, self, &Locale::US_ENGLISH)
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
