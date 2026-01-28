use super::Ident;
use super::IdentTable;

pub enum Sexp {
    List(Vec<Sexp>),
    Vector(Vec<Sexp>),
    Ident(Ident),
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
            &Sexp::Ident(..) => true,
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
    pub fn is(&self, test: Ident) -> bool {
        match self {
            &Sexp::Ident(sym) if test == sym => true,
            _ => false,
        }
    }
}

pub fn print_sexp(sexp: &Sexp, ident_table: &IdentTable) {
    match sexp {
        &Sexp::List(ref items) => {
            print!("(");
            let mut count = 0;
            for i in items.iter() {
                if count != 0 {
                    print!(" ");
                }
                print_sexp(i, ident_table);
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
                print_sexp(i, ident_table);
                count += 1;
            }
            print!("]");
        }
        &Sexp::Ident(sym) => {
            let name = ident_table.get_name(sym);
            print!("{}", name);
        }
        &Sexp::Number(num) => {
            print!("{}", num)
        }
    }
}