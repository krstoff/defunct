pub use defunct::Vm;
pub use defunct::global::Global;
pub use defunct::compiler::compile;
pub use defunct::bytecode::ByteCode;
pub use defunct::values::{Cases, Val};

pub fn eval(global: &mut Global, src: &str) -> Val {
    let bytecode = compile(src, &mut global.st).expect("Could not compile bytecode.").pop().unwrap().clone();
    let mut vm = Vm::new(global, bytecode, &[], false);
    let result = vm.run();
    result
}

pub fn trace(global: &mut Global, src: &str) {
    let mut code_objs = compile(src, &mut global.st).expect("Could not compile bytecode.");
    for obj in &code_objs {
      println!("{:?}", obj);
    }
    let bytecode = code_objs.pop().unwrap();
    let mut vm = Vm::new(global, bytecode, &[], true);
    let result = vm.run();
}

pub fn eval_and_assert_eq(global: &mut Global, src: &str, test_val: Val) {
    let result = eval(global, src);
    assert_eq!(result, test_val);
}
