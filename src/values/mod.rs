/// Virtual machine values representing the complete set of types in defunct.
/// Uses Webkit's NaN-boxing scheme to encode pointers and 32 bit integers into
/// a double-precision float. 
/// Pointers themselves are also tagged with type information in the low-bits.

mod closures;
mod symbols;
mod maps;
mod boolean;

pub use closures::Closure;
pub use symbols::Symbol;
pub use symbols::SymbolTable;
pub use boolean::nil;
pub use boolean::t;

use crate::values;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Tag {
    Symbol = 0,
    Function = 1,
    Cons = 2,
    Array = 3,
    Map = 4,
    Object = 5,
    Error = 6,
    Other = 7,
}

const LOWTAG_MASK: u64 = 0b111;

pub struct Ptr(u64);

impl Ptr {
    pub fn test_ptr() -> Ptr {
        Ptr(0xAAAABADD8)
    }

    pub unsafe fn new(tag: Tag, raw_ptr: u64) -> Ptr {
        fn is_word_aligned(bits: u64) -> bool {
            bits & 0b111 == 0
        }
        assert!(is_word_aligned(raw_ptr));
        Ptr(raw_ptr | (tag as u8) as u64)
    }

    pub fn to_bits(&self) -> u64 {
        let Ptr(bits) = *self;
        bits
    }

    pub fn to_raw(&self) -> (Tag, u64) {
        use Tag::*;
        let Ptr(bits) = *self;
        let tag = match (bits & LOWTAG_MASK) as u8 {
            0 => Symbol,
            1 => Function,
            2 => Cons,
            3 => Array,
            4 => Map,
            5 => Object,
            6 => Error,
            7 => Other,
            _ => unreachable!(),
        };
        let ptr = bits & !LOWTAG_MASK;
        (tag, ptr)
    }
}

const HIGHTAG_MASK: u64 = 0xFFFF_0000_0000_0000;

#[derive(Copy, Clone, PartialEq)]
pub struct Val(u64);

impl Val {
    pub fn from_num(num: f64) -> Val {
        let rotated = num.to_bits().wrapping_add(1 << 48);
        Val(rotated)
    }

    pub fn is_num(&self) -> bool {
        let Val(bits) = *self;
        bits & HIGHTAG_MASK != HIGHTAG_MASK 
            && bits & HIGHTAG_MASK != 0
    }

    pub fn get_num(&self) -> Option<f64> {
        let Val(bits) = *self;
        if self.is_num() {
            let num = f64::from_bits(bits.wrapping_sub(1 << 48));
            Some(num)
        }
        else {
            None
        }
    }

    pub fn from_int(int: u32) -> Val {
        let extended = int as u64;
        Val(extended | HIGHTAG_MASK)
    }

    pub fn is_int(&self) -> bool {
        let Val(bits) = *self;
        bits & HIGHTAG_MASK == HIGHTAG_MASK
    }

    pub fn get_int(&self) -> Option<u32> {
        let Val(bits) = *self;
        if self.is_int() {
            Some(bits as u32)
        }
        else {
            None
        }
    }

    pub fn from_ptr(tag: Tag, ptr: *const u8) -> Val {
        fn is_word_aligned(bits: usize) -> bool {
            bits & 0b111 == 0
        }
        assert!(is_word_aligned(ptr as usize));
        Val(ptr as u64 | (tag as u8) as u64)
    }

    pub fn is_ptr(&self) -> bool {
        let Val(bits) = *self;
        bits & HIGHTAG_MASK == 0
    }

    pub fn get_ptr(&self) -> Option<Ptr> {
        let Val(bits) = *self;
        if self.is_ptr() {
            Some(unsafe { std::mem::transmute(bits) })
        } else {
            None
        }
    }

    pub fn get(&self) -> Cases {
        if self.is_int() {
            return Cases::Int(self.get_int().unwrap())
        }

        if self.is_num() {
            return Cases::Num(self.get_num().unwrap())
        }
        
        let (tag, ptr) = self.get_ptr().unwrap().to_raw();
        match tag {
            Tag::Function => {
                Cases::Function(ptr as *const _)
            }
            _ => unimplemented!()
        }
    }
}

pub enum Cases {
    Int(u32),
    Num(f64),
    Symbol(*const Symbol),
    Function(*const Closure),
    Cons(),
    Array(),
    Map(),
    Object(),
    Error(),
    Other(),
}

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tag::*;
        if self.is_int() {
            write!(f, "{}", self.get_int().unwrap())
        } else if self.is_ptr() {
            let (t, p) = self.get_ptr().unwrap().to_raw();
            let tag_str = match t {
                Symbol => {
                    if p as usize == 0 {
                        write!(f, ":nil");
                    }
                    else if p as usize == 1 << 4 {
                        write!(f, ":t");
                    }
                    else {
                        let sym = unsafe { &*(p as  *const crate::values::Symbol) };
                        write!(f, ":{}", sym.to_str());
                    }
                    return Ok(())
                }
                Function => "fun",
                Cons => "cons",
                Array => "vec",
                Map => "map",
                Object => "obj",
                Error => "err",
                Other => "?"
            };
            write!(f, "<{} {:x}>", tag_str, p)
        } else {
            write!(f, "{}f", self.get_num().unwrap())
        }
    }
}