use crate::values::{Val, Ptr, Tag};

pub fn nil() -> Val { 
    Val::from_ptr(Tag::Symbol, 0 as *const u8)
}

pub fn t() -> Val { 
    Val::from_ptr(Tag::Symbol, (1 << 4) as *const u8)
}