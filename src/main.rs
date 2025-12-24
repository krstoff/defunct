mod alloc;
mod bytecode;
mod vm;
mod values;
mod assembler;

use vm::Vm;
use values::Val;

fn main() {
    let mut heap = alloc::Heap::new();
    let text = "
        const 30.0
        const -202.3
        gt
        halt
    ";
    let bytecode = assembler::assemble(text).unwrap();
    println!("{:?}", bytecode);
    let mut vm = Vm::new(&mut heap, bytecode, &[]);
    println!("Result: {:?}", vm.run());
}
