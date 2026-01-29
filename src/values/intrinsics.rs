#[derive(Copy, Clone)]
pub struct Intrinsic(fn() -> ());

impl Intrinsic {
    pub fn addr(&self) -> usize {
        (self.0 as *const u8).addr()
    }
}