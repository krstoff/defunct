use crate::values::Val;
use crate::values::nil;

const SMALL_MAP_MAX: usize = 32;

pub enum Map {
    SmallMap([Val; SMALL_MAP_MAX * 2]),
    HashMap(std::collections::HashMap<Val, Val>)
}

impl Map {
    pub fn new() -> Map {
        Map::SmallMap([nil(); SMALL_MAP_MAX * 2])
    }
}