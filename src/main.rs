mod alloc;
mod bytecode;
mod vm;
mod values;
mod assembler;
mod symbols;

use vm::Vm;
use values::Val;

fn main() {
    let mut heap = alloc::Heap::new();
    let text = "
    .start
        const 8
        const 9
        mul
        const 1
        sub
        const 70
        lte
        brnil .small
        const 999
        halt

    .small
        const 1
        halt
    ";
    let bytecode = assembler::compile(text).unwrap();
    println!("{:?}", bytecode);
    let mut vm = Vm::new(&mut heap, bytecode, &[]);
    println!("Result: {:?}", vm.run());
}
