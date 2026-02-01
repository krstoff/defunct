mod idents;
mod sexp;
mod read;
mod parse;
mod emit;
mod assembler;

use sexp::Sexp;
use idents::{Ident, IdentTable};
use read::Reader;
use parse::{Specials, parse};
use crate::{bytecode::OpCode, values::SymbolTable};
pub use assembler::assemble;
use crate::bytecode::ByteCode;

const PRIMITIVES: [(&'static str, OpCode); 9] = [
    ("+", OpCode::Add),
    ("-", OpCode::Sub),
    ("*", OpCode::Mul),
    ("/", OpCode::Div),
    ("<", OpCode::Lt),
    (">", OpCode::Gt),
    ("<=", OpCode::Lte),
    (">=", OpCode::Gte),
    ("eq", OpCode::Eq),
];

#[cfg(test)]
mod test {
    use super::*;
    use parse::Primitives;
    #[test]
    fn sexp() {
        use read::*;
        use sexp::Sexp::*;
        let mut idents = super::IdentTable::new();
        let list_str = r"
        (defn add3 [x y z]
          (let ((*sum* (+ x y z)))
            sum))
        ";
        let sexp = {
            let mut reader = Reader::new(list_str, &mut idents);
            reader.read()
        }.expect("failed to read sexp");

        
        let name_of = |ident: &_| idents.get_name(*ident);
        match sexp {
            List(items) => {
                match (&items[0], &items[1], &items[2], &items[3]) {
                    (Ident(defn), Ident(add3), Vector(bindings), List(body)) if name_of(defn) == "defn" && name_of(add3) == "add3" => {
                        for b in bindings {
                            match b {
                                Ident(i) => { continue; }
                                _ => { panic!("Expected a symbol in bindings list.") }
                            }
                        }
                        match &body[..] {
                            [Ident(head), List(bindings), Ident(sum)] if name_of(head) == "let" && name_of(sum) == "sum" => {
                                match &bindings[..] {
                                    [List(items)] => {
                                        match &items[..] {
                                            [Ident(sum), List(binding)] => {
                                                match &binding[..] {
                                                    [Ident(plus), Ident(x), Ident(y), Ident(z)] if 
                                                        name_of(sum) == "*sum*" &&
                                                        name_of(x) == "x" &&
                                                        name_of(y) == "y" &&
                                                        name_of(z) == "z" => {
                                                            return;
                                                    }
                                                    _ => panic!("Failed at point a")
                                                }
                                            }
                                            _ => panic!("Failed at point b")
                                        }
                                    }
                                    _ => panic!("Failed at point c")
                                }
                            }
                            _ => panic!("Failed at point d")
                        }
                    }
                    _ => panic!("Failed at point e")
                }
            }
            _ => panic!("Failed at point f")
        }
        panic!("Read form did not match expected structure.");
    }

    #[test]
    fn parsing() {
        use parse::Expr::*;
        let mut idents = IdentTable::new();
        let list_str = r"
        (let [x 0
              y 1
              z (* 62 42)]
          (let [sum (+ x (+ y z))]
            (if (< sum 1000)
              1000
              (* sum sum))))
        ";
        let sexp = {
            let mut reader = Reader::new(list_str, &mut idents);
            reader.read()
        }.unwrap();
        let specials = Specials::new_in(&mut idents);
        let primitives = Primitives::new_in(&mut idents);
        let parsed = parse(&sexp, &specials, &primitives).unwrap();
        
        let name_of = |ident: &_| idents.get_name(*ident);
        match parsed {
            Let { bindings, body } => {
                match &bindings[..] {
                    [(x, NumLiteral(0.0)), (y, NumLiteral(1.0)), (z, PrimOp { op: times, left, right })] 
                    if name_of(x) == "x" && name_of(y) == "y" && name_of(z) == "z" 
                    && name_of(times) == "*" => {
                        // at this point I got tired of matching through boxes without box patterns
                        return;
                    }
                    _ => panic!("Failed to parse at point a")
                }
            }
            _ => panic!("Failed ot parse at point b")
        }
        
    }

    #[test]
    fn emit() {
        let mut symbols = SymbolTable::new();
        let mut idents = IdentTable::new();
        let specials = Specials::new_in(&mut idents);
        
        let src = r"
(set transform
  (fn [t]
    (let [x (* t t)
          y (+ t 1)
          z (* 62 42)]
      (let [sum (+ x (+ y z))]
        (if (< sum 1000)
          1000
          (* sum sum))))))

        ";
        let sexp = {
            let mut reader = Reader::new(src, &mut idents);
            reader.read()
        }.unwrap();
        let primitives = Primitives::new_in(&mut idents);
        let parsed = parse(&sexp, &specials, &primitives).unwrap();
        let objects = emit::emit(&idents, &primitives, &mut symbols, &parsed);
        for obj in objects.iter() {
            let bytecode: &ByteCode = obj.try_into().unwrap();
            println!("\n{:?}", bytecode);
        }
    }
}