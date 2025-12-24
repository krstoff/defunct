use crate::{bytecode::ByteCode, values::Val, bytecode::OpCode::*};

pub fn assemble(text: &str) -> Result<ByteCode, String> {
    let mut code = vec![];
    let mut consts = vec![];

    let lines = text.lines();
    for line in lines {
        if line == "" { continue }
        let words: Vec<_> = line.split_whitespace().collect();
        if words.len() == 0 { continue } // Who knows? Could happen.
        match words[0] {
            "const" => {
                let i = consts.len();
                consts.push(parse_val(words[1])?);
                code.push(Const as u8);
                code.push(i as u8);
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