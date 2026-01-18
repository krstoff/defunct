use super::*;

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
    Symbol(Symbol),
    Apply {
        _fn: Box<Expr>,
        args: Vec<Expr>,
    },
    Let {
        bindings: Vec<(Symbol, Expr)>,
        body: Vec<Expr>,
    },
    Fn {
        bindings: Vec<Symbol>,
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
    pub _fn: Symbol,
    pub _let: Symbol,
    pub _cond: Symbol,
    pub _if: Symbol,
}

impl Specials {
    pub fn new_in<'a>(st: &mut SymbolTable<'a>) -> Specials {
        Specials {
            _fn: st.intern("fn"),
            _let: st.intern("let"),
            _if: st.intern("if"),
            _cond: st.intern("cond"),
        }
    }
}

/// A familiar friend...
fn eval(sexp: &Sexp, specials: &Specials) -> Result<Expr, ParseError> {
    use Sexp::*;
    match sexp {
        Number(num) => Ok(Expr::NumLiteral(*num)),
        Symbol(sym) => Ok(Expr::Symbol(*sym)),
        List(items) => {
            assert!(items.len() > 0);
            match &items[0] {
                inner_list @ &List(..) => {
                    let head = eval(inner_list, specials)?;
                    let args = &items[1..];
                    let mut args_eval = Vec::new();
                    for arg in args {
                        args_eval.push(eval(arg, specials)?)
                    }
                    Ok(Expr::Apply {
                        _fn: Box::new(head),
                        args: args_eval,
                    })
                }
                Symbol(sym) if *sym == specials._if => {
                    if (items.len() != 4) {
                        return Err(MalformedIf)
                    }
                    Ok(Expr::If {
                        condition: Box::new(eval(&items[1], specials)?),
                        resultant: Box::new(eval(&items[2], specials)?),
                        else_branch: Box::new(eval(&items[3], specials)?),
                    })
                }
                Symbol(sym) if *sym == specials._let => {
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
                                if let (&Symbol(sym), expr) = (&bindings[2 * i], &bindings[2 * i + 1]) {
                                    _bindings.push((sym, eval(expr, specials)?));
                                }
                                else {
                                    return Err(LetBindingsAreNotSymbols)
                                }
                            }
                            let mut _exprs = Vec::new();
                            for e in &items[2..] {
                                _exprs.push(eval(e, specials)?);
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
                Symbol(sym) if *sym == specials._fn => {
                    match &items[1] {
                        Vector(bindings) => {
                            let mut _bindings = Vec::new();
                            for b in bindings {
                                match b {
                                    Symbol(sym) => { _bindings.push(*sym) }
                                    _ => { return Err(FnBindingsAreNotSymbols) }
                                }
                            }
                            let mut body = Vec::new();
                            for expr in &items[2..] {
                                body.push(eval(expr, specials)?);
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
                Symbol(sym) if *sym == specials._cond => {
                    let cases = &items[1..];
                    if cases.len() % 2 != 0 {
                        return Err(UnbalancedCond)
                    }
                    let mut _cases = Vec::new();
                    for i in 0..cases.len() {
                        let case = eval(&cases[2 * i], specials)?;
                        let branch = eval(&cases[2 * i + 1], specials)?;
                        _cases.push((case, branch));
                    }
                    Ok(Expr::Cond(_cases))
                }
                Symbol(sym) => {
                    let args = &items[1..];
                    Ok(Expr::Apply {
                        _fn: Box::new( Expr::Symbol(*sym) ),
                        args: evlist(args, specials)?,
                    })
                }
                Vector(inner_list) => {
                    let mut vector_eval = Vec::new();
                    for item in inner_list {
                        vector_eval.push(eval(item, specials)?);
                    }
                    Ok(Expr::Apply {
                        _fn: Box::new(Expr::VectorLiteral(vector_eval)),
                        args: evlist(&items[1..], specials)?
                    })
                }
                Number(num) => {
                    Ok(Expr::Apply {
                        _fn: Box::new(Expr::NumLiteral(*num)),
                        args: evlist(&items[1..], specials)?
                    })
                }
            }
        }
        Vector(items) => {
            let mut _items = Vec::new();
            for i in items {
                _items.push(eval(i, specials)?);
            }
            Ok(Expr::VectorLiteral(_items))
        }
    }
}

fn evlist(list: &[Sexp], specials: &Specials) -> Result<Vec<Expr>, ParseError> {
    let mut list_eval = Vec::new();
    for item in list {
        list_eval.push(eval(item, specials)?)
    }
    Ok(list_eval)
}