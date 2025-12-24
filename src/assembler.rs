use crate::{bytecode::ByteCode, values::Val, bytecode::OpCode::*};

pub fn compile(text: &str) -> Result<ByteCode, String> {
    use std::collections::HashMap;
    let mut code = vec![];
    let mut consts = vec![];
    let mut symbols = crate::symbols::SymbolTable::new();
    let mut labels = HashMap::<_, usize>::new();
    let mut refs = vec![];

    let lines = text.lines();
    for line in lines {
        if line == "" { continue }
        let words: Vec<_> = line.split_whitespace().collect();
        if words.len() == 0 { continue } // Who knows? Could happen.

        if words[0].as_bytes()[0] == ('.' as u8) {
            let sym = symbols.intern(words[0]);
            labels.insert(sym, code.len());
            continue;
        }
        match words[0] {
            "const" => {
                let i = consts.len();
                consts.push(parse_val(words[1])?);
                code.push(Const as u8);
                code.push(i as u8);
            }
            "brnil" => {
                code.push(BrNil as u8);
                // Push a 0 into the code stream for now. Patch refs later.
                let sym = symbols.intern(words[1]);
                refs.push((code.len() as u8, sym));
                code.push(0);
            }
            "dup" => {
                code.push(Dup as u8);
                code.push(parse_immediate(words[1])? as u8);
            }
            "ret" => {
                code.push(Ret as u8);
                code.push(parse_immediate(words[1])? as u8);
            }
            "call" => {
                code.push(Call as u8);
                code.push(parse_immediate(words[1])? as u8);
            }
            "add" => {
                code.push(Add as u8);
            }
            "sub" => {
                code.push(Sub as u8);
            }
            "mul" => {
                code.push(Mul as u8);
            }
            "div" => {
                code.push(Div as u8);
            }
            "gt" => {
                code.push(Gt as u8);
            }
            "lt" => {
                code.push(Lt as u8);
            }
            "gte" => {
                code.push(Gte as u8);
            }
            "lte" => {
                code.push(Lte as u8);
            }
            "eq" => {
                code.push(Eq as u8);
            }
            "halt" => {
                code.push(Halt as u8)
            }
            _ => return Err(line.to_string())
        }
        if consts.len() > 255 {
            return Err("Too many constants in assembled code.".to_string());
        }
        // TODO: need to be able to jmp longer distances!
        if code.len() > 255 {
            return Err("Assembled code contains too many instructions.".to_string());
        }
    }

    // patch up refs
    for (i, label) in refs {
        if !labels.contains_key(&label) {
            return Err("Referenced a label that does not exist.".to_string())
        }
        let dest = labels[&label];
        assert!(dest < code.len());
        code[i as usize] = dest as u8;
    }

    let code = code.into_boxed_slice();
    let consts = consts.into_boxed_slice();

    let bytecode =  ByteCode {
        code: &*code as *const [u8],
        consts: &*consts as *const [Val],
    };

    unsafe {
        std::mem::forget(code);
        std::mem::forget(consts);
    }
    
    Ok(bytecode)
}

fn parse_val(s: &str) -> Result<Val, String> {
    let mut chars = s.chars().peekable();
    let first = chars.peek().unwrap();
    // Expects the next value to be a machine word. That includes tag bits!
    if *first == '%' {
        if let Ok(raw) = s[1..].parse::<usize>() {
            let val = unsafe { std::mem::transmute(raw)};
            return Ok(val)
        }
    }
    if *first == '-' || first.is_ascii_digit() {
        if let Ok(i) = s.parse::<u32>() {
            return Ok(Val::from_int(i));
        }
        else if let Ok(f) = s.parse::<f64>() {
            return Ok(Val::from_num(f));
        }
    }
    let err_string = "not a valid constant: ".to_string() + s;
    Err(err_string)
}

fn parse_immediate(s: &str) -> Result<u8, String> {
    let mut chars = s.chars().peekable();
    let first = chars.peek().unwrap();
    if *first == '#' {
        if let Ok(i) = s[1..].parse::<u8>() {
            return Ok(i);
        }
    }
    let err_string = "not a valid immediate: ".to_string() + s;
    Err(err_string)
}