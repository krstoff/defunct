use defunct::global::Global;
use defunct::{compiler::assemble, values};
use values::Val;
use values::Tag;
#[test]
fn functions() {
    let mut global = Global::new();
    let func_obj = assemble("
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

    let entrypoint = assemble(&format!("
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
    let entrypoint = assemble("
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
    let mut vm = defunct::Vm::new(&mut global, entrypoint, &[], false);
    println!("Result: {:?}", vm.run());
}

#[test]
fn vectors() {
    let mut global = Global::new();
    let entrypoint = assemble("
    vecnew
    dup #0
    const :age
    vecpush
    dup #0
    const 30
    vecpush
    dup #0
    const :children
    vecpush
    dup #0 
    const 2
    vecpush
    halt
    ", &mut global).unwrap();
    let mut vm = defunct::Vm::new(&mut global, entrypoint, &[], false);
    println!("Result: {:?}", vm.run());
}