use crate::ptr::Ptr;

const HIGHTAG_MASK: u64 = 0xFFFF_0000_0000_0000;

pub struct Val(u64);

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::ptr::Tag::*;
        if self.is_int() {
            write!(f, "{}", self.get_int().unwrap())
        } else if self.is_ptr() {
            let (t, p) = self.get_ptr().unwrap().to_raw();
            let tag_str = match t {
                Symbol => "sym",
                Function => "fun",
                Cons => "cons",
                Array => "vec",
                Map => "map",
                Class => "class",
                Error => "err",
                Other => "?"
            };
            write!(f, "<{} {:x}>", tag_str, p)
        } else {
            write!(f, "{}f", self.get_num().unwrap())
        }
    }
}

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

    pub fn from_ptr(ptr: Ptr) -> Val {
        Val(ptr.to_bits())
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
}