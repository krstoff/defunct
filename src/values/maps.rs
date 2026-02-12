use crate::alloc::Heap;
use crate::values::*;

use hashbrown::DefaultHashBuilder;
use hashbrown::HashMap;

const SMALL_MAP_MAX: usize = 31;

pub enum Map {
    SmallMap { len: usize, items: [(Val, Val); SMALL_MAP_MAX] },
    HashMap (HashMap<Val, Val, DefaultHashBuilder, Heap>)
}

impl Map {
    pub fn new() -> Map {
        Map::SmallMap {
            len: 0,
            items: [(Symbol::nil(), Symbol::nil()); SMALL_MAP_MAX]
        }
    }

    pub fn insert(&mut self, key: Val, value: Val) -> Val {
        match self {
            Map::SmallMap { len, items } if *len == SMALL_MAP_MAX => {
                let mut hashmap = HashMap::new_in(Heap);
                for i in 0..SMALL_MAP_MAX {
                    let (k, v) = items[i];
                    hashmap.insert(k, v);
                }
                let old_value = hashmap.insert(key, value);
                *self = Map::HashMap(hashmap);
                old_value.unwrap_or(Val::nil())
            }
            Map::SmallMap { len, items } => {
                if let Some(i) = items.iter().take(*len).position(|(k, v)| key == *k) {
                    let old_value = items[i];
                    items[i] = (key, value);
                    old_value.1
                } else {
                    items[*len] = (key, value);
                    *len = *len + 1;
                    Val::nil()
                }
            }
            Map::HashMap(hashmap) => {
                hashmap.insert(key, value).unwrap_or(Val::nil())
            }
        }
    }

    pub fn get(&self, key: Val) -> Val {
        match self {
            Map::SmallMap { len, items } => {
                if let Some(i) = items.iter().take(*len).position(|(k, v)| key == *k) {
                    items[i].1
                } else {
                    Symbol::nil()
                }
            }
            Map::HashMap(hashmap) => {
                *hashmap.get(&key).unwrap_or(&Symbol::nil())
            }           
        }
    }

    pub fn remove(&mut self, key: Val) -> Val {
        match self {
            Map::SmallMap { len, items } => {
                if let Some(i) = items.iter().take(*len).position(|(k, v)| key == *k) {
                    let deleted = items[i].1;
                    if i == *len - 1 {
                        items[i] = (Symbol::nil(), Symbol::nil());
                    } else {
                        items[i] = items[*len - 1];
                        items[*len - 1] = (Symbol::nil(), Symbol::nil());
                    }
                    *len = *len - 1;
                    deleted
                } else {
                    Symbol::nil()
                }
            }
            Map::HashMap(hashmap) => {
                hashmap.remove(&key).unwrap_or(Symbol::nil())
            }           
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Map::SmallMap { len, ..} => { *len }
            Map::HashMap(hashmap) => { hashmap.len() }
        }
    }

    pub fn clear(&mut self) {
        *self = Map::new();
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item=(Val, Val)> + '_> {
        match self {
            Map::SmallMap { len, items } => {
                Box::new(items.iter().take(*len).map(|(k, v)| (*k, *v)))
            }
            Map::HashMap(hashmap) => {
                Box::new(hashmap.iter().take(hashmap.len()).map(|(k, v)| (*k, *v)))
            }             
        }
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut count = 0;
        for (k, v) in self.iter() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?} {:?}", k, v)?;
            count += 1;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn int(i: i32) -> Val {
        Val::from_int(i)
    }
    #[test]
    fn insert_increases_size() {
        let mut map = Map::new();
        for i in 0..(SMALL_MAP_MAX + 2) as i32 {
            assert!(map.len() == i as usize);
            map.insert(int(i), int(i));
        }
    }

    #[test]
    fn double_insert_overwrites() {
        let mut map = Map::new();
        map.insert(int(2), int(4));
        map.insert(int(2), int(25));
        assert_eq!(map.get(int(2)), int(25));
    }

    #[test]
    fn get() {
        let mut map = Map::new();
        map.insert(int(2), int(4));
        assert_eq!(map.get(int(2)), int(4));
    }

    #[test]
    fn remove() {
        let mut map = Map::new();
        map.insert(int(2), int(4));
        map.remove(int(2));
        assert_eq!(map.get(int(2)), Symbol::nil());
    }

    #[test]
    fn remove_many() {
        let mut map = Map::new();
        for i in 0..5 {
            map.insert(int(i), int(i));
        }
        
        for i in (0..5).rev() {
            assert_eq!(map.remove(int(i)), int(i));
            assert!(map.len() == i as usize);
        }

        for i in 0..5 {
            map.insert(int(i), int(i));
        }

        for i in 0..5 {
            map.remove(int(i));
            assert!(map.len() == 4 - i as usize);
        }
    }

    #[test]
    fn map_remains_small() {
        let mut map = Map::new();
        for i in 0..SMALL_MAP_MAX as i32 {
            map.insert(int(i), int(i));
        }
        for i in 0..SMALL_MAP_MAX as i32 {
            map.remove(int(i));
        }
        assert!(map.len() == 0);
        if let &Map::HashMap(..) = &map {
            panic!("Should not have switched representations!");
        }
    }

    #[test]
    fn nil_on_not_found() {
        let mut map = Map::new();
        for i in 0..SMALL_MAP_MAX as i32 {
            map.insert(int(i), int(i));
        }
        assert_eq!(map.get(int(1000000)), Symbol::nil());
    }
}