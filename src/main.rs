mod alloc;
mod bytecode;
mod vm;
mod values;
mod assembler;
mod symbols;
mod global;

use std::ptr;

use vm::Vm;
use values::{Val, Tag, Closure};

use crate::{bytecode::ByteCode, global::Global};

thread_local! {
    static HEAP: alloc::Heap = alloc::Heap::new();
}

fn main() {
    let mut global = Global::new();
    let func_obj = assembler::compile("
    dup #0
    dup #1
    lt
    brnil .gte
    ret #1
.gte
    ret #0
    ", &mut global).unwrap();

    let mut closure = values::Closure {
        env: &[],
        code_obj: &func_obj
    };

    let ptr = Val::from_ptr(Tag::Function, &mut closure as *mut _ as *mut u8);
    let bits = ptr.bits();

    let entrypoint = assembler::compile(&format!("
    const 30
    const 100 
    const %{}
    call #2
    const :toodaloo
    halt
    ", bits), &mut global).unwrap();

    println!("{:?}", func_obj);
    println!("{:?}", entrypoint);
    let mut vm = Vm::new(&mut global, entrypoint, &[], true);
    println!("Result: {:?}", vm.run());
}
