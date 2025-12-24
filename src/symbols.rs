use std::collections::{HashMap, hash_map::Entry};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(usize);

pub struct SymbolTable {
    pub counter: usize,
    pub table: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { counter: 0, table: HashMap::new() }
    }

    pub fn intern(&mut self, name: &str) -> Symbol {
        if !self.table.contains_key(name) {
            self.table.insert(name.to_string(), Symbol(self.counter));
            self.counter += 1;
        }
        *self.table.get(name).unwrap()
    }
}