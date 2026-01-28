/// Virtual machine values representing the complete set of types in defunct.
/// Uses Webkit's NaN-boxing scheme to encode pointers and 32 bit signed integers into
/// a double-precision float. 
/// Pointers themselves are also tagged with type information in the low-bits.

mod closures;
mod symbols;
mod maps;
mod vectors;

pub use closures::Closure;
pub use symbols::Symbol;
pub use symbols::SymbolTable;
pub use maps::Map;
pub use vectors::Vector;

use crate::values;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Tag {
    Symbol = 0,
    Function = 1,
    Cons = 2,
    Vector = 3,
    Map = 4,
    Object = 5,
    Error = 6,
    Other = 7,
}

fn byte_to_tag(byte: u8) -> Tag {
    assert!(byte < 8);
    unsafe { std::mem::transmute(byte) }
}

const LOWTAG_BITS: usize = 3;
const LOWTAG_MASK: usize = 0b111;
const HIGHTAG_MASK: usize = 0xFFFF_0000_0000_0000;

// We do not support 32-bit architectures.
#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone)]
pub struct Val(*mut u8);

impl Val {
    pub fn bits(&self) -> usize {
        self.0.addr()
    }

    pub fn from_num(num: f64) -> Val {
        let rotated = num.to_bits().wrapping_add(1 << 48);
        Val(rotated as *mut u8)
    }

    #[inline(always)]
    pub fn is_num(&self) -> bool {
        let Val(ptr) = *self;
        let bits = ptr.addr();
        bits & HIGHTAG_MASK != HIGHTAG_MASK && bits & HIGHTAG_MASK != 0
    }

    #[inline(always)]
    pub fn get_num(&self) -> Option<f64> {
        let Val(ptr) = *self;
        let bits = ptr.addr() as u64;
        if self.is_num() {
            let num = f64::from_bits(bits.wrapping_sub(1 << 48));
            Some(num)
        }
        else {
            None
        }
    }

    pub fn from_int(int: i32) -> Val {
        let extended = int as usize;
        Val((extended | HIGHTAG_MASK) as *mut u8)
    }

    #[inline(always)]
    pub fn is_int(&self) -> bool {
        let Val(ptr) = *self;
        let bits = ptr.addr();
        bits & HIGHTAG_MASK == HIGHTAG_MASK
    }

    #[inline(always)]
    pub fn get_int(&self) -> Option<i32> {
        let Val(ptr) = *self;
        let bits = ptr.addr();
        if self.is_int() {
            Some((bits & !HIGHTAG_MASK) as i32)
        }
        else {
            None
        }
    }

    pub fn from_ptr(tag: Tag, ptr: *mut u8) -> Val {
        fn is_word_aligned(bits: usize) -> bool {
            bits & LOWTAG_MASK as usize == 0
        }
        assert!(is_word_aligned(ptr as usize));
        Val(ptr.map_addr(|bits| (bits | tag as u8 as usize) ))
    }

    #[inline(always)]
    pub fn is_ptr(&self) -> bool {
        let Val(ptr) = *self;
        let bits = ptr.addr();
        bits & HIGHTAG_MASK == 0
    }

    pub fn get(&self) -> Cases {
        if self.is_int() {
            return Cases::Int(self.get_int().unwrap())
        }

        if self.is_num() {
            return Cases::Num(self.get_num().unwrap())
        }
        
        let tag_bits = (self.0 as usize & LOWTAG_MASK) as u8;
        let ptr = self.0.map_addr(|addr| addr & !LOWTAG_MASK);
        match byte_to_tag(tag_bits) {
            Tag::Function => {
                Cases::Function(ptr as *const _)
            }
            Tag::Symbol => {
                Cases::Symbol(unsafe{ std::mem::transmute(ptr) })
            }
            Tag::Map => {
                Cases::Map(ptr as *mut _)
            }
            Tag::Vector => {
                Cases::Vector(ptr as *mut _)
            }
            _ => unimplemented!()
        }
    }
}

pub enum Cases {
    Int(i32),
    Num(f64),
    Symbol(Symbol),
    Function(*const Closure),
    Cons(),
    Vector(*mut Vector),
    Map(*mut Map),
    Object(),
    Error(),
    Other(),
}

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cases::*;
        match self.get() {
            Int(i) => write!(f, "{}", i),
            Num(n) => write!(f, "{}f", n),
            Symbol(p) => {
                write!(f, ":{}", p.name())
            }
            Function(p) => {
                write!(f, "<fn {:x}>", p.addr())
            }
            Map(p) => {
                let map = unsafe { &*p };
                write!(f, "{:?}", map)
            }
            Vector(p) => {
                let vec = unsafe { &*p };
                write!(f, "{:?}", vec)
            }
            _ => unimplemented!()
        }
    }
}

impl std::cmp::PartialEq for Val {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl std::cmp::Eq for Val {}

impl std::hash::Hash for Val {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        use Cases::*;
        match self.get() {
            Int(i) => {
                state.write_i32(i)
            }
            Num(n) => {
                state.write_u64(n.to_bits())
            }
            Symbol(s) => {
                state.write_usize(s.addr())
            }
            Function(f) => {
                state.write_usize(f.addr())
            }
            _ => unimplemented!()
        }
    }
}