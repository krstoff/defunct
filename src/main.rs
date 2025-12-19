mod alloc;
mod bytecode;
mod vm;
mod values;

use vm::Vm;
use values::Val;

fn main() {
    use bytecode::OpCode::*;

    let mut heap = alloc::Heap::new();
    let l = 500;
    let r = 10.0;
    let sum = l as f64 / r;
    let consts = [
        Val::from_int(l),
        Val::from_num(r),
    ];
    let code = [
        Const as u8,
        0,
        Const as u8,
        1,
        Div as u8,
        Halt as u8,
    ];

    let entrypoint = crate::bytecode::ByteCode {
        consts: &consts,
        code: &code,
    };
    let mut vm = Vm::new(&mut heap, entrypoint, &[]);
    let result = vm.run();
    match result.get_num() {
        Some(i) if i == sum => { println!("Woohoo!"); }
        Some(i) => { println!("Got {} instaed of {}.", i, sum)}
        None => { println!("It wasn't an int...")}
    }
}
