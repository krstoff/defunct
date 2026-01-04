use crate::alloc::Heap;

pub struct Symbol {
    name: UnsafeStr,
}

#[derive(Copy, Clone, Eq)]
struct UnsafeStr(pub *const str);
impl UnsafeStr {
    pub unsafe fn new(s: *const str) -> UnsafeStr {
        assert!(s as *const () as usize != 0);
        UnsafeStr(s)
    }
    pub fn to_str(&self) -> &str {
        unsafe { &(*self.0) as &str }
    }
}

impl std::cmp::PartialEq for UnsafeStr {
   fn eq(&self, other: &Self) -> bool {
    unsafe {
        *self.0 == *other.0
    }
   }
}

impl std::hash::Hash for UnsafeStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe {
            (*self.0).hash(state);
        }
    }
}

pub struct SymbolTable<'a> {
    table: std::collections::HashMap<UnsafeStr, *mut Symbol>,
    heap: &'a mut Heap,    
}

impl <'a> SymbolTable<'a> {
    pub fn new(heap: &'a mut Heap) -> SymbolTable<'a> {
        SymbolTable {
            table: std::collections::HashMap::new(),
            heap
        }
    }
    pub fn intern(&mut self, name: &str) -> *mut Symbol {
        let name = unsafe { UnsafeStr::new(name as *const str) };
        if !self.table.contains_key(&name) {
            let mut sym = self.heap.alloc(size_of::<Symbol>()) as *mut Symbol;
            unsafe { *sym = Symbol { name } };
            self.table.insert(name, sym);
        }
        *self.table.get(&name).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn symbols_compare_by_value_not_identity() {
        let mut heap = Heap::new();
        let first = "HELLO";
        let second = String::from("hello").to_uppercase();
        let third = String::from("Nope");
        let mut table = SymbolTable::new(&mut heap);


        let first_symbol = table.intern(first);
        let second_symbol = table.intern(&second);
        let third_symbol = table.intern(&third);
        assert_eq!(first_symbol, second_symbol);
        assert_ne!(first_symbol, third_symbol);
    }
}