use super::tag::Tags;

pub type ContextMsgId = &'static str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ContextKind {
    Note,
    Help,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub msg_id: ContextMsgId,
    pub tags: Tags,
    pub kind: ContextKind,
}

impl Context {
    pub const fn new(msg_id: ContextMsgId, kind: ContextKind) -> Self {
        Self {
            msg_id,
            tags: Tags::new(),
            kind,
        }
    }

    pub fn with_tags(mut self, tags: Tags) -> Self {
        self.tags = tags;
        self
    }
}

pub static INVALID_INDEX_TYPES: ContextMsgId = "index-operator-types";
pub static MODULE_LOAD_ERROR: ContextMsgId = "module-load-error";
pub static EXPORT_ONLY_ALLOWED_IN_MODULES: ContextMsgId = "export-only-allowed-in-modules";
pub static EXPORT_ONLY_ALLOWED_IN_TOP_LEVEL_SCOPE: ContextMsgId = "export-only-allowed-in-top-level-scope";
pub static IMPORT_REQUIRES_ITEMS_TO_BE_IMPORTED: ContextMsgId = "import-requires-items-to-be-imported";
pub static IMPORT_REQUIRES_ITEMS_TO_BE_IMPORTED_HELP: ContextMsgId = "import-requires-items-to-be-imported-help";
pub static MISMATCHED_TYPE_ASSERTIONS: ContextMsgId = "mismatched-type-assertions";
