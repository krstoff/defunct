use super::Symbol;
use super::SymbolTable;

pub enum Sexp {
    List(Vec<Sexp>),
    Vector(Vec<Sexp>),
    Symbol(Symbol),
    Number(f64),
}

impl Sexp {
    pub fn is_list(&self) -> bool {
        match self {
            &Sexp::List(..) => true,
            _ => false,
        }
    }
    pub fn is_symbol(&self) -> bool {
        match self {
            &Sexp::Symbol(..) => true,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            &Sexp::Number(..) => true,
            _ => false,
        }
    }
    pub fn is_vector(&self) -> bool {
        match self {
            &Sexp::Vector(..) => true,
            _ => false,
        }
    }
    pub fn is(&self, test: Symbol) -> bool {
        match self {
            &Sexp::Symbol(sym) if test == sym => true,
            _ => false,
        }
    }
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
        &Sexp::Vector(ref items) => {
            print!("[");
            let mut count = 0;
            for i in items.iter() {
                if count != 0 {
                    print!(" ");
                }
                print_sexp(i, symbol_table);
                count += 1;
            }
            print!("]");
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