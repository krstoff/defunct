mod common;
use common::*;

#[test]
fn basic_operations() {
    let mut global = &mut Global::new();
    let src = "
    (let [v []]
      (vector-push! v 0)
      (vector-push! v 1) 
      (vector-push! v 2)
      (vector-push! v 3)
      (vector-push! v 4)
      (vector-get v 2))
    ";
    eval_and_assert_eq(&mut global, src, Val::from_num(2.0));

    let src = "(let [v [1 2 3 4 5]] (vector-length v))";
    eval_and_assert_eq(&mut global, src, Val::from_int(5));

    let src = ("(let [v [1 2 3 4 5]] (vector-set! v 3 -1.0) (vector-get v 3))");
    eval_and_assert_eq(&mut global, src, Val::from_num(-1.0));

    let src = "
    (let [v [1 2 3]]
      (vector-pop! v)
      (vector-pop! v)
      (vector-pop! v)
      (vector-length v))
    ";
    eval_and_assert_eq(&mut global, src, Val::from_int(0));
}


#[test]
#[should_panic]
fn bounds_respected() {
    let mut global = &mut Global::new();
    let src = "
    (let [v []]
      (vector-get v 0))
    ";
    eval(&mut global, src);
}

#[test]
#[should_panic]
fn bounds_respected2() {
    let mut global = &mut Global::new();
    let src = "
    (let [v [1 2 3 4]]
      (vector-get v -1))
    ";
    eval(&mut global, src);
}

#[test]
#[should_panic]
fn bounds_respected3() {
    let mut global = &mut Global::new();
    let src = "
    (let [v [1]]
      (vector-pop! v)
      (vector-pop! v))
    ";
    eval(&mut global, src);
}

#[test]
#[should_panic]
fn bounds_respected4() {
    let mut global = &mut Global::new();
    let src = "
    (let [v [1]]
      (vector-set! v 1 100))
    ";
    eval(&mut global, src);
}