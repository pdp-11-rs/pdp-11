use std::fmt;

pub type Result<T> = std::result::Result<T, AsmError>;

#[derive(Debug, Clone)]
pub struct AsmError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl AsmError {
    pub fn new(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            column,
            message: message.into(),
        }
    }
}

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for AsmError {}
