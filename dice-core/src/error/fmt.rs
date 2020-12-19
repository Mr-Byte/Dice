use crate::error::{
    localization::{localize_error_code, Locale},
    Error,
};
use std::fmt::Write;

pub trait ErrorFormatter {
    fn fmt(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result;
    fn fmt_pretty(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result;
}

pub struct HumanReadableErrorFormatter;

impl ErrorFormatter for HumanReadableErrorFormatter {
    fn fmt(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result {
        let localized_message = localize_error_code(error.error_code, &error.tags, locale);
        writeln!(buffer, "error[{}]: {}", error.error_code, localized_message)?;

        if let Some(source) = &error.source_code {
            let position = source.line_index().position_of(error.span.start);
            if let Some(path) = source.path() {
                write!(
                    buffer,
                    "  --> {}:{}:{}",
                    path,
                    position.line + 1,
                    position.column_utf16 + 1
                )?;
            } else {
                write!(
                    buffer,
                    "  --> <Script>:{}:{}",
                    position.line + 1,
                    position.column_utf16 + 1
                )?;
            }
        }

        Ok(())
    }

    fn fmt_pretty(&self, buffer: &mut impl Write, error: &Error, locale: &Locale) -> std::fmt::Result {
        let localized_message = localize_error_code(error.error_code, &error.tags, locale);
        writeln!(buffer, "error[{}]: {}", error.error_code, localized_message)?;

        if let Some(source) = &error.source_code {
            let position = source.line_index().position_of(error.span.start);
            if let Some(path) = source.path() {
                write!(
                    buffer,
                    "  --> {}:{}:{}",
                    path,
                    position.line + 1,
                    position.column_utf16 + 1
                )?;
            } else {
                write!(
                    buffer,
                    "  --> <Script>:{}:{}",
                    position.line + 1,
                    position.column_utf16 + 1
                )?;
            }

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

        Ok(())
    }
}
