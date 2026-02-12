use allocator_api2::vec::Vec;
use super::Val;
use crate::alloc::Heap;

pub struct Vector(Vec<Val, Heap>);

impl Vector {
    pub fn new() -> Vector {
        Vector(Vec::new_in(Heap))
    }

    pub fn get(&self, i: usize) -> Option<Val> {
        (self.0).get(i).map(|item| *item)
    }

    pub fn set(&mut self, i: usize, v: Val) -> Option<()> {
        (self.0).get_mut(i).map(|slot| *slot = v)
    }

    pub fn push(&mut self, v: Val) {
        (self.0).push(v);
    }

    pub fn pop(&mut self) -> Option<Val> {
        (self.0).pop()
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_vector_usage() {
        let mut v = Vector::new();
        for i in 0..100 {
            v.push(Val::from_int(i));
        }
        assert_eq!(Val::from_int(50), v.get(50).unwrap());
        assert_eq!(v.len(), Val::from_int(100));
        v.set(50, Val::from_int(951));
        assert_eq!(v.get(50).unwrap(), Val::from_int(951));
        for i in 0..99 {
            v.pop();
        }
        assert_eq!(v.len(), Val::from_int(1));
        assert_eq!(v.pop().unwrap(), Val::from_int(0));
    }

    #[test]
    #[should_panic]
    fn no_setting_past_last() {
        let mut v = Vector::new();
        for i in 0..100 {
            v.push(Val::from_int(i));
        }
        v.set(100, Val::from_int(1000)).unwrap();
    }

    #[test]
    #[should_panic]
    fn no_getting_past_last() {
        let mut v = Vector::new();
        for i in 0..100 {
            v.push(Val::from_int(i));
        }
        v.get(100).unwrap();
    }

    #[test]
    #[should_panic]
    fn no_popping_after_empty() {
        let mut v = Vector::new();
        for i in 0..100 {
            v.push(Val::from_int(i));
        }

        for i in 0..100 {
            v.pop();
        }

        v.pop().unwrap();
    }
}