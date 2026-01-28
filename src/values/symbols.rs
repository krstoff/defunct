use crate::values::{LOWTAG_BITS, Tag, Val};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Symbol(*mut Cell);

const NIL: usize = 0 << LOWTAG_BITS;
const T: usize = 1 << LOWTAG_BITS;

impl Symbol {
    pub fn name(&self) -> &str {
        let Symbol(ptr) = *self;
        if ptr.addr() == NIL {
            "nil"
        } else if ptr.addr() == T {
            "t"
        } else {
            unsafe { &*(*self.0)._name }
        }
    }

    pub fn val(&self) -> Option<Val> {
        let Symbol(ptr) = *self;
        if ptr.addr() == NIL {
            Some(Self::nil())
        } else if ptr.addr() == T {
            Some(Self::t())
        } else {
            unsafe {
                (*self.0)._value
            }
        }
    }

    pub fn set(&mut self, value: Val) {
        let Symbol(ptr) = *self;
        if ptr.addr() == NIL {
            panic!("Cannot assign a value to :nil.");
        } else if ptr.addr() == T {
            panic!("Cannot assign a value to :t");
        } else {
            unsafe {
                (*self.0)._value = Some(value)
            }
        }
    }

    pub fn t() -> Val {
        Val::from_ptr(Tag::Symbol, T as *mut _)
    }

    pub fn nil() -> Val {
        Val::from_ptr(Tag::Symbol, NIL as *mut _)
    }

    pub fn addr(&self) -> usize {
        self.0.addr()
    }

    pub fn as_val(&self) -> Val {
        Val::from_ptr(Tag::Symbol, self.0 as *mut u8)
    }
}


impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}", self.name())
    }
}

struct Cell {
    _name: *const str,
    _value: Option<Val>
}

/// Wraps a raw str pointer and implements hash, eq by value. Used internally by SymbolTable.
#[derive(Copy, Clone, Eq)]
struct UnsafeStr(*const str);
impl UnsafeStr {
    pub unsafe fn from_raw(s: *const str) -> UnsafeStr {
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
    table: std::collections::HashMap<UnsafeStr, Symbol>,
}

impl  SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: std::collections::HashMap::new(),
        }
    }

    /// Takes a &str and checks if it names an existing symbol.
    /// If not, the string is interned and a fresh symbol is allocated.
    /// In both cases, the symbol is returned.
    pub fn intern(&mut self, name: &str) -> Symbol {
        use crate::alloc::Heap;
        let name = unsafe { UnsafeStr::from_raw(name as *const str) };
        if !self.table.contains_key(&name) {
            unsafe {
                let _name = &*name.0;
                let mut size = _name.len();
                let mut name_copy_bytes = Heap::alloc(size);
                std::ptr::copy_nonoverlapping(_name.as_ptr(), name_copy_bytes, size);
                let name_copy = std::str::from_utf8_unchecked(std::slice::from_raw_parts(name_copy_bytes as *const _, size));

                let mut cell = Heap::alloc(size_of::<Cell>()) as *mut Cell;
                std::ptr::write(cell, Cell { _name: name_copy, _value: None });

                self.table.insert(name, Symbol(cell));
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
        let first = "HELLO";
        let second = String::from("hello").to_uppercase();
        let third = String::from("Nope");
        let mut table = SymbolTable::new();


        let first_symbol = table.intern(first);
        let second_symbol = table.intern(&second);
        let third_symbol = table.intern(&third);
        assert_eq!(first_symbol, first_symbol);
        assert_eq!(first_symbol, second_symbol);
        assert_ne!(first_symbol, third_symbol);
    }
}