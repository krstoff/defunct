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
    use crate::compiler::parse::Primitives;

    use super::*;
    use sexp::print_sexp;
    #[test]
    fn sexp() {
        use read::*;
        let mut idents = super::IdentTable::new();
        let list_str = r"
        (defn add3 [x y z]
          (let ((*sum* (+ x y z))
            sum)))
        ";
        let sexp = {
            let mut reader = Reader::new(list_str, &mut idents);
            reader.read()
        }.unwrap();
        print_sexp(&sexp, &idents);
    }

    #[test]
    fn parsing() {
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
        print!("\n");
        parsed.pprint(&idents, 0);
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