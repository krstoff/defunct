use crate::values::Val;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum OpCode {
    Const,   // const n {} -> {frame.constants[n]}
    Pop,         // pop n {..n} -> {}
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
                    write!(f, "{:?}, ", val)?
                }
            }
        }
        unsafe {
            f.write_str("\n---CODE---\n");
            let mut i = 0;
            while i < self.code.len() {
                write!(f, "{}: ", i)?;
                match to_op((*self.code)[i]) {
                    Const => {
                        write!(f, "const #{}\n", (*self.code)[i + 1])?;
                        i += 1;
                    }
                    Dup => {
                        write!(f, "dup #{}\n", (*self.code)[i+1])?;
                        i += 1;
                    }
                    Pop => {
                        write!(f, "pop #{}\n", (*self.code)[i+1])?;
                        i += 1;   
                    }
                    Add => { write!(f, "add\n")?; }
                    Sub => { write!(f, "sub\n")?; }
                    Mul => { write!(f, "mul\n")?; }
                    Div => { write!(f, "div\n")?; }
                    Gt => { write!(f, "gt\n")?; }
                    Lt => { write!(f, "lt\n")?; }
                    Gte => { write!(f, "gte\n")?; }
                    Lte => { write!(f, "lte\n")?; }
                    Eq => { write!(f, "eq\n")?; }
                    BrNil => {
                        write!(f, "brnil #{}\n", (*self.code)[i+1])?;
                        i += 1;
                    }
                    Call => {
                        write!(f, "call #{}\n", (*self.code)[i+1])?;
                        i += 1;
                    }
                    Ret => {
                        write!(f, "ret #{}\n", (*self.code)[i+1])?;
                        i +=1 ;
                    }
                    MapGet => { write!(f, "mapget\n")?; }
                    MapSet => { write!(f, "mapset\n")?; }
                    MapDel => { write!(f, "mapdel\n")?; }
                    MapNew => { write!(f, "mapnew\n")?; }
                    VecNew => { write!(f, "vecnew\n")?; }
                    VecSet => { write!(f, "vecset\n")?; }
                    VecGet => { write!(f, "vecget\n")?; }
                    VecPush => { write!(f, "vecpush\n")?; }
                    VecPop => { write!(f, "vecpop\n")?; }
                    
                    Halt => {
                        write!(f, "halt\n")?
                    }
                }
                i += 1;
            }
            f.write_str("---END---\n")
        }
    }
}