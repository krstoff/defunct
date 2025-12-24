use crate::values::{Val, Ptr, Tag};

pub fn nil() -> Val { 
    let ptr = unsafe { Ptr::new(Tag::Symbol, 0) };
    Val::from_ptr(ptr)
}

pub fn t() -> Val { 
    let ptr = unsafe { Ptr::new(Tag::Symbol, 1 << 4) };
    Val::from_ptr(ptr)
}