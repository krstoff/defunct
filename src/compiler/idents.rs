use std::{collections::HashMap};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ident(usize);

pub struct IdentTable<'a> {
    table: HashMap<&'a str, Ident>,
    idents: Vec<&'a str> 
}

impl<'a> IdentTable<'a> {
    pub fn new() -> IdentTable<'a> {
        IdentTable { table: HashMap::new(), idents: Vec::new() }
    }

    pub fn intern(&mut self, name: &'a str) -> Ident {
        if !self.table.contains_key(name) {
            self.table.insert(name, Ident(self.idents.len()));
            self.idents.push(name);
        }
        *self.table.get(name).unwrap()
    }

    pub fn get_name(&self, Ident(index): Ident) -> &'a str {
        self.idents[index]
    }
}