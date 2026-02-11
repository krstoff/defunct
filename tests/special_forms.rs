use defunct::Vm;
use defunct::global::Global;
use defunct::compiler::compile;
use defunct::bytecode::ByteCode;
use defunct::values::{Cases, Val};

fn eval(global: &mut Global, src: &str) -> Val {
    let bytecode = compile(src, &mut global.st).expect("Could not compile bytecode.").pop().unwrap().clone();
    let mut vm = Vm::new(global, bytecode, &[], false);
    let result = vm.run();
    result
}

fn trace(global: &mut Global, src: &str) {
    let mut code_objs = compile(src, &mut global.st).expect("Could not compile bytecode.");
    for obj in &code_objs {
      println!("{:?}", obj);
    }
    let bytecode = code_objs.pop().unwrap();
    let mut vm = Vm::new(global, bytecode, &[], true);
    let result = vm.run();
}

fn eval_and_assert_eq(global: &mut Global, src: &str, test_val: Val) {
    let result = eval(global, src);
    assert_eq!(result, test_val);
}

#[test]
fn test_let() {
    let mut global = Global::new();

    eval_and_assert_eq(&mut global, "
    (let [x 1
          y 2]
      (+ x y))
    ", Val::from_num(3.0f64));

    eval_and_assert_eq(&mut global, "
    (let [x 40
          y 50]
      (let [z 100]
        (+ (* x y) z))))
    ", Val::from_num(2100.0f64));

    eval_and_assert_eq(&mut global, "
    (let [x 1]
      (let [x 2]
        (let [x 3]
          x)))
    ", Val::from_num(3.0f64));

    eval_and_assert_eq(&mut global, "
    (let []
      (* 100 100))
    ", Val::from_num(10000.0f64));

    eval_and_assert_eq(&mut global, "
    (let []
      (+ 2 3)
      (+ 5 2)
      (/ 200 2)
      (* 300 3))
    ", Val::from_num(900.0f64))
}

#[test]
fn test_if() {
  let mut global = Global::new();
  eval_and_assert_eq(&mut global, "
  (if (> 1.0 2.0)
    0.0
    99.0)
  ", Val::from_num(99.0));

  eval_and_assert_eq(&mut global, "
  (let [x 30.0]
     (if (>= x 50.0)
       (if (<= x 75.0)
         3
         4)
       (if (<= x 25.0)
         1
         2)))
  ", Val::from_num(2.0))
}

#[test]
fn test_fn() {
  let mut global = Global::new();
  // todo: closures
  let src = "
  (let [f (fn [x] (+ x 20.0))]
    (f 40.0))
  ";
  eval_and_assert_eq(&mut global, src, Val::from_num(60.0));

  let src = "
  (let [f (fn [x]
            (let [f (fn [x] (+ x 3))]
              (f (* x 2.0))))]
    (f 20.0))
  ";
  eval_and_assert_eq(&mut global, src, Val::from_num(43.0));

  let src = "
  (let [x0 0
        x1 1
        x2 2
        x3 3
        x4 4
        f (fn [x4] (* 10 x4))]
    (f x2))
  ";
  eval_and_assert_eq(&mut global, src, Val::from_num(20.0))
}

// (fn [parameters*] body)
// (cond test1 expr1 test2 expr2 ...)
// (do expr1 expr2 ...)
// (set symbol expr)
// (return expr)