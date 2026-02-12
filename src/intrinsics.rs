use crate::{common::*, global::Global};
use crate::values::{Cases, NativeFn, Val};

pub const INTRINSICS: &[(&str, NativeFn)] = &[
    ("print", NativeFn(print)),
    ("exit", NativeFn(exit)),
    ("vector-push", NativeFn(vector_push)),
    ("vector-len", NativeFn(vector_len)),
    ("vector-set", NativeFn(vector_set)),
    ("vector-get", NativeFn(vector_get)),
    ("vector-pop", NativeFn(vector_pop))
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

pub fn vector_get(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 2);
    let (_vector, _index) = (args[0], args[1]);
    let item = match (_vector.get(), _index.get()) {
        (Cases::Vector(vector), Cases::Num(index)) => {
            vector.get(index as usize)
        }
        (Cases::Vector(vector), Cases::Int(index)) => {
            vector.get(index as usize)
        }
        _ => unimplemented!()
    }.unwrap();
    (item, false)
}

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

pub fn vector_set(args: &[Val], global: &mut Global) -> (Val, bool) {
    assert!(args.len() == 3);
    let (_vector, _index, to_add) = (args[0], args[1], args[2]);
     match (_vector.get(), _index.get()) {
        (Cases::Vector(vector), Cases::Num(index)) => {
            vector.set(index as usize, to_add)
        }
        (Cases::Vector(vector), Cases::Int(index)) => {
            vector.set(index as usize, to_add)
        }
        _ => unimplemented!()
    }.expect("Vector index was out of bounds");
    (Val::nil(), false)
}