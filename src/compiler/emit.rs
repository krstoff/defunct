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
    primitives: &'primitives Primitives
}

pub fn emit<'idents, 'symbols, 'primitives>(
    idents: &'idents IdentTable,
    primitives: &'primitives Primitives,
    symbol_table: &'symbols mut SymbolTable,
    expr: &Expr
) -> ByteCode
{
    let mut scope = Scope { symbols: Vec::new() };
    let mut emitter = Emitter::new(&mut scope, idents, symbol_table, primitives);
    emitter.emit(expr);
    let bytecode = emitter.finish();
    bytecode
}

impl<'scope, 'idents, 'symbols, 'primitives> Emitter<'scope, 'idents, 'symbols, 'primitives> {
    fn new(scope: &'scope mut Scope, idents: &'idents IdentTable, symbol_table: &'symbols mut SymbolTable, primitives: &'primitives Primitives) -> Emitter<'scope, 'idents, 'symbols, 'primitives> {
        Emitter { consts: Vec::new(), code: Vec::new(), sp: 0, scope, idents, symbol_table, primitives }
    }

    fn finish(self) -> ByteCode {
        // todo: allocate this in the heap
        let consts = Box::leak(self.consts.into_boxed_slice()) as *mut _;
        let code = Box::leak(self.code.into_boxed_slice()) as *mut _;
        ByteCode { consts, code }
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
                for expr in body {
                    if expr_counter != 0 {
                        self.push_code(OpCode::Pop as u8); // Discard intermediary results in body
                        self.push_code(1);
                    }
                    self.emit(expr)?;
                    expr_counter += 1;
                }
                for _ in 0..bindings.len() {
                    self.scope.pop();
                }
                self.sp -= bindings.len();
                self.push_code(OpCode::PopSave as u8);
                self.push_code(bindings.len() as u8);
                Ok(())
            }
            Fn { bindings, body } => {
                todo!()
            }
            If { condition, resultant, else_branch } => {
                self.emit(condition)?;
                self.push_code(OpCode::BrNil as u8);
                let false_jmp_param = self.push_code(0); // if condition is false, branch to else-block

                self.emit(resultant)?;
                self.push_code(OpCode::Jmp as u8);
                let true_exit_jmp_param = self.push_code(0); // after resultant block, jmp past end of the else-block

                self.emit(else_branch)?;
                self.write(false_jmp_param, true_exit_jmp_param as u8 + 1);
                self.write(true_exit_jmp_param, self.end() as u8);
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
    symbols: Vec<(Ident, Slot)>,
}

impl Scope {
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