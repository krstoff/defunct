use crate::{alloc::Heap, values::{Symbol, SymbolTable}};

use crate::HEAP;

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

    pub fn alloc(&self, size: usize) -> *mut u8 {
        HEAP.with(|heap| heap.alloc(size))
    }
}