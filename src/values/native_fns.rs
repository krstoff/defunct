#[derive(Copy, Clone)]
pub struct NativeFn(fn() -> ());

impl NativeFn {
    pub fn addr(&self) -> usize {
        (self.0 as *const u8).addr()
    }
}