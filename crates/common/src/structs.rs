//! Commonly used structs.

use serde::{Deserialize, Serialize};

/// The kind of output to render.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputKind {
    Markdown,
    JSON,
}

impl Default for OutputKind {
    fn default() -> Self {
        Self::Markdown
    }
}
