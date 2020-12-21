use crate::{
    error::{
        localization::{localize_error_code, Locale},
        Error,
    },
    source::Source,
};
use std::fmt::Write;

use super::localization::localize_context_msg_id;

pub trait ErrorFormatter {
    fn fmt(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result;
    fn fmt_pretty(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result;
}

pub struct HumanReadableErrorFormatter;

impl ErrorFormatter for HumanReadableErrorFormatter {
    fn fmt(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result {
        HumanReadableErrorFormatter::fmt_message(buffer, error, locale)?;

        if let Some(source) = &error.source_code {
            HumanReadableErrorFormatter::fmt_position(buffer, error, source)?;
        }

        Ok(())
    }

    fn fmt_pretty(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result {
        HumanReadableErrorFormatter::fmt_message(buffer, error, locale)?;
        HumanReadableErrorFormatter::fmt_context(buffer, error, locale)?;

        if let Some(source) = &error.source_code {
            HumanReadableErrorFormatter::fmt_position(buffer, error, source)?;

            let position = source.line_index().position_of(error.span.start);
            let lines = source.line_index().lines(error.span).collect::<Vec<_>>();

            if !lines.is_empty() {
                writeln!(buffer, "     |")?;

                for line in &lines {
                    writeln!(
                        buffer,
                        "{:<4} | {}",
                        position.line + 1,
                        &source.source()[line.range()].trim_end()
                    )?
                }

                writeln!(buffer, "     |")?;
            }
        }

        if !error.trace.is_empty() {
            writeln!(buffer)?;
            // TODO: Should this be localized? Yes! Do it sometime PEPW.
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

impl HumanReadableErrorFormatter {
    fn fmt_position(buffer: &mut impl Write, error: &Error, source: &Source) -> std::fmt::Result {
        let position = source.line_index().position_of(error.span.start);

        if let Some(path) = source.path() {
            writeln!(
                buffer,
                "  --> {}:{}:{}",
                path,
                position.line + 1,
                position.column_utf16 + 1
            )
        } else {
            writeln!(
                buffer,
                "  --> <Script>:{}:{}",
                position.line + 1,
                position.column_utf16 + 1
            )
        }
    }

    fn fmt_message(buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result {
        let localized_message = localize_error_code(error.error_code, &error.tags, locale);

        writeln!(buffer, "error[{}]: {}", error.error_code, localized_message)
    }

    fn fmt_context(buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result {
        if let Some(context_msg_id) = error.context_msg_id {
            let localized_message = localize_context_msg_id(context_msg_id, &error.context_tags, locale);

            writeln!(buffer, "{}", localized_message)?;
        }

        Ok(())
    }
}
