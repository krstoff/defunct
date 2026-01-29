use crate::{bytecode::ByteCode, values::Tag};
use super::Val;

pub struct Closure {
    pub env: *const [Val],
    pub code_obj: *const ByteCode
}

impl Closure {
    pub fn new(env: *const [Val], code_obj: *const ByteCode) -> Val {
        use crate::alloc::Heap;
        let mut closure = Heap::new::<Closure>();
        unsafe { std::ptr::write(closure, Closure { env, code_obj }) };
        Val::from_ptr(Tag::Function, closure as *mut u8)
    }
}