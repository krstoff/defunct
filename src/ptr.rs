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