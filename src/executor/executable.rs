use std::fmt::{Debug, Formatter};

use crate::fenced_attributes::CodeChunk;

/// Executable position with start and end line numbers
#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
pub struct ExecutablePosition {
    pub start: usize,
    pub end: usize,
}

impl ExecutablePosition {
    /// Start setter
    pub fn start(start: usize) -> Self {
        Self { start, end: 0 }
    }

    /// Create new position
    #[allow(dead_code)]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    /// End setter
    pub fn end(self, end: usize) -> Self {
        Self { end, ..self }
    }
}

/// Executable metadata with the information on the code chunk
#[derive(Default, PartialEq, Eq, Clone)]
pub struct Executable {
    /// Position of the executable
    pub position: ExecutablePosition,
    /// Extracted code chunks
    pub code_chunk: Option<CodeChunk>,
    /// The actual code to be executed
    pub code: String,
}

impl Debug for Executable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Executable")
            .field("position", &self.position)
            .field("code_chunk", &self.code_chunk)
            .finish()
    }
}

impl Executable {
    pub fn new(position: ExecutablePosition, code_chunk: Option<CodeChunk>, code: String) -> Self {
        Self {
            position,
            code_chunk,
            code,
        }
    }
}
