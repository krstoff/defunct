mod alloc;
mod bytecode;
mod vm;
mod values;
mod assembler;
mod symbols;

use vm::Vm;
use values::{Val, Tag, Ptr, Closure};

use crate::{bytecode::ByteCode};

fn main() {
    let mut heap = alloc::Heap::new();
    let func_obj = assembler::compile("
    dup #0
    dup #1
    lt
    brnil .gte
    ret #1
.gte
    ret #0
    ").unwrap();

    let closure = values::Closure {
        env: &[],
        code_obj: &func_obj
    };

    let ptr = Val::from_ptr(unsafe { Ptr::new(Tag::Function, &closure as *const _ as u64) });
    let bits = ptr.get_ptr().unwrap().to_bits();

    let entrypoint = assembler::compile(&format!("
    const -30.0
    const 0
    const %{}
    call #2
    halt
    ", bits)).unwrap();

    println!("{:?}", func_obj);
    println!("{:?}", entrypoint);
    let mut vm = Vm::new(&mut heap, entrypoint, &[], true);
    println!("Result: {:?}", vm.run());
}
