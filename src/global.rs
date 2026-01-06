use allocator_api2::alloc::Allocator;

use crate::values::{Symbol, SymbolTable};

pub struct Global {
    pub st: SymbolTable,
}

impl Global {
    pub fn new() -> Global {
        let st = SymbolTable::new();
        Global { st }
    }
    
    pub fn intern(&mut self, name: &str) -> *mut Symbol {
        self.st.intern(name)
    }
}