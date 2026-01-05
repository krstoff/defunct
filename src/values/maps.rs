use crate::alloc::Heap;
use crate::values::Val;
use crate::values::nil;

use hashbrown::HashMap;

const SMALL_MAP_MAX: usize = 31;

pub enum Map {
    SmallMap { len: usize, items: [(Val, Val); SMALL_MAP_MAX] },
    HashMap (HashMap<Val, Val>)
}

impl Map {
    pub fn new() -> Map {
        Map::SmallMap {
            len: 0,
            items: [(nil(), nil()); SMALL_MAP_MAX]
        }
    }

    pub fn insert(&mut self, key: Val, value: Val) {
        match self {
            &mut Map::SmallMap { ref mut len, ref mut items } if *len == SMALL_MAP_MAX => {
                let mut hashmap = HashMap::new();
                for i in 0..SMALL_MAP_MAX {
                    let (k, v) = items[i];
                    hashmap.insert(k, v);
                }
                hashmap.insert(key, value);
                *self = Map::HashMap(hashmap)
            }
            &mut Map::SmallMap { ref mut len, ref mut items } => {
                if let Some(i) = items.iter().take(*len).position(|(k, v)| key == *k) {
                    items[i]= (key, value); 
                } else {
                    items[*len] = (key, value);
                    *len = *len + 1;
                }
            }
            &mut Map::HashMap(ref mut hashmap) => {
                hashmap.insert(key, value);
            }
        }
    }

    pub fn get(&self, key: Val) -> Val {
        match self {
            &Map::SmallMap { ref len, ref items } => {
                if let Some(i) = items.iter().take(*len).position(|(k, v)| key == *k) {
                    items[i].1
                } else {
                    nil()
                }
            }
            &Map::HashMap(ref hashmap) => {
                *hashmap.get(&key).unwrap_or(&nil())
            }           
        }
    }

    pub fn remove(&mut self, key: Val) -> Val {
        match self {
            &mut Map::SmallMap { ref mut len, ref mut items } => {
                if let Some(i) = items.iter().take(*len).position(|(k, v)| key == *k) {
                    let deleted = items[i].1;
                    if i == *len - 1 {
                        items[i] = (nil(), nil());
                    } else {
                        items[i] = items[*len - 1];
                        items[*len - 1] = (nil(), nil());
                    }
                    *len = *len - 1;
                    deleted
                } else {
                    nil()
                }
            }
            &mut Map::HashMap(ref mut hashmap) => {
                hashmap.remove(&key).unwrap_or(nil())
            }           
        }
    }

    pub fn len(&self) -> usize {
        match self {
            &Map::SmallMap { ref len, ..} => { *len }
            &Map::HashMap(ref hashmap) => { hashmap.len() }
        }
    }

    pub fn clear(&mut self) {
        *self = Map::new();
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
        assert_eq!(map.get(int(2)), nil());
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
        assert_eq!(map.get(int(1000000)), nil());
    }
}