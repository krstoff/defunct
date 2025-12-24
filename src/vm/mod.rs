mod boolean;

use crate::values::Val;
use crate::alloc::Heap;
use crate::bytecode::OpCode;
use crate::vm::boolean::nil;

#[derive(Copy, Clone)]
struct Frame {
    ip: usize,
    constants: *const [Val],
    code: *const [u8]
}

pub struct Vm<'a> {
    heap: &'a mut Heap,
    fp: Frame,
    frames: Vec<Frame>,
    values: Vec<Val>,
}

macro_rules! primitive_math_op {
    ($self:expr, $bin_op:tt) => {{
        let right = $self.pop();
        let left = $self.pop();

        if left.is_int() && right.is_int() {
            let result = left.get_int().unwrap() $bin_op right.get_int().unwrap();
            $self.push(Val::from_int(result));
        } 
                
        else if left.is_num() && right.is_num() {
            let result = left.get_num().unwrap() $bin_op right.get_num().unwrap();
            $self.push(Val::from_num(result));
        }
                
        else if left.is_ptr() || right.is_ptr() {
            // TODO: TypeErr
            unimplemented!();
        }

        else {
            let result =
                left.get_int().map(|i| i as f64).or(left.get_num()).unwrap()
                $bin_op right.get_int().map(|i| i as f64).or(right.get_num()).unwrap();
            $self.push(Val::from_num(result));
        }
                // TODO: TypeErr
    }}
}

macro_rules! primitive_logic_op {
    ($self:expr, $bin_op:tt) => {{
        use boolean::*;
        let right = $self.pop();
        let left = $self.pop();

        if left.is_int() && right.is_int() {
            let result = left.get_int().unwrap() $bin_op right.get_int().unwrap();
            
            $self.push(if result { t() } else { nil() });
        } 
                
        else if left.is_num() && right.is_num() {
            let result = left.get_num().unwrap() $bin_op right.get_num().unwrap();
            $self.push(if result { t() } else { nil() });
        }
                
        else if left.is_ptr() || right.is_ptr() {
            // TODO: TypeErr
            unimplemented!();
        }

        else {
            let result =
                left.get_int().map(|i| i as f64).or(left.get_num()).unwrap()
                $bin_op right.get_int().map(|i| i as f64).or(right.get_num()).unwrap();
            $self.push(if result { t() } else { nil() });
        }
                // TODO: TypeErr
    }}
}


impl<'a> Vm<'a> {
    pub fn new(heap: &'a mut Heap, entrypoint: crate::bytecode::ByteCode, initargs: &[Val]) -> Vm <'a> {
        let mut values = vec![];
        for val in initargs {
            values.push(*val);
        }
        let frames = vec![];
        let initial_frame = Frame {
            ip: 0,
            constants: entrypoint.consts,
            code: entrypoint.code,
        };
        Vm { fp: initial_frame, frames, values, heap }
    }

    pub fn pop(&mut self) -> Val {
        self.values.pop().expect("VM value stack was too small")
    }

    pub fn push(&mut self, v: Val) {
        self.values.push(v)
    }

    pub fn take_operand(&mut self) -> u8 {
        let byte = unsafe { (*self.fp.code)[self.fp.ip] };
        self.fp.ip += 1;
        return byte;
    }

    // Returns true if machine has to suddenly halt.
    pub fn step(&mut self) -> bool {
        use OpCode::*;
        let op_code = unsafe { (*self.fp.code)[self.fp.ip] };
        self.fp.ip += 1;
        match crate::bytecode::to_op(op_code) {
            Halt => { return true; },
            Const => {
                let i = self.take_operand();
                let val = unsafe { (*self.fp.constants)[i as usize] };
                self.push(val);
            },
            Pop => { self.pop(); }
            BrNil => {
                let val = self.pop();
                let i = self.take_operand();
                if val == nil() {
                    self.fp.ip = i as usize;
                }
            }
            Add => primitive_math_op!(self, +),
            Sub => primitive_math_op!(self, -),
            Mul => primitive_math_op!(self, *),
            Div => primitive_math_op!(self, /),
            Lt =>  primitive_logic_op!(self, <),
            Gt =>  primitive_logic_op!(self, >),
            Lte =>  primitive_logic_op!(self, <=),
            Gte =>  primitive_logic_op!(self, >=),
            Eq =>  primitive_logic_op!(self, ==),
            _ => unimplemented!()
        }
        return false;
    }

    pub fn run(&mut self) -> Val {
        loop {
            if self.step() {
                // TODO: collect backtrace if debugging enabled
                return self.values.pop().expect("VM halted without a final value")
            }
        }
    }
}