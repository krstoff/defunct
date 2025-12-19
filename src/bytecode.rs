use crate::values::Val;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum OpCode {
    Const = 0,   // {n} -> {frame.constants[n]}
    Pop,         // {n} -> {}
    Load,        // {p} -> {*p}
    Store,       // {p, v} -> {}; *p = v;
    Add,         // {num a, num b} -> {a + b}
    Sub,         // {num a, num b} -> {a - b}
    Mul,         // {num a, num b} -> {a * b}
    Div,         // {num a, num b} -> {a / b}
    Halt,        // {v} -> {}; break v;
}

pub fn to_op(code: u8) -> OpCode {
    use OpCode::*;
    match code {
        0 => Const,
        1 => Pop,
        2 => Load,
        3 => Store,
        4 => Add,
        5 => Sub,
        6 => Mul,
        7 => Div,
        8 => Halt,
        _ => panic!("Invalid opcode detected")
    }
}

pub struct ByteCode {
    pub consts: *const [Val],
    pub code: *const [u8],
}