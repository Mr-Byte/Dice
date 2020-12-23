use colored::Colorize;

use crate::{
    error::{
        localization::{localize_error_code, Locale},
        Error,
    },
    source::Source,
};
use std::fmt::Write;

use super::{context::ContextKind, localization::localize_context_msg_id};

pub trait ErrorFormatter {
    fn fmt(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result;
    fn fmt_pretty(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result;
}

pub struct HumanReadableErrorFormatter {
    should_colorize: bool,
}

impl HumanReadableErrorFormatter {
    pub const fn new(should_colorize: bool) -> Self {
        Self { should_colorize }
    }
}

impl ErrorFormatter for HumanReadableErrorFormatter {
    fn fmt(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result {
        HumanReadableErrorFormatter::fmt_message(buffer, error, locale)?;

        if let Some(source) = &error.source_code {
            HumanReadableErrorFormatter::fmt_position(buffer, error, source)?;
        }

        Ok(())
    }

    fn fmt_pretty(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result {
        if !self.should_colorize {
            colored::control::set_override(false);
        }

        HumanReadableErrorFormatter::fmt_message(buffer, error, locale)?;

        if let Some(source) = &error.source_code {
            HumanReadableErrorFormatter::fmt_position(buffer, error, source)?;
            HumanReadableErrorFormatter::fmt_source(buffer, error, &source)?;
        }

        HumanReadableErrorFormatter::fmt_context(buffer, error, locale)?;
        HumanReadableErrorFormatter::fmt_trace(buffer, error)?;

        if !self.should_colorize {
            colored::control::unset_override();
        }

        Ok(())
    }
}

impl HumanReadableErrorFormatter {
    fn fmt_position(buffer: &mut impl Write, error: &Error, source: &Source) -> std::fmt::Result {
        let position = source.line_index().position_of(error.span.start);
        let arrow = "  -->".cyan().bold();

        if let Some(path) = source.path() {
            writeln!(
                buffer,
                "{} {}:{}:{}",
                arrow,
                path,
                position.line + 1,
                position.column_utf16 + 1
            )
        } else {
            writeln!(
                buffer,
                "{} <Script>:{}:{}",
                arrow,
                position.line + 1,
                position.column_utf16 + 1
            )
        }
    }

    fn fmt_message(buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result {
        let localized_message = localize_error_code(error.error_code, &error.tags, locale);
        let error_code = format!("error[{}]: ", error.error_code).red();
        let message = format!("{}{}", error_code, localized_message).bold();

        writeln!(buffer, "{}", message)
    }

    fn fmt_context(buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result {
        for context in &error.context {
            let localized_message = localize_context_msg_id(context.msg_id, &context.tags, locale);
            match context.kind {
                ContextKind::Note => write!(buffer, "{}", "note: ".green().bold()),
                ContextKind::Help => write!(buffer, "{}", "help: ".yellow().bold()),
            }?;

            writeln!(buffer, "{}", localized_message)?;
        }

        Ok(())
    }

    fn fmt_source(buffer: &mut impl Write, error: &Error, source: &Source) -> std::fmt::Result {
        let lines = source.line_index().lines(error.span).collect::<Vec<_>>();
        let empty_line = "     |".cyan().bold();

        if !lines.is_empty() {
            writeln!(buffer, "{}", empty_line)?;

            for line in &lines {
                let position = source.line_index().position_of(line.start);
                let line_no = format!("{:<4} | ", position.line + 1).cyan().bold();
                writeln!(buffer, "{} {}", line_no, &source.source()[line.range()].trim_end())?;
            }

            writeln!(buffer, "{}", empty_line)?;
        }

        Ok(())
    }

    fn fmt_trace(buffer: &mut impl Write, error: &Error) -> std::fmt::Result {
        if !error.trace.is_empty() {
            writeln!(buffer)?;
            writeln!(buffer, "Trace:")?;

            for trace in error.trace.iter().rev() {
                let position = trace.source.line_index().position_of(trace.span.start);

                if let Some(path) = &trace.source.path() {
                    writeln!(
                        buffer,
                        "  Location: {}:{}:{}",
                        path,
                        position.line + 1,
                        position.column_utf16 + 1
                    )?;
                } else {
                    writeln!(
                        buffer,
                        "  Location: <Script>:{}:{}",
                        position.line + 1,
                        position.column_utf16 + 1
                    )?;
                }

                for line in trace.source.line_index().lines(trace.span) {
                    writeln!(
                        buffer,
                        "    {:<4} | {}",
                        position.line + 1,
                        &trace.source.source()[line.range()].trim()
                    )?
                }
            }
        }

        Ok(())
    }
}
