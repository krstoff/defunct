use crate::{alloc::Heap, values::{Symbol, SymbolTable}};

pub struct Global {
    pub heap: Heap,
    pub st: SymbolTable,
}

impl Global {
    pub fn new() -> Global {
        let heap = Heap::new();
        let st = SymbolTable::new();
        Global { heap, st }
    }
    
    pub fn intern(&mut self, name: &str) -> *mut Symbol {
        self.st.intern(name, &mut self.heap)
    }

    pub fn alloc(&mut self, size: usize) -> *mut u8 {
        self.heap.alloc(size)
    }
}