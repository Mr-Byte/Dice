use crate::span::Span;
use std::{collections::HashMap, hash::BuildHasherDefault, iter, rc::Rc};
use wyhash::WyHash;

#[derive(Debug, Clone, Copy)]
pub struct LineColumn {
    pub line: usize,
    pub column_utf16: usize,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Utf16Character(Span);

impl Utf16Character {
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn len_utf16(&self) -> usize {
        if self.len() == 4 {
            2
        } else {
            1
        }
    }

    pub const fn start(&self) -> usize {
        self.0.start
    }

    pub const fn end(&self) -> usize {
        self.0.end
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SourceKind {
    Module,
    Script,
}

#[derive(Debug)]
pub struct SourceInner {
    path: Option<String>,
    source: String,
    line_index: LineIndex,
    kind: SourceKind,
}

#[derive(Debug, Clone)]
pub struct Source {
    inner: Rc<SourceInner>,
}

impl Source {
    pub fn new(source: impl Into<String>, kind: SourceKind) -> Self {
        let source = source.into();
        let line_index = LineIndex::new(&source);

        Self {
            inner: Rc::new(SourceInner {
                path: None,
                source,
                line_index,
                kind,
            }),
        }
    }

    pub fn with_path(source: impl Into<String>, path: impl Into<String>, kind: SourceKind) -> Self {
        let source = source.into();
        let line_index = LineIndex::new(&source);

        Self {
            inner: Rc::new(SourceInner {
                path: Some(path.into()),
                source,
                line_index,
                kind,
            }),
        }
    }

    pub fn path(&self) -> Option<&str> {
        self.inner.path.as_deref()
    }

    pub fn source(&self) -> &str {
        &self.inner.source
    }

    pub fn kind(&self) -> SourceKind {
        self.inner.kind
    }

    pub fn line_index(&self) -> &LineIndex {
        &self.inner.line_index
    }
}

pub type Utf16LineMap = HashMap<usize, Vec<Utf16Character>, BuildHasherDefault<WyHash>>;

#[derive(Debug, Clone, Default)]
pub struct LineIndex {
    newlines: Vec<usize>,
    utf16_lines: Utf16LineMap,
    utf8_len: usize,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut utf16_lines = Utf16LineMap::default();
        let mut utf16_characters = Vec::new();
        let mut newlines = vec![0];
        let mut current_line = 0;
        let mut current_column = 0;
        let mut current_row = 0;

        for character in source.chars() {
            let character_len = character.len_utf8();
            current_row += character_len;

            if character == '\n' {
                newlines.push(current_row);

                if !utf16_characters.is_empty() {
                    utf16_lines.insert(current_line, utf16_characters);
                    utf16_characters = Vec::new();
                }

                current_column = 0;
                current_line += 1;

                continue;
            }

            if !character.is_ascii() {
                let utf16_character = Utf16Character(Span::new(current_column..(current_column + character_len)));
                utf16_characters.push(utf16_character);
            }

            current_column += character_len;
        }

        if !utf16_characters.is_empty() {
            utf16_lines.insert(current_line, utf16_characters);
        }

        LineIndex {
            newlines,
            utf16_lines,
            utf8_len: source.len(),
        }
    }

    /// Retrieve the line and column position (in UTF-16 characters) from the given UTF-8 (byte) offset.
    pub fn position_of(&self, offset: usize) -> LineColumn {
        let line = partition_point(&self.newlines, |&it| it <= offset) - 1;
        let line_start = self.newlines[line];
        let column = offset - line_start;

        LineColumn {
            line,
            column_utf16: self.convert_to_utf16_column(line, column),
        }
    }

    /// Get the entirety of all lines that contain the provided span.
    pub fn lines(&self, span: Span) -> impl Iterator<Item = Span> + '_ {
        let mut start_line = partition_point(&self.newlines, |&it| it <= span.start);
        let mut end_line = partition_point(&self.newlines, |&it| it < span.end);

        // NOTE: If the start line is not the first line, subtract 1 to get the start of the current line.
        if start_line != 0 {
            start_line -= 1;
        }

        // NOTE: If the end of the line is not the last line, add 1 to get the end of the current line.
        // If this is the last, set the end of the range to the utf8 byte length of the source.
        let end_range = if end_line < self.newlines.len() {
            let result = self.newlines[end_line];
            end_line += 1;
            result
        } else {
            self.utf8_len
        };

        let all_lines = self.newlines[start_line..end_line].iter().copied().chain(iter::once(end_range));

        all_lines
            .clone()
            .zip(all_lines.skip(1))
            .map(|(start, end)| Span::new(start..end))
            .filter(|span| !span.is_empty())
    }

