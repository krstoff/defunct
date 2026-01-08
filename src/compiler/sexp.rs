use super::Symbol;
use super::SymbolTable;

pub enum Sexp {
    List(Vec<Sexp>),
    Symbol(Symbol),
    Number(f64),
}

pub fn print_sexp(sexp: &Sexp, symbol_table: &SymbolTable) {
    match sexp {
        &Sexp::List(ref items) => {
            print!("(");
            let mut count = 0;
            for i in items.iter() {
                if count != 0 {
                    print!(" ");
                }
                print_sexp(i, symbol_table);
                count += 1;
            }
            print!(")");
        }
        &Sexp::Symbol(sym) => {
            let name = symbol_table.get_name(sym);
            print!("{}", name);
        }
        &Sexp::Number(num) => {
            print!("{}", num)
        }
    }
}