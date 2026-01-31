use std::result;

use super::*;
use parse::Expr;
use crate::{bytecode::ByteCode, compiler::parse::Primitives, values::{SymbolTable, Val}};
/// Walks an AST, emitting bytecode instructions into bytecode objects in the program heap
pub struct Emitter<'scope, 'idents, 'symbols, 'primitives> {
    consts: Vec<Val>,
    code: Vec<u8>,
    sp: usize,
    scope: &'scope mut Scope,
    idents: &'idents IdentTable<'idents>,
    symbol_table: &'symbols mut SymbolTable,
    primitives: &'primitives Primitives,
    code_objs: Vec<Val>,
}

pub fn emit<'idents, 'symbols, 'primitives>(
    idents: &'idents IdentTable,
    primitives: &'primitives Primitives,
    symbol_table: &'symbols mut SymbolTable,
    expr: &Expr
) -> Vec<Val>
{
    let mut scope = Scope::new();
    let mut emitter = Emitter::new(&mut scope, idents, symbol_table, primitives);
    emitter.emit(expr);
    let objs = emitter.finish();
    objs
}

impl<'scope, 'idents, 'symbols, 'primitives> Emitter<'scope, 'idents, 'symbols, 'primitives> {
    fn new(scope: &'scope mut Scope, idents: &'idents IdentTable, symbol_table: &'symbols mut SymbolTable, primitives: &'primitives Primitives) -> Emitter<'scope, 'idents, 'symbols, 'primitives> {
        Emitter { consts: Vec::new(), code: Vec::new(), sp: 0, scope, idents, symbol_table, primitives, code_objs: Vec::new() }
    }

    fn finish(mut self) -> Vec<Val> {
        // todo: allocate this in the heap
        let consts = Box::leak(self.consts.into_boxed_slice()) as *mut _;
        let code = Box::leak(self.code.into_boxed_slice()) as *mut _;
        let code_obj = ByteCode::new(consts, code);
        self.code_objs.push(code_obj);
        self.code_objs
    }

    fn push_code(&mut self, byte: u8) -> usize {
        assert!(self.code.len() < 256);
        self.code.push(byte);
        self.code.len() - 1
    }

    fn push_const(&mut self, val: Val) {
        assert!(self.consts.len() < 256);
        self.consts.push(val);
    }

    fn write(&mut self, code_index: usize, byte: u8) {
        self.code[code_index] = byte;
    }

    fn end(&self) -> usize {
        self.code.len()
    }

