mod common;
use common::*;

#[test]
fn basic_operations() {
    let mut global = Global::new();
    let src = "
    (let [m {}]
      (map-put! m :a 1)
      (map-put! m :b 2)
      (map-put! m :c 3)
      (map-get m :b))
    ";

    eval_and_assert_eq(&mut global, src, Val::from_num(2.0));

    let src = "(let [m {:a 1 :b 2}] (map-length m))";
    eval_and_assert_eq(&mut global, src, Val::from_int(2));

    let src = "
    (let [m {:a 1 :b 2}]
      (map-clear! m)
      (map-length m))
    ";
    eval_and_assert_eq(&mut global, src, Val::from_int(0));

    let src = "(let [m {:a 1, :b 2}] (map-remove! m :a) (map-get m :a))";
    eval_and_assert_eq(&mut global, src, Val::nil());

    let src = "(let [m {:a 1, :b 2}] (map-remove! m :a))";
    eval_and_assert_eq(&mut global, src, Val::from_num(1.0));
}