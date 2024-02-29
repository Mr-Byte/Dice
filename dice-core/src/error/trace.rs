use crate::source::Source;
use crate::span::Span;

#[derive(Debug, Clone)]
pub struct ErrorTrace {
    pub source: Source,
    pub span: Span,
}

impl ErrorTrace {
    // pub fn from_bytecode(bytecode: &Bytecode, offset: u64) -> Self {
    //     todo!();

    //     // let span = bytecode.source_map()[&offset];
    //     // let source = bytecode.source().clone();

    //     // Self { source, span }
    // }
}
