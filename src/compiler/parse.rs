//! Lowers an sexp into an AST after validating the structure.

use crate::bytecode::OpCode;

use super::*;

#[derive(Debug)]
pub enum ParseError {
    MalformedIf,
    MalformedLet,
    MalformedSet,
    MalformedRet,
    UnbalancedLetBindings,
    LetBindingsAreNotSymbols,
    LetBindingsNotInVector,
    FnBindingsAreNotSymbols,
    FnBindingsNotInVector,
    UnbalancedCond,
    PrimOpWrongArity,
}

use ParseError::*;

pub enum Expr {
    NumLiteral(f64),
    VectorLiteral(Vec<Expr>),
    MapLiteral(Vec<(Expr, Expr)>),
    Ident(Ident),
    Apply {
        _fn: Box<Expr>,
        args: Vec<Expr>,
    },
    Let {
        bindings: Vec<(Ident, Expr)>,
        body: Box<Expr>,
    },
    Fn {
        bindings: Vec<Ident>,
        body: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        resultant: Box<Expr>,
        else_branch: Box<Expr>,
    },
    Cond(Vec<(Expr, Expr)>),
    PrimOp {
        op: Ident,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Do(Vec<Expr>),
    Set(Ident, Box<Expr>),
    Ret(Box<Expr>),
}

// Store the symbols for special forms here to enable quick comparisons
pub struct Specials {
    pub _fn: Ident,
    pub _let: Ident,
    pub _cond: Ident,
    pub _if: Ident,
    pub _do: Ident,
    pub _set: Ident,
    pub _ret: Ident,
}

impl Specials {
    pub fn new_in<'a>(st: &mut IdentTable<'a>) -> Specials {
        Specials {
            _fn: st.intern("fn"),
            _let: st.intern("let"),
            _if: st.intern("if"),
            _cond: st.intern("cond"),
            _set: st.intern("set"),
            _do: st.intern("do"),
            _ret: st.intern("return"),
        }
    }
}



// Store the symbols for primitive operations here and map them to opcodes.
pub struct Primitives(std::collections::HashMap<Ident, OpCode>);
impl Primitives {
    pub fn new_in<'a>(it: &mut IdentTable<'a>) -> Primitives {
        let mut primitives = std::collections::HashMap::new();
        for (name, op) in PRIMITIVES {
            let sym = it.intern(name);
            primitives.insert(sym, op);
        }

        Primitives(primitives)
    }
    pub fn get(&self, sym: Ident) -> Option<&OpCode>{
        self.0.get(&sym)
    }
}