    fn emit(&mut self, expr: &Expr) -> Result<(), CompileError> {
        use Expr::*;
        use crate::bytecode::OpCode;
        match expr {
            NumLiteral(num) => {
                self.push_code(OpCode::Const as u8);
                self.push_code(self.consts.len() as u8);
                self.push_const(Val::from_num(*num));
                Ok(())
            }
            VectorLiteral(_) => {
                todo!()
            }
            Ident(symbol) => {
                if let Some(slot) = self.scope.lookup(symbol) {
                    if(slot > 256) {
                        return Err(CompileError::SlotTooLarge(slot))
                    }
                    self.push_code(OpCode::Dup as u8);
                    self.push_code(slot as u8);
                    Ok(())
                } else {
                    // Dynamic symbol lookup
                    let name = self.idents.get_name(*symbol);
                    let interned_symbol = self.symbol_table.intern(name);
                    self.push_code(OpCode::Const as u8);
                    self.push_code(self.consts.len() as u8);
                    self.push_const(interned_symbol.as_val());
                    self.push_code(OpCode::SymGet as u8);
                    Ok(())
                }
            }
            PrimOp { op, left, right } => {
                let opcode = self.primitives.get(*op).unwrap();
                self.emit(left);
                self.sp += 1;
                self.emit(right);
                self.push_code(*opcode as u8);
                self.sp -= 1;
                Ok(())
            }
            Apply { _fn, args } => {
                for arg in args {
                    self.emit(arg)?;
                    self.sp += 1;
                }

                self.emit(_fn)?;
                self.push_code(OpCode::Call as u8);
                self.push_code(args.len() as u8);

                self.sp -= args.len();
                Ok(())
            }
            Let { bindings, body } => {
                let mut new_bindings = Vec::new();
                for (binding, expr) in bindings {
                    self.emit(expr)?;
                    new_bindings.push((*binding, self.sp));
                    self.sp += 1;
                }
                for (binding, slot) in new_bindings.iter() {
                    self.scope.push(binding, *slot);
                }
                let mut expr_counter = 0;
                self.emit(body)?;
                for _ in 0..bindings.len() {
                    self.scope.pop();
                }
                self.sp -= bindings.len();
                self.push_code(OpCode::PopSave as u8);
                self.push_code(bindings.len() as u8);
                Ok(())
            }
            Fn { bindings, body } => {
                let mut scope = Scope::new();
                let mut sp = 0;
                for b in bindings {
                    scope.push(b, sp);
                    sp += 1;
                }
                let mut body_emitter = Emitter::new(&mut scope, self.idents, self.symbol_table, self.primitives);
                let mut first_expression = true;
                body_emitter.emit(body)?;
                body_emitter.push_code(OpCode::Ret as u8);
                body_emitter.push_code(bindings.len() as u8);

                let mut code_objs = body_emitter.finish();
                self.push_code(OpCode::Closure as u8);
                self.push_code(self.consts.len() as u8);
                self.push_const(code_objs[code_objs.len() - 1]);
                self.code_objs.append(&mut code_objs);
                Ok(())
            }
            Do(exprs) => {
                let mut first_expression = true;
                for expr in exprs {
                    if !first_expression {
                        self.push_code(OpCode::Pop as u8);
                        self.push_code(1);
                        first_expression = false;
                    }
                    self.emit(expr)?;
                }
                Ok(())
            }
            If { condition, resultant, else_branch } => {
                self.emit(condition)?;
                self.push_code(OpCode::BrNil as u8);
                let false_jmp_param = self.push_code(0); // if condition is false, branch to else-block

                self.emit(resultant)?;
                self.push_code(OpCode::Jmp as u8);
                let true_exit_jmp_param = self.push_code(0); // after resultant block, jmp past end of the else-block

                self.emit(else_branch)?;
                self.write(false_jmp_param, (true_exit_jmp_param - false_jmp_param - 1) as u8);
                self.write(true_exit_jmp_param, self.end() as u8);
                Ok(())
            }
            Set(symbol, value) => {
                if let Some(slot) = self.scope.lookup(symbol) {
                    todo!() // cannot set locals just yet.
                } else {
                    // Dynamic symbol lookup
                    let name = self.idents.get_name(*symbol);
                    let interned_symbol = self.symbol_table.intern(name);
                    self.push_code(OpCode::Const as u8);
                    self.push_code(self.consts.len() as u8);
                    self.push_const(interned_symbol.as_val());
                    self.emit(value)?;
                    self.push_code(OpCode::SymSet as u8);
                    Ok(())
                }
            }
            Ret(expr) => {
                self.emit(expr)?;
                self.push_code(OpCode::Ret as u8);
                self.push_code(self.sp as u8);
                Ok(())
            }
            Cond(cases) => {
                todo!()
            }
        }
    }
}

type Slot = usize;

struct Scope {
    symbols: allocator_api2::vec::Vec::<(Ident, Slot), crate::alloc::Heap>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {symbols: allocator_api2::vec::Vec::new_in(crate::alloc::Heap)}
    }
    pub fn push(&mut self, sym: &Ident, slot: Slot) {
        self.symbols.push((*sym, slot))
    }

    pub fn pop(&mut self) {
        self.symbols.pop().expect("Unbalanced scope exit");
    }

    pub fn lookup(&self, symbol: &Ident) -> Option<Slot> {
        self.symbols.iter().find(
            |(sym, slot)| sym == symbol
        )
        .map(|(sym, slot)| *slot)
    }
}

pub enum CompileError {
    SlotTooLarge(usize)
}