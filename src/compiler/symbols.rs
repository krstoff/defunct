use std::{collections::HashMap};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(usize);

pub struct SymbolTable<'a> {
    table: HashMap<&'a str, Symbol>,
    symbols: Vec<&'a str> 
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        SymbolTable { table: HashMap::new(), symbols: Vec::new() }
    }

    pub fn intern(&mut self, name: &'a str) -> Symbol {
        if !self.table.contains_key(name) {
            self.table.insert(name, Symbol(self.symbols.len()));
            self.symbols.push(name);
        }
        *self.table.get(name).unwrap()
    }

    pub fn get_name(&self, Symbol(index): Symbol) -> &'a str {
        self.symbols[index]
    }
}