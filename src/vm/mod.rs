use crate::alloc::Heap;
use crate::global::Global;
use crate::values::{Map, Tag, Val, Vector, Symbol};
use crate::bytecode::{OpCode, to_op};

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
        let right = $self.pop();
        let left = $self.pop();

        if left.is_int() && right.is_int() {
            let result = left.get_int().unwrap() $bin_op right.get_int().unwrap();
            
            $self.push(if result { Symbol::t() } else { Symbol::nil() });
        } 
                
        else if left.is_num() && right.is_num() {
            let result = left.get_num().unwrap() $bin_op right.get_num().unwrap();
            $self.push(if result { Symbol::t() } else { Symbol::nil() });
        }
                
        else if left.is_ptr() || right.is_ptr() {
            // TODO: TypeErr
            unimplemented!();
        }

        else {
            let result =
                left.get_int().map(|i| i as f64).or(left.get_num()).unwrap()
                $bin_op right.get_int().map(|i| i as f64).or(right.get_num()).unwrap();
            $self.push(if result { Symbol::t() } else { Symbol::nil() });
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
        use crate::values::Cases;
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
            Pop => {
                let count = self.take_operand();
                for i in 0..count {
                    self.pop();
                }
            }
            PopSave => {
                let count = self.take_operand();
                let val = self.pop();
                for i in 0..count {
                    self.pop();
                }
                self.push(val);
            }
            Dup => {
                let i = self.take_operand();
                self.push(self.values[self.fp.base + i as usize]);
            }
            BrNil => {
                let val = self.pop();
                let i = self.take_operand();
                if val == Symbol::nil() {
                    self.fp.ip = i as usize;
                }
            }
            Jmp => {
                let i = self.take_operand();
                self.fp.ip += i as usize;
            }
            Call => {
                let n = self.take_operand();
                let f = self.pop();
                match f.get() {
                    Cases::Function(ptr) => {
                        self.frames.push(self.fp);
                        unsafe {
                            self.fp.code = (*(*ptr).code_obj).code;
                            self.fp.constants = (*(*ptr).code_obj).consts;
                            self.fp.env = (*ptr).env;
                        }
                        self.fp.ip = 0;
                        self.fp.base = self.values.len() - n as usize;
                    }
                    Cases::NativeFn(native_fn) => {
                        assert!(self.values.len() >= n as usize);
                        let crate::values::NativeFn(f) = native_fn;
                        let (begin, end) = (self.values.len() - n as usize, self.values.len());
                        assert!(begin >= self.fp.base);
                        let (result, should_halt) = f(&self.values[begin..end], self.global);
                        if should_halt {
                            return true;
                        }
                        for _ in 0..n {
                            self.pop();
                        }
                        self.push(result);
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
                    self.push(Symbol::t());
                } else {
                    self.push(Symbol::nil());
                }
            }
            MapGet => {
                let key = self.pop();
                let map = self.pop();
                match map.get() {
                    Cases::Map(m) => {
                        self.push(m.get(key))
                    }
                    _ => {
                        // TODO: TypeError
                        unimplemented!()
                    }
                }
            }
            MapSet => {
                let val = self.pop();
                let key = self.pop();
                let map = self.pop();
                match map.get() {
                    Cases::Map(m) => {
                        m.insert(key, val);
                    }
                    _ => {
                        // TODO: TypeError
                        unimplemented!()
                    }
                }
            }
            MapNew => {
                let mut ptr = Heap::alloc(size_of::<Map>()) as *mut _;
                unsafe { std::ptr::write(ptr, Map::new()); }
                self.push(Val::from_ptr(Tag::Map, ptr as *mut u8));
            }
            MapDel => {
                let key = self.pop();
                let map = self.pop();
                match map.get() {
                    Cases::Map(m) => {
                        self.push(m.remove(key))
                    }
                    _ => {
                        // TODO: TypeError
                        unimplemented!()
                    }
                }
            }
            VecNew => {
                let mut ptr = Heap::alloc(size_of::<Vector>()) as *mut _;
                unsafe { std::ptr::write(ptr, Vector::new()); }
                self.push(Val::from_ptr(Tag::Vector, ptr as *mut u8));
            }
            VecGet => {
                let index = self.pop();
                let vec = self.pop();
                match (vec.get(), index.get()) {
                    (Cases::Vector(ptr), Cases::Int(i))  if i >= 0 => {
                        let v = unsafe {&mut *ptr};
                        self.push(v.get(i as usize));
                    }
                    _ => {
                        // TODO: TypeError
                        unimplemented!()
                    }
                }
            }
            VecSet => {
                let value = self.pop();
                let index = self.pop();
                let vec = self.pop();
                match (vec.get(), index.get()) {
                    (Cases::Vector(v), Cases::Int(i))  if i >= 0 => {
                        v.set(i as usize, value);
                    }
                    _ => {
                        // TODO: TypeError
                        unimplemented!()
                    }
                }
            }
            VecPush => {
                let value = self.pop();
                let vec = self.pop();
                match vec.get() {
                    Cases::Vector(v) => {
                        v.push(value);                      
                    }
                    _ => {
                        // TODO: TypeError
                        unimplemented!()
                    }
                }
            }
            VecPop => {
                let vec = self.pop();
                match vec.get() {
                    Cases::Vector(v) => {
                        self.push(v.pop());                       
                    }
                    _ => {
                        // TODO: TypeError
                        unimplemented!()
                    }
                }
            }
            SymGet => {
                let val = self.pop();
                match val.get() {
                    Cases::Symbol(sym) => {
                        match sym.val() {
                            Some(symbol_value) => {
                                self.push(symbol_value)
                            }
                            _ => {
                                // TODO: TypeError
                                unimplemented!()
                            }
                        }
                    }
                    _ => {
                        // TODO: TypeError
                        unimplemented!()
                    }
                }
            }
            SymSet => {
                let val = self.pop();
                let sym = self.pop();
                match sym.get() {
                    Cases::Symbol(mut sym) => {
                        sym.set(val);
                    }
                    _ => {
                        // TODO: TypeError
                        unimplemented!()
                    }
                }
            }
            Closure => {
                let i = self.take_operand();
                let ptr = unsafe { (*self.fp.constants)[i as usize] };
                match ptr.get() {
                    Cases::Object(obj) => {
                        let closure = crate::Closure::new(&[], obj as *const _);
                        self.push(closure)
                    }
                    // TODO: TypeError
                    _ => unimplemented!()
                }
            }
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
        let op = unsafe { to_op((*self.fp.code)[self.fp.ip]) };
        print!("{}", op.to_str());
        if op.has_param() {
            print!(" #{}", unsafe { (*self.fp.code)[self.fp.ip + 1] })
        }
        
        println!("\t{:?}", &self.values[..])
    }
}