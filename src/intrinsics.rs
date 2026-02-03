use crate::{common::*, global::Global};
use crate::values::{Cases, NativeFn, Val};

pub fn print(args: &[Val], global: &Global) -> (Val, bool) {
    assert!(args.len() == 1);
    let arg = args[0];
    println!("{:?}", arg);
    (Val::nil(), false)
}

pub fn exit(args: &[Val], _global: &Global) -> (Val, bool) {
    assert!(args.len() == 1);
    // TODO
    let placeholder = args[0];
    match args[0].get() {
        Cases::Int(i) => (placeholder, true),
        Cases::Num(i) => (placeholder, true),
        _ => unimplemented!()
    }
}

pub const INTRINSICS: &[(&str, NativeFn)] = &[
    ("print", NativeFn(print)),
    ("exit", NativeFn(exit))
];