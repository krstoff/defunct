use crate::values::Val;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum OpCode {
    Const,   // const n {} -> {frame.constants[n]}
    Pop,         // pop n {..n} -> {}
    PopSave,     // popsave n { i_1, ..., i_n, v } -> { v }
    Dup,         // dup n {i_1, i_n, ... } -> {i_1, i_n, ..., i_n}

    Add,         // {num a, num b} -> {a + b}
    Sub,         // {num a, num b} -> {a - b}
    Mul,         // {num a, num b} -> {a * b}
    Div,         // {num a, num b} -> {a / b}
    Lt,          // {num a, num b} -> {a < b}
    Gt,          // {num a, num b} -> {a > b}
    Lte,         // {num a, num b} -> { a <= b }
    Gte,         // {num a, num b} -> { a >= b } 
    Eq,          // {num a, num b} -> { a == b }

    BrNil,       // brnil ip. Checks for nil and jumps to ip
    Jmp,         // jmp ip.   unconditional jump to ip
    Call,        // call n {a, b, c, ..., f} -> {f(a, b, c, ...)}
    Ret,         // ret n {i_1, i_2, i_n, ...} -> {i_n}; pop call frame

    MapSet,      // {map m, k, v} -> {}; m[k] = v
    MapGet,      // {map m, k}    -> {m[k]}
    MapDel,      // {map m, k} -> deletes m[k]
    MapNew,      // {} -> {map m}

    VecNew,
    VecSet,
    VecGet,
    VecPush,
    VecPop,

    SymSet,      // { sym s, v } -> {}; s.value = v
    SymGet,      // { sym s } -> { s.value }

    Halt,        // {v} -> {}; break v;
    // Halt MUST be the last op-code in order for fn to_op to work!
}

pub fn to_op(code: u8) -> OpCode {
    if code > OpCode::Halt as u8 {
        panic!("Invalid opcode detected.");
    } else { unsafe { std::mem::transmute(code) }}
}

impl OpCode {
    pub fn to_str(&self) -> &str {
        use OpCode::*;
        match *self {
            Const => "const",
            Pop => "pop",
            PopSave => "popsave",
            Dup => "dup",
            Add => "add",
            Sub => "sub",
            Mul => "mul",
            Div => "div",
            Lt => "lt",
            Gt => "gt",
            Gte => "gte",
            Lte => "lte",
            Eq => "eq",
            BrNil => "brnil",
            Jmp => "jmp",
            Call => "call",
            Ret => "ret",
            MapSet => "mapset",
            MapGet => "mapget",
            MapDel => "mapdel",
            MapNew => "mapnew",
            VecNew => "vecnew",
            VecSet => "vecset",
            VecGet => "vecget",
            VecPush => "vecpush",
            VecPop => "vecpop",
            SymSet => "symset",
            SymGet => "symget",
            Halt => "halt",
        }
    }

    pub fn has_param(&self) -> bool {
        use OpCode::*;
        match *self {
            Const => true,
            Pop => true,
            PopSave => true,
            Dup => true,
            BrNil => true,
            Call => true,
            Ret => true,
            Jmp => true,
            _ => false,
        }
    }
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
                    write!(f, "{:?}, ", val)?
                }
            }
        }
        unsafe {
            f.write_str("\n---CODE---\n");
            let mut i = 0;
            while i < self.code.len() {
                let op =  to_op((*self.code)[i]);
                write!(f, "{}: ", i)?;
                write!(f, "{}", op.to_str())?;
                if op.has_param() {
                    write!(f, " #{}", (*self.code)[i+1])?;
                    i += 1;
                }
                write!(f, "\n")?;
                i += 1;
            }
            f.write_str("---END---\n")
        }
    }
}