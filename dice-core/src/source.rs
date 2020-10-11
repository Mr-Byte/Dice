use std::rc::Rc;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SourceKind {
    Module,
    Script,
}

#[derive(Clone)]
pub struct Source {
    path: Option<Rc<str>>,
    source: Rc<str>,
    kind: SourceKind,
}

impl Source {
    pub fn new(source: String, kind: SourceKind) -> Self {
        Self {
            path: None,
            source: source.into(),
            kind,
        }
    }

    pub fn with_path(source: String, path: String, kind: SourceKind) -> Self {
        Self {
            path: Some(path.into()),
            source: source.into(),
            kind,
        }
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn source(&self) -> &str {
        &*self.source
    }

    pub fn kind(&self) -> SourceKind {
        self.kind
    }
}
