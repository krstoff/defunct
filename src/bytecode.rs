use crate::values::Val;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum OpCode {
    Const = 4,   // {n} -> {frame.constants[n]}
    Pop,         // {n} -> {}
    Load,        // {p} -> {*p}
    Store,       // {p, v} -> {}; *p = v;

    Add,         // {num a, num b} -> {a + b}
    Sub,         // {num a, num b} -> {a - b}
    Mul,         // {num a, num b} -> {a * b}
    Div,         // {num a, num b} -> {a / b}
    Lt,          // {num a, num b} -> {a < b}
    Gt,          // {num a, num b} -> {a > b}
    Lte,         // {num a, num b} -> { a <= b }
    Gte,         // {num a, num b} -> { a >= b } 
    Eq,          // {num a, num b} -> { a == b }

    Halt,        // {v} -> {}; break v;
    // Halt MUST be the last op-code in order for fn to_op to work!
}

pub fn to_op(code: u8) -> OpCode {
    if code > OpCode::Halt as u8 {
        panic!("Invalid opcode detected.");
    } else { unsafe { std::mem::transmute(code) }}
}

pub struct ByteCode {
    pub consts: *const [Val],
    pub code: *const [u8],
}

impl std::fmt::Debug for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpCode::*;
        unsafe {
            f.write_str("---CONSTS---\n");
            for val in &*self.consts {
                if val.is_int() {
                    write!(f, "{}, ", val.get_int().unwrap())?
                }
                else if val.is_num() {
                    write!(f, "{}, ", val.get_num().unwrap())?
                }
                else {
                    write!(f, "{:x}, ", val as *const Val as usize)?
                }
            }
        }
        unsafe {
            f.write_str("\n---CODE---\n");
            let mut i = 0;
            let mut ip = 0;
            while i < self.code.len() {
                write!(f, "{}: ", ip)?;
                match to_op((*self.code)[i]) {
                    Const => {
                        write!(f, "const #{}\n", (*self.code)[i + 1])?;
                        i += 1;
                    }
                    Gt => {
                        write!(f, "gt\n")?;
                    }
                    Lt => {
                        write!(f, "lt\n")?;
                    }
                    Gte => {
                        write!(f, "gte\n")?;
                    }
                    Lte => {
                        write!(f, "lte\n")?;
                    }
                    Eq => {
                        write!(f, "eq\n")?;
                    }
                    Halt => {
                        write!(f, "halt\n")?
                    }
                    _ => unimplemented!()
                }
                ip += 1;
                i += 1;
            }
            f.write_str("---END---\n")
        }
    }
}