pub fn parse(sexp: &Sexp, specials: &Specials, primitives: &Primitives) -> Result<Expr, ParseError> {
    use Sexp::*;
    match sexp {
        Number(num) => Ok(Expr::NumLiteral(*num)),
        Ident(sym) => Ok(Expr::Ident(*sym)),
        List(items) => {
            assert!(items.len() > 0);
            match &items[0] {
                inner_list @ &List(..) => {
                    let head = parse(inner_list, specials, primitives)?;
                    let args = &items[1..];
                    let mut args_eval = Vec::new();
                    for arg in args {
                        args_eval.push(parse(arg, specials, primitives)?)
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
                        condition: Box::new(parse(&items[1], specials, primitives)?),
                        resultant: Box::new(parse(&items[2], specials, primitives)?),
                        else_branch: Box::new(parse(&items[3], specials, primitives)?),
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
                                    _bindings.push((sym, parse(expr, specials, primitives)?));
                                }
                                else {
                                    return Err(LetBindingsAreNotSymbols)
                                }
                            }
                            let mut _exprs = Vec::new();
                            for e in &items[2..] {
                                _exprs.push(parse(e, specials, primitives)?);
                            }
                            Ok(Expr::Let {
                                bindings: _bindings,
                                body: Box::new(Expr::Do(_exprs)),
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
                                body.push(parse(expr, specials, primitives)?);
                            }
                            Ok(Expr::Fn {
                                bindings: _bindings,
                                body: Box::new(Expr::Do(body))
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
                        let case = parse(&cases[2 * i], specials, primitives)?;
                        let branch = parse(&cases[2 * i + 1], specials, primitives)?;
                        _cases.push((case, branch));
                    }
                    Ok(Expr::Cond(_cases))
                }
                Ident(sym) if *sym == specials._do => {
                    let mut body = Vec::new();
                    for expr in &items[1..] {
                        body.push(parse(expr, specials, primitives)?);
                    }
                    Ok(Expr::Do(body))
                }
                Ident(sym) if *sym == specials._set => {
                    if items.len() != 3 {
                        return Err(MalformedSet)
                    }
                    match items[1] {
                        Ident(binding) => {
                            Ok(Expr::Set(binding, Box::new(parse(&items[2], specials, primitives)?)))
                        }
                        _ => Err(MalformedSet)
                    }
                }
                Ident(sym) if *sym == specials._ret => {
                    if items.len() != 2 {
                        return Err(MalformedRet)
                    }
                    Ok(Expr::Ret(Box::new(parse(&items[1], specials, primitives)?)))
                }
                Ident(sym) if primitives.get(*sym).is_some() => {
                    let args = &items[1..];
                    if args.len() != 2 {
                        return Err(PrimOpWrongArity)
                    } else {
                        Ok(Expr::PrimOp {
                            op: *sym,
                            left: Box::new(parse(&args[0], specials, primitives)?),
                            right: Box::new(parse(&args[1], specials, primitives)?),
                        })
                    }
                }
                Ident(sym) => {
                    let args = &items[1..];
                    Ok(Expr::Apply {
                        _fn: Box::new( Expr::Ident(*sym) ),
                        args: parse_list(args, specials, primitives)?,
                    })
                }
                Vector(inner_list) => {
                    let mut vector_eval = Vec::new();
                    for item in inner_list {
                        vector_eval.push(parse(item, specials, primitives)?);
                    }
                    Ok(Expr::Apply {
                        _fn: Box::new(Expr::VectorLiteral(vector_eval)),
                        args: parse_list(&items[1..], specials, primitives)?
                    })
                }
                Map(inner_list) => {
                    let mut map_eval = Vec::new();
                    for (key, val) in inner_list {
                        map_eval.push((parse(key, specials, primitives)?, parse(val, specials, primitives)?));
                    }
                    Ok(Expr::Apply {
                        _fn: Box::new(Expr::MapLiteral(map_eval)),
                        args: parse_list(&items[1..], specials, primitives)?
                    })
                }
                Number(num) => {
                    Ok(Expr::Apply {
                        _fn: Box::new(Expr::NumLiteral(*num)),
                        args: parse_list(&items[1..], specials, primitives)?
                    })
                }
            }
        }
        Vector(items) => {
            let mut _items = Vec::new();
            for i in items {
                _items.push(parse(i, specials, primitives)?);
            }
            Ok(Expr::VectorLiteral(_items))
        }
        Map(items) => {
            let mut _items = Vec::new();
            for (key, val) in items {
                _items.push((parse(key, specials, primitives)?, parse(val, specials, primitives)?));
            }
            Ok(Expr::MapLiteral(_items))
        }
    }
}

fn parse_list(list: &[Sexp], specials: &Specials, primitives: &Primitives) -> Result<Vec<Expr>, ParseError> {
    let mut list_eval = Vec::new();
    for item in list {
        list_eval.push(parse(item, specials, primitives)?)
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
            
            print!("{:width$}", "", width = indent_level + 2);
            body.pprint(idents, indent_level + 2);
        }
        If { condition, resultant, else_branch } => {
            print!("IF ");
            condition.pprint(idents, indent_level + 4);
            print!("{:indent_level$}", "");
            resultant.pprint(idents, indent_level + 2);
            print!("{:indent_level$}", "");
            else_branch.pprint(idents, indent_level + 2);
        }
        PrimOp { op, left, right } => {
            print!("{}\n", idents.get_name(*op));
            print!("{:width$}", "", width=indent_level + 2);
            left.pprint(idents, indent_level + 2);
            print!("{:width$}", "", width=indent_level + 2);
            right.pprint(idents, indent_level + 2);
        }
        Do(exprs) => {
            print!("DO ");
            for expr in exprs {
                print!("{:width$}", "", width = indent_level + 2);
                expr.pprint(idents, indent_level + 2);
            }
        }
        _ => todo!()
        }
    }
}