use std::ptr;
use super::{nil, t};

use crate::alloc::Heap;

pub struct Symbol {
    name: UnsafeStr,
}

impl Symbol {
    pub fn to_str(&self) -> &str {
        let v = super::Val::from_ptr(super::Tag::Symbol, unsafe { std::mem::transmute(self) });
        if v == nil() {
            return "nil"
        } else if v == t() {
            return "t"
        }
        self.name.to_str()
    }
}

#[derive(Copy, Clone, Eq)]
struct UnsafeStr(pub *const str);
impl UnsafeStr {
    pub unsafe fn new(s: *const str) -> UnsafeStr {
        assert!(s as *const () as usize != 0);
        UnsafeStr(s)
    }
    pub fn to_str(&self) -> &str {
        unsafe { &(*self.0) }
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

pub struct SymbolTable {
    table: std::collections::HashMap<UnsafeStr, *mut Symbol>,
}

impl  SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: std::collections::HashMap::new(),
        }
    }
    pub fn intern(&mut self, name: &str, heap: &mut Heap) -> *mut Symbol {
        let name = unsafe { UnsafeStr::new(name as *const str) };
        if !self.table.contains_key(&name) {
            unsafe {
                let mut size = (&*name.0).len();
                let mut name_copy = heap.alloc(size);
                ptr::copy_nonoverlapping((*name.0).as_ptr(), name_copy, size);
                let mut sym = heap.alloc(size_of::<Symbol>()) as *mut Symbol;
                *sym = Symbol { name };
                self.table.insert(name, sym);
            }
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
        let mut table = SymbolTable::new();


        let first_symbol = table.intern(first, &mut heap);
        let second_symbol = table.intern(&second, &mut heap);
        let third_symbol = table.intern(&third, &mut heap);
        assert_eq!(first_symbol, second_symbol);
        assert_ne!(first_symbol, third_symbol);
    }
}