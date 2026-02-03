use crate::values::{Symbol, SymbolTable};
use crate::intrinsics;

pub struct Global {
    pub st: SymbolTable,
}

impl Global {
    pub fn new() -> Global {
        let mut st = SymbolTable::new();
        for (name, function) in intrinsics::INTRINSICS {
            let mut sym = st.intern(name);
            sym.set(function.to_val())
        }
        Global { st }
    }
    
    pub fn intern(&mut self, name: &str) -> Symbol {
        self.st.intern(name)
    }
}