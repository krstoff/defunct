//! Converts source code into a list representation for macros before lowering to an AST.

use std::str::Chars;
use super::Sexp;
use super::{SymbolTable, Symbol};

#[derive(Debug)]
pub enum ReadError {
    UnexpectedChar(char),
    NumberParseErr(String),
    EOF
}
use ReadError::*;

struct Reader<'src, 'sym> {
    src: &'src str,
    chars: std::iter::Peekable<std::str::CharIndices<'src>>,
    symbols: &'sym mut SymbolTable<'src>,
}

impl<'src, 'sym> Reader<'src, 'sym> {
    pub fn new(src: &'src str, symbols: &'sym mut SymbolTable<'src>) -> Reader<'src, 'sym> {
        Reader {
            src, 
            chars: src.char_indices().peekable(),
            symbols,
        }
    }

    pub fn read(&mut self) -> Result<Sexp, ReadError>  {
        // trim whitespace
        self.trim_whitespace();

        match self.chars.peek().map(|(i, c)| (*i, *c)) {
            None => {
                return Err(EOF)
            }
            Some((_, '(')) => {
                self.read_list()
            }
            Some((i, c)) if is_symbol_start_char(c) => {
                self.read_symbol(i)
            }
            Some((i, c)) if is_number_start_char(c) => {
                self.read_number(i)
            }
            Some((_, c)) => {
                return Err(UnexpectedChar(c))
            }
        }
    }

    fn read_list(&mut self) -> Result<Sexp, ReadError> {
        self.chars.next(); // trim '('
        self.trim_whitespace();

        let mut items = Vec::new();
        while let Some((i, c)) = self.chars.peek() && *c != ')' {
            items.push(self.read()?);
            self.trim_whitespace();
        }

        // trim ')'
        if let None = self.chars.next() {
            return Err(ReadError::EOF)
        }

        Ok(Sexp::List(items))
    }

    fn read_symbol(&mut self, start: usize) -> Result<Sexp, ReadError> {
        let mut last_index = 0;
        while let Some((i, c)) = self.chars.peek() {
            if !is_symbol_char(*c) {
                break;
            }
            last_index = *i;
            self.chars.next();
        }
        let chars = &self.src[start..last_index + 1];
        Ok(Sexp::Symbol(self.symbols.intern(chars)))
    }

    fn read_number(&mut self, start: usize) -> Result<Sexp, ReadError> {
        let mut last_index = 0;
        while let Some((i, c)) = self.chars.peek() {
            if !is_number_char(*c) {
                break;
            }
            last_index = *i;
            self.chars.next();
        }
        let digits = &self.src[start..last_index + 1];
        match digits.parse::<f64>() {
            Ok(num) => Ok(Sexp::Number(num)),
            Err(_) => Err(ReadError::NumberParseErr(digits.to_string()))
        }
    }

    fn trim_whitespace(&mut self) {
        while let Some((i, c)) = self.chars.peek() && is_whitespace(*c) {
            self.chars.next();
        }
    }
}

const SYMBOL_CHARS: &'static str = "+-*/:_!";

fn is_symbol_start_char(c: char) -> bool {
    c.is_alphanumeric() || SYMBOL_CHARS.contains(c)
}

fn is_number_start_char(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_number_char(c: char) -> bool {
    c.is_ascii_digit() || c == '.' || c == '_'
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric() || SYMBOL_CHARS.contains(c)
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace() || c == ',' // This is for ease of reading map literals
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::sexp::print_sexp;
    #[test]
    fn sexp() {
        let mut symbols = super::SymbolTable::new();
        let list_str = r"
        ((defun (add3 x y z)
          (let ((*sum* (+ x y z))
            sum))))
        ";
        let sexp = {
            let mut reader = Reader::new(list_str, &mut symbols);
            reader.read()
        }.unwrap();
        print_sexp(&sexp, &symbols);
    }
}