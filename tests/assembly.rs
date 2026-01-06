use defunct::global::Global;
use defunct::{assembler, values};
use values::Val;
use values::Tag;
#[test]
fn functions() {
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
    let mut vm = defunct::Vm::new(&mut global, entrypoint, &[], true);
    println!("Result: {:?}", vm.run());
}

#[test]
fn maps() {
    let mut global = Global::new();
    let entrypoint = assembler::compile("
    mapnew
    dup #0
    const :age
    const 30
    mapset
    dup #0
    const :children
    const 2
    mapset
    const :age
    mapget
    halt
    ", &mut global).unwrap();
    let mut vm = defunct::Vm::new(&mut global, entrypoint, &[], true);
    println!("Result: {:?}", vm.run());
}