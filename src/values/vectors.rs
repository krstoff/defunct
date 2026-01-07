use allocator_api2::vec::Vec;
use super::Val;
use crate::alloc::Heap;

pub struct Vector(Vec<Val, Heap>);

impl Vector {
    pub fn new() -> Vector {
        Vector(Vec::new_in(Heap))
    }

    pub fn get(&self, i: usize) -> Val {
        (self.0)[i]
    }

    pub fn set(&mut self, i: usize, v: Val) {
        (self.0)[i] = v;
    }

    pub fn push(&mut self, v: Val) {
        (self.0).push(v);
    }

    pub fn pop(&mut self) -> Val {
        (self.0).pop().unwrap()
    }

    pub fn len(&self) -> Val {
        assert!((self.0).len() < i32::MAX as usize);
        Val::from_int((self.0).len() as i32)
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item=Val> + '_> {
        Box::new((self.0).iter().map(|v| *v))
    }
}

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut count = 0;
        for v in self.iter() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", v)?;
            count += 1;
        }
        write!(f, "]")?;
        Ok(())
    }
}