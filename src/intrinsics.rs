use crate::{common::*, global::Global};
use crate::values::{Cases, NativeFn, Val};

pub const INTRINSICS: &[(&str, NativeFn)] = &[
    ("print", NativeFn(print)),
    ("exit", NativeFn(exit)),
    ("vector-push!", NativeFn(vector_push)),
    ("vector-length", NativeFn(vector_len)),
    ("vector-set!", NativeFn(vector_set)),
    ("vector-get", NativeFn(vector_get)),
    ("vector-pop!", NativeFn(vector_pop)),
    ("map-put!", NativeFn(map_put)),
    ("map-get", NativeFn(map_get)),
    ("map-length", NativeFn(map_length)),
    ("map-remove!", NativeFn(map_remove)),
    ("map-clear!", NativeFn(map_clear))
];

pub fn print(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 1);
    let arg = args[0];
    println!("{:?}", arg);
    (Val::nil(), false)
}

pub fn exit(args: &[Val], _global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 1);
    // TODO
    let placeholder = args[0];
    match args[0].get() {
        Cases::Int(i) => (placeholder, true),
        Cases::Num(i) => (placeholder, true),
        _ => unimplemented!()
    }
}

// (vector-push vector value) -> nil
pub fn vector_push(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 2);
    let (_vector, to_push) = (args[0], args[1]);
    match _vector.get() {
        Cases::Vector(vector) => {
            vector.push(to_push);
        }
        _ => unimplemented!()
    }
    (Val::nil(), false)
}

// (vector-len vector) -> integer
pub fn vector_len(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 1);
    let _vector = args[0] ;
    let len = match _vector.get() {
        Cases::Vector(vector) => {
            vector.len()
        }
        _ => unimplemented!()
    };
    (len, false)
}

// (vector-get vector index) -> item_i
pub fn vector_get(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 2);
    let (_vector, _index) = (args[0], args[1]);
    let item = match (_vector.get(), _index.get()) {
        (Cases::Vector(vector), Cases::Num(index)) => {
            if index < 0.0 { unimplemented!() }
            vector.get(index as usize)
        }
        (Cases::Vector(vector), Cases::Int(index)) => {
            if index < 0 { unimplemented!() }
            vector.get(index as usize)
        }
        _ => unimplemented!()
    }.unwrap();
    (item, false)
}

// (vector-pop vector) -> last-item
pub fn vector_pop(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 1);
    let _vector = args[0];
    let popped = match _vector.get() {
        Cases::Vector(vector) => {
            vector.pop()
        }
        _ => unimplemented!()
    }.expect("Tried to pop an empty vector");
    (popped, false)
}

/// (vector-set vector index addend) -> nil
pub fn vector_set(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 3);
    let (_vector, _index, to_add) = (args[0], args[1], args[2]);
     match (_vector.get(), _index.get()) {
        (Cases::Vector(vector), Cases::Num(index)) => {
            if index < 0.0 { unimplemented!() }
            vector.set(index as usize, to_add)
        }
        (Cases::Vector(vector), Cases::Int(index)) => {
            if index < 0 { unimplemented!() }
            vector.set(index as usize, to_add)
        }
        _ => unimplemented!()
    }.expect("Vector index was out of bounds");
    (Val::nil(), false)
}

/// (map-put map key value) -> old_value
pub fn map_put(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 3);
    let (_map, _key, _val) = (args[0], args[1], args[2]);
    match _map.get() {
        Cases::Map(map) => {
            (map.insert(_key, _val), false)
        }
        _ => unimplemented!()
    }
}

/// (map-get map key) -> value
pub fn map_get(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 2);
    let (_map, _key) = (args[0], args[1]);
    match _map.get() {
        Cases::Map(map) => {
            (map.get(_key), false)
        }
        _ => unimplemented!()
    }
}

/// (map-remove map key) -> old_value
pub fn map_remove(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 2);
    let (_map, _key) = (args[0], args[1]);
    match _map.get() {
        Cases::Map(map) => {
            (map.remove(_key), false)
        }
        _ => unimplemented!()
    }
}

/// (map-count map) -> integer
pub fn map_length(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 1);
    let _map = args[0];
    match _map.get() {
        Cases::Map(map) => {
            (Val::from_int(map.len() as i32), false)
        }
        _ => unimplemented!()
    }
}

/// (map-clear! map)
pub fn map_clear(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 1);
    let _map = args[0];
    match _map.get() {
        Cases::Map(map) => {
            map.clear();
            (Val::nil(), false)
        }
        _ => unimplemented!()
    }
}