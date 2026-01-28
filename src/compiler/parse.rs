//! Lowers an sexp into an AST after validating the structure.

use super::*;

#[derive(Debug)]
pub enum ParseError {
    MalformedIf,
    MalformedLet,
    UnbalancedLetBindings,
    LetBindingsAreNotSymbols,
    LetBindingsNotInVector,
    FnBindingsAreNotSymbols,
    FnBindingsNotInVector,
    UnbalancedCond
}

use ParseError::*;

pub enum Expr {
    NumLiteral(f64),
    VectorLiteral(Vec<Expr>),
    Ident(Ident),
    Apply {
        _fn: Box<Expr>,
        args: Vec<Expr>,
    },
    Let {
        bindings: Vec<(Ident, Expr)>,
        body: Vec<Expr>,
    },
    Fn {
        bindings: Vec<Ident>,
        body: Vec<Expr>,
    },
    If {
        condition: Box<Expr>,
        resultant: Box<Expr>,
        else_branch: Box<Expr>,
    },
    Cond(Vec<(Expr, Expr)>),
}

// Store the symbols for special forms here to enable quick comparisons
pub struct Specials {
    pub _fn: Ident,
    pub _let: Ident,
    pub _cond: Ident,
    pub _if: Ident,
}

impl Specials {
    pub fn new_in<'a>(st: &mut IdentTable<'a>) -> Specials {
        Specials {
            _fn: st.intern("fn"),
            _let: st.intern("let"),
            _if: st.intern("if"),
            _cond: st.intern("cond"),
        }
    }
}

pub fn parse(sexp: &Sexp, specials: &Specials) -> Result<Expr, ParseError> {
    use Sexp::*;
    match sexp {
        Number(num) => Ok(Expr::NumLiteral(*num)),
        Ident(sym) => Ok(Expr::Ident(*sym)),
        List(items) => {
            assert!(items.len() > 0);
            match &items[0] {
                inner_list @ &List(..) => {
                    let head = parse(inner_list, specials)?;
                    let args = &items[1..];
                    let mut args_eval = Vec::new();
                    for arg in args {
                        args_eval.push(parse(arg, specials)?)
                    }
                    Ok(Expr::Apply {
                        _fn: Box::new(head),
                        args: args_eval,
                    })
                }
                Ident(sym) if *sym == specials._if => {
                    if (items.len() != 4) {
                        return Err(MalformedIf)
                    }
                    Ok(Expr::If {
                        condition: Box::new(parse(&items[1], specials)?),
                        resultant: Box::new(parse(&items[2], specials)?),
                        else_branch: Box::new(parse(&items[3], specials)?),
                    })
                }
                Ident(sym) if *sym == specials._let => {
                    if items.len() < 3 {
                        return Err(MalformedLet)
                    }
                    match &items[1] {
                        Vector(bindings) => {
                            if bindings.len() % 2 != 0 {
                                return Err(UnbalancedLetBindings)
                            }
                            let mut _bindings = Vec::new();
                            for i in 0..bindings.len() / 2 {
                                if let (&Ident(sym), expr) = (&bindings[2 * i], &bindings[2 * i + 1]) {
                                    _bindings.push((sym, parse(expr, specials)?));
                                }
                                else {
                                    return Err(LetBindingsAreNotSymbols)
                                }
                            }
                            let mut _exprs = Vec::new();
                            for e in &items[2..] {
                                _exprs.push(parse(e, specials)?);
                            }
                            Ok(Expr::Let {
                                bindings: _bindings,
                                body: _exprs,
                            })
                        }
                        _ => {
                            return Err(LetBindingsNotInVector)
                        }
                    }
                }
                Ident(sym) if *sym == specials._fn => {
                    match &items[1] {
                        Vector(bindings) => {
                            let mut _bindings = Vec::new();
                            for b in bindings {
                                match b {
                                    Ident(sym) => { _bindings.push(*sym) }
                                    _ => { return Err(FnBindingsAreNotSymbols) }
                                }
                            }
                            let mut body = Vec::new();
                            for expr in &items[2..] {
                                body.push(parse(expr, specials)?);
                            }
                            Ok(Expr::Fn {
                                bindings: _bindings,
                                body
                            })
                        }
                        _ => {
                            return Err(FnBindingsNotInVector);
                        }
                    }
                }
                Ident(sym) if *sym == specials._cond => {
                    let cases = &items[1..];
                    if cases.len() % 2 != 0 {
                        return Err(UnbalancedCond)
                    }
                    let mut _cases = Vec::new();
                    for i in 0..cases.len() {
                        let case = parse(&cases[2 * i], specials)?;
                        let branch = parse(&cases[2 * i + 1], specials)?;
                        _cases.push((case, branch));
                    }
                    Ok(Expr::Cond(_cases))
                }
                Ident(sym) => {
                    let args = &items[1..];
                    Ok(Expr::Apply {
                        _fn: Box::new( Expr::Ident(*sym) ),
                        args: parse_list(args, specials)?,
                    })
                }
                Vector(inner_list) => {
                    let mut vector_eval = Vec::new();
                    for item in inner_list {
                        vector_eval.push(parse(item, specials)?);
                    }
                    Ok(Expr::Apply {
                        _fn: Box::new(Expr::VectorLiteral(vector_eval)),
                        args: parse_list(&items[1..], specials)?
                    })
                }
                Number(num) => {
                    Ok(Expr::Apply {
                        _fn: Box::new(Expr::NumLiteral(*num)),
                        args: parse_list(&items[1..], specials)?
                    })
                }
            }
        }
        Vector(items) => {
            let mut _items = Vec::new();
            for i in items {
                _items.push(parse(i, specials)?);
            }
            Ok(Expr::VectorLiteral(_items))
        }
    }
}

fn parse_list(list: &[Sexp], specials: &Specials) -> Result<Vec<Expr>, ParseError> {
    let mut list_eval = Vec::new();
    for item in list {
        list_eval.push(parse(item, specials)?)
    }
    Ok(list_eval)
}

impl Expr {
    pub fn pprint(&self, idents: &IdentTable, indent_level: usize) {
        use parse::Expr::*;
        match self {
            NumLiteral(num) => print!("{}i\n", num),
            VectorLiteral(items) => {
                print!("{:indent_level$}VEC\n", "");
                for i in items {
                    i.pprint(idents, indent_level + 2);
                }
            }
            Ident(i) => {
                print!("{}\n", idents.get_name(*i))
            }
            Apply { _fn, args } => {
                print!("APPLY ");
                _fn.pprint(idents, indent_level + 6);
                for a in args {
                    print!("{:width$}", "", width=indent_level + 2);
                    a.pprint(idents, indent_level + 2);
                }
            }
        Let { bindings, body } => {
            print!("LET \n");
            match bindings.len() {
                0 => print!("[]\n"),
                n => {
                    for (binding, expr) in bindings {
                        let name = idents.get_name(*binding);
                        print!("{:width$}{} ", "", name, width=(indent_level + 4));
                        expr.pprint(idents, indent_level + 4 + name.len());
                    }
                }
            }
            for expr in body {
                print!("{:width$}", "", width = indent_level + 2);
                expr.pprint(idents, indent_level + 2);
            }
        }
        If { condition, resultant, else_branch } => {
            print!("IF ");
            condition.pprint(idents, indent_level + 4);
            print!("{:indent_level$}", "");
            resultant.pprint(idents, indent_level + 2);
            print!("{:indent_level$}", "");
            else_branch.pprint(idents, indent_level + 2);
        }
        _ => todo!()
        }
    }
}