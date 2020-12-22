use super::tag::Tags;

pub type ContextMsgId = &'static str;

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub msg_id: ContextMsgId,
    pub tags: Tags,
}

impl ErrorContext {
    pub const fn new(msg_id: ContextMsgId) -> Self {
        Self {
            msg_id,
            tags: Tags::new(),
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
