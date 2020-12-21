pub mod codes;
pub mod context;
pub mod fmt;
pub mod tag;

mod localization;

use self::context::ContextMsgId;
use crate::{
    bytecode::Bytecode,
    error::{
        codes::ErrorCode,
        fmt::{ErrorFormatter, HumanReadableErrorFormatter},
        localization::Locale,
    },
    source::Source,
    span::Span,
    tags,
};
use codes::IO_ERROR;
use std::fmt::{Debug, Display, Formatter};
use tag::Tags;

#[derive(thiserror::Error, Clone)]
pub struct Error {
    error_code: ErrorCode,
    source_code: Option<Source>,
    span: Span,
    tags: Tags,
    trace: Vec<TraceLocation>,
    context_msg_id: Option<ContextMsgId>,
    context_tags: Tags,
}

impl Error {
    pub const fn new(error_code: ErrorCode) -> Self {
        Self {
            error_code,
            source_code: None,
            span: Span::empty(),
            tags: Tags::new(),
            trace: Vec::new(),
            context_msg_id: None,
            context_tags: Tags::new(),
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

    pub fn with_context(mut self, context_msg_id: ContextMsgId) -> Self {
        self.context_msg_id = Some(context_msg_id);
        self
    }

    pub fn with_context_tags(mut self, tags: Tags) -> Self {
        self.context_tags = tags;
        self
    }

    pub fn push_trace(mut self, trace: TraceLocation) -> Self {
        self.trace.push(trace);
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

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(IO_ERROR).with_tags(tags! {
            message => error.to_string()
        })
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
    fn with_tags(self, tags: impl Fn() -> Tags) -> Self;
    fn push_trace(self, trace: impl Fn() -> TraceLocation) -> Self;
    fn with_context(self, id: ContextMsgId) -> Self;
    fn with_context_tags(self, tags: impl Fn() -> Tags) -> Self;
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

    fn push_trace(self, trace: impl Fn() -> TraceLocation) -> Self {
        self.map_err(|error| error.push_trace(trace()))
    }

    fn with_context(self, id: ContextMsgId) -> Self {
        self.map_err(|error| error.with_context(id))
    }

    fn with_context_tags(self, tags: impl Fn() -> Tags) -> Self {
        self.map_err(|error| error.with_context_tags(tags()))
    }
}
