#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SourceKind {
    Module,
    Script,
}

pub struct Source {
    path: Option<String>,
    source: String,
    kind: SourceKind,
}

impl Source {
    pub fn new(source: String, kind: SourceKind) -> Self {
        Self {
            path: None,
            source,
            kind,
        }
    }

    pub fn with_path(source: String, kind: SourceKind, path: String) -> Self {
        Self {
            path: Some(path),
            source,
            kind,
        }
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    pub fn kind(&self) -> SourceKind {
        self.kind
    }
}
