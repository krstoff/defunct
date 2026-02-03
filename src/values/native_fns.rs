use crate::global::Global;

use super::*;
pub type ShouldHalt = bool;
#[derive(Copy, Clone)]
pub struct NativeFn(pub fn(&[Val], &mut Global) -> (Val, ShouldHalt));

impl NativeFn {
    pub fn addr(&self) -> usize {
        (self.0 as *const u8).addr()
    }

    pub fn to_val(&self) -> Val {
        Val::from_ptr(Tag::NativeFn, self.0 as *mut _)
    }
}