    fn convert_to_utf16_column(&self, line: usize, col: usize) -> usize {
        let mut result: usize = col;

        if let Some(utf16_chars) = self.utf16_lines.get(&line) {
            for character in utf16_chars {
                if character.end() <= col {
                    result -= character.len() - character.len_utf16();
                } else {
                    break;
                }
            }
        }

        result
    }
}

pub fn partition_point<T, P>(slice: &[T], mut predicate: P) -> usize
where
    P: FnMut(&T) -> bool,
{
    use std::cmp::Ordering::{Greater, Less};

    slice.binary_search_by(|x| if predicate(x) { Less } else { Greater }).unwrap_or_else(|i| i)
}

#[cfg(test)]
mod test {
    use crate::{source::LineIndex, span::Span};

    #[test]
    fn line_column_of_ascii_char() {
        let index = LineIndex::new("abc\ndef");
        let position = index.position_of(4);

        assert_eq!(position.line, 1);
        assert_eq!(position.column_utf16, 0);
    }

    #[test]
    fn line_column_of_utf16_char() {
        let index = LineIndex::new("abc\ndef\n♥♥♥");
        let position = index.position_of(11);

        assert_eq!(position.line, 2);
        assert_eq!(position.column_utf16, 1);
    }

    #[test]
    fn lines_with_ascii_chars() {
        let input = "abc\ndef\nghi\njkl";
        let index = LineIndex::new(input);
        let mut lines = index.lines(Span::new(3..13));

        check_line("abc\n", input, lines.next());
        check_line("def\n", input, lines.next());
        check_line("ghi\n", input, lines.next());
        check_line("jkl", input, lines.next());
        assert!(lines.next().is_none());
    }

    #[test]
    fn lines_with_utf16_chars() {
        let input = "abc\ndef\n♥♥♥\njkl";
        let index = LineIndex::new(input);
        let mut lines = index.lines(Span::new(3..22));

        check_line("abc\n", input, lines.next());
        check_line("def\n", input, lines.next());
        check_line("♥♥♥\n", input, lines.next());
        check_line("jkl", input, lines.next());
        assert!(lines.next().is_none());
    }

    #[test]
    fn sub_lines_with_ascii_chars() {
        let input = "abc\ndef\nghi\njkl";
        let index = LineIndex::new(input);
        let mut lines = index.lines(Span::new(4..9));

        check_line("def\n", input, lines.next());
        check_line("ghi\n", input, lines.next());
        assert!(lines.next().is_none());
    }

    #[test]
    fn sub_lines_with_utf16_chars() {
        let input = "abc\ndef\n♥♥♥\njkl";
        let index = LineIndex::new(input);
        let mut lines = index.lines(Span::new(4..12));

        check_line("def\n", input, lines.next());
        check_line("♥♥♥\n", input, lines.next());
        assert!(lines.next().is_none());
    }

    fn check_line(expected: &str, input: &str, span: Option<Span>) {
        if let Some(span) = span {
            assert_eq!(&input[span.range()], expected)
        } else {
            panic!("No input span provided.")
        }
    }
}
