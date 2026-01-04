mod alloc;
mod bytecode;
mod vm;
mod values;
mod assembler;
mod symbols;
mod global;

use vm::Vm;
use values::{Val, Tag, Ptr, Closure};

use crate::{bytecode::ByteCode, global::Global};

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

    let closure = values::Closure {
        env: &[],
        code_obj: &func_obj
    };

    let ptr = Val::from_ptr(Tag::Function, &closure as *const _ as *const u8);
    let bits = ptr.get_ptr().unwrap().to_bits();

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
