use std::collections::HashMap;

/// Symbol table for labels and constants
#[derive(Debug, Default)]
pub struct SymbolTable {
    symbols: HashMap<String, i32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, value: i32) {
        self.symbols.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<i32> {
        self.symbols.get(name).copied()
    }

    pub fn symbols(&self) -> &HashMap<String, i32> {
        &self.symbols
    }
}
