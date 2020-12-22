use crate::{bytecode::Bytecode, source::Source, span::Span};

#[derive(Debug, Clone)]
pub struct ErrorTrace {
    pub source: Source,
    pub span: Span,
}

impl ErrorTrace {
    pub fn from_bytecode(bytecode: &Bytecode, offset: u64) -> Self {
        let span = bytecode.source_map()[&offset];
        let source = bytecode.source().clone();

        Self { source, span }
    }
}
