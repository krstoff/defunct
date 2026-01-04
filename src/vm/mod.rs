mod boolean;

use crate::global::Global;
use crate::values::Val;
use crate::bytecode::{OpCode, to_op};
use crate::vm::boolean::nil;

#[derive(Copy, Clone)]
struct Frame {
    ip: usize,
    base: usize,
    constants: *const [Val],
    code: *const [u8],
    env: *const [Val],
}

pub struct Vm<'a> {
    debug: bool,
    global: &'a mut Global,
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
    pub fn new(global: &'a mut Global, entrypoint: crate::bytecode::ByteCode, initargs: &[Val], debug: bool) -> Vm <'a> {
        let mut values = vec![];
        for val in initargs {
            values.push(*val);
        }
        let frames = vec![];
        let initial_frame = Frame {
            ip: 0,
            base: 0,
            constants: entrypoint.consts,
            code: entrypoint.code,
            env: &[],
        };
        Vm { debug, fp: initial_frame, frames, values, global }
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
            Dup => {
                let i = self.take_operand();
                self.push(self.values[self.fp.base + i as usize]);
            }
            BrNil => {
                let val = self.pop();
                let i = self.take_operand();
                if val == nil() {
                    self.fp.ip = i as usize;
                }
            }
            Call => {
                use crate::values::Cases::*;
                let n = self.take_operand();
                let f = self.pop();
                match f.get() {
                    Function(ptr) => {
                        self.frames.push(self.fp);
                        unsafe {
                            self.fp.code = (*(*ptr).code_obj).code;
                            self.fp.constants = (*(*ptr).code_obj).consts;
                            self.fp.env = (*ptr).env;
                        }
                        self.fp.ip = 0;
                        self.fp.base = self.values.len() - n as usize;
                    }
                    _ => {
                        // TODO: TypeError
                        unimplemented!()
                    }
                }
                
            }
            Ret => {
                assert!(self.frames.len() > 0);
                let n = self.take_operand();
                let val = self.values[self.fp.base + n as usize];
                while self.values.len() > self.fp.base {
                    self.pop();
                }
                self.push(val);
                let frame = self.frames.pop().unwrap();
                self.fp = frame;
            }
            Add => primitive_math_op!(self, +),
            Sub => primitive_math_op!(self, -),
            Mul => primitive_math_op!(self, *),
            Div => primitive_math_op!(self, /),
            Lt =>  primitive_logic_op!(self, <),
            Gt =>  primitive_logic_op!(self, >),
            Lte =>  primitive_logic_op!(self, <=),
            Gte =>  primitive_logic_op!(self, >=),
            Eq =>  {
                let right = self.pop();
                let left = self.pop();
                if left == right {
                    self.push(boolean::t());
                } else {
                    self.push(nil());
                }
            }
            _ => unimplemented!()
        }
        return false;
    }

    pub fn run(&mut self) -> Val {
        loop {
            if self.debug {
                self.print_state();
            }
            if self.step() {
                // TODO: collect backtrace if debugging enabled
                return self.values.pop().expect("VM halted without a final value")
            }
        }
    }

    pub fn print_state(&self) {
        use crate::bytecode::OpCode::*;
        match unsafe { to_op((*self.fp.code)[self.fp.ip]) } {
            Const => print!("const {}", unsafe{(*self.fp.code)[self.fp.ip + 1]}),
            Dup => print!("dup {}", unsafe{(*self.fp.code)[self.fp.ip + 1]}),
            Pop => print!("pop"),

            Add => print!("add"),
            Sub => print!("sub"),
            Mul => print!("mul"),
            Div => print!("div"),
            Lt => print!("lt"),
            Gt => print!("gt"),    
            Lte => print!("lte"), 
            Gte => print!("gte"),
            Eq => print!("eq"),

            BrNil => print!("brnil {}", unsafe{(*self.fp.code)[self.fp.ip + 1]}),
            Call => print!("call {}", unsafe{(*self.fp.code)[self.fp.ip + 1]}),
            Ret => print!("ret {}", unsafe{(*self.fp.code)[self.fp.ip + 1]}),
            Halt => print!("halt"),
        };
        
        println!("\t{:?}", &self.values[..])
    }
}