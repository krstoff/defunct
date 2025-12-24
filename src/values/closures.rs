use crate::bytecode::ByteCode;
use super::Val;

pub struct Closure {
    pub env: *const [Val],
    pub code_obj: *const ByteCode
}