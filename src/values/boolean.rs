use crate::values::{Val, Tag};

/// The booleans :t and :nil are treated as symbol constants.

pub fn nil() -> Val { 
    Val::from_ptr(Tag::Symbol, 0 as *mut u8)
}

pub fn t() -> Val { 
    Val::from_ptr(Tag::Symbol, (1 << 4) as *mut u8)
}