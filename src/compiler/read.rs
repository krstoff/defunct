//! Converts source code into a list representation for macros before lowering to an AST.

use std::str::Chars;
use super::Sexp;
use super::{IdentTable, Ident};

#[derive(Debug, PartialEq, Eq)]
enum ReadErrorReason {
    UnexpectedChar(char),
    NumberParseErr(String),
    UnbalancedBracket,
    UnbalancedParen,
    UnbalancedBrace,
    UnbalancedMapItems,
    BareColon,
    EOF
}
use ReadErrorReason::*;

pub struct ReadError {
    line: usize,
    col: usize,
    reason: ReadErrorReason,
}

impl std::fmt::Debug for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} - ", self.line, self.col)?;
        match self.reason {
            UnexpectedChar(c) => {
                write!(f, "Did not expect character {}", c)
            },
            NumberParseErr(ref digits) => {
                write!(f, "{} is not a valid number", digits)
            },
            UnbalancedBracket => {
                write!(f, "Encountered unexpected ']' character; count your brackets")
            }
            UnbalancedParen => {
                write!(f, "Encountered unexpected ')' character; count your parens")
            }
            UnbalancedBrace => {
                write!(f, "Encountered unexpected '}}' character; count your braces")
            }
            UnbalancedMapItems => {
                write!(f, "Unexpected end of map literal; items were unbalanced")
            }
            EOF => {
                write!(f, "Unexpected end of file")
            }
            BareColon => {
                write!(f, "Invalid symbol name: ':' ")
            }
        }
    }
}

pub struct Reader<'src, 'sym> {
    src: &'src str,
    line: usize,
    col: usize,
    chars: std::iter::Peekable<std::str::CharIndices<'src>>,
    idents: &'sym mut IdentTable<'src>,
}

impl<'src, 'sym> Reader<'src, 'sym> {
    pub fn new(src: &'src str, idents: &'sym mut IdentTable<'src>) -> Reader<'src, 'sym> {
        Reader {
            src,
            line: 0,
            col: 0, 
            chars: src.char_indices().peekable(),
            idents,
        }
    }

    fn error(&self, reason: ReadErrorReason) -> ReadError {
        let (line, col) = self.location();
        ReadError {
            line, col, reason 
        }
    }

    fn peek(&mut self) -> Option<&(usize, char)> {
        self.chars.peek()
    }

    fn next(&mut self) -> Option<(usize, char)> {
        let peeked = self.chars.peek();
        match peeked {
            Some(&(_, '\n')) => {
                self.line += 1;
                self.col = 0;
            },
            Some(_) => {
                self.col += 1;
            }
            _ => {},
        }
        self.chars.next()
    }

    /// Returns line, column no. in src code.
    fn location(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    pub fn read(&mut self) -> Result<Sexp, ReadError>  {
        // trim whitespace
        self.trim_whitespace();

        match self.chars.peek().map(|(i, c)| (*i, *c)) {
            None => {
                return Err(self.error(EOF))
            }
            Some((_, '(')) => {
                self.read_list()
            }
            Some((_, '[')) => {
                self.read_vector()
            }
            Some((_, '{')) => {
                self.read_map()
            }
            Some((i, ':')) => {
                self.read_keyword(i)
            }
            Some((i, c)) if is_number_start_char(c) => {
                self.read_number(i)
            }
            Some((i, c)) if is_symbol_start_char(c) => {
                self.read_symbol(i)
            }
            Some((_, c)) => {
                return Err(self.error(UnexpectedChar(c)))
            }
        }
    }

    fn read_list(&mut self) -> Result<Sexp, ReadError> {
        self.chars.next(); // trim '('
        self.trim_whitespace();

        let mut items = Vec::new();
        while let Some((i, c)) = self.chars.peek() && *c != ')' {
            if *c == ']' {
                return Err(self.error(UnbalancedBracket))
            }
            if *c == '}' {
                return Err(self.error(UnbalancedBrace))
            }
            items.push(self.read()?);
            self.trim_whitespace();
        }

        // trim ')'
        if let None = self.chars.next() {
            return Err(self.error(EOF))
        }

        Ok(Sexp::List(items))
    }

    fn read_vector(&mut self) -> Result<Sexp, ReadError> {
        self.chars.next(); // trim '['
        self.trim_whitespace();

        let mut items = Vec::new();
        while let Some((i, c)) = self.chars.peek() && *c != ']' {
            if *c == ')' {
                return Err(self.error(UnbalancedParen))
            }
            if *c == '}' {
                return Err(self.error(UnbalancedBrace))
            }
            items.push(self.read()?);
            self.trim_whitespace();
        }

        // trim ']'
        if let None = self.chars.next() {
            return Err(self.error(EOF))
        }

        Ok(Sexp::Vector(items))
    }

    fn read_map(&mut self) -> Result<Sexp, ReadError> {
        self.chars.next(); // trim '['
        self.trim_whitespace();

        let mut items = Vec::new();
        while let Some((i, c)) = self.chars.peek() && *c != '}' {
            if *c == ')' {
                return Err(self.error(UnbalancedParen))
            }
            if *c == ']' {
                return Err(self.error(UnbalancedBracket))
            }
            let key = self.read()?;
            let value = match self.read() {
                Ok(value) => value,
                Err(e) if e.reason == UnexpectedChar('}') => {
                    return Err(self.error(UnbalancedMapItems))
                }
                Err(e) => {
                    return Err(e)
                }
            };
            items.push((key, value));
            self.trim_whitespace();
        }

        // trim '}'
        if let None = self.chars.next() {
            return Err(self.error(EOF))
        }

        Ok(Sexp::Map(items))
    }

    fn read_keyword(&mut self, start: usize) -> Result<Sexp, ReadError> {
        self.chars.next(); // trim ':'
        if let None = self.chars.peek() {
            return Err(self.error(EOF))
        }
        let symbol = self.read_symbol(start + 1)?;
        if let(Sexp::Ident(ident)) = symbol { Ok(Sexp::Keyword(ident)) }
        else { panic!("read_symbol returned something else. this should not happen") }
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
        if last_index == 0 {
            return Err(self.error(BareColon))
        }
        let chars = &self.src[start..last_index + 1];
        Ok(Sexp::Ident(self.idents.intern(chars)))
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
            Err(_) => Err(self.error(NumberParseErr(digits.to_string())))
        }
    }

    fn trim_whitespace(&mut self) {
        while let Some((i, c)) = self.chars.peek() && is_whitespace(*c) {
            self.chars.next();
        }
    }
}

const SYMBOL_CHARS: &'static str = "+-*/_!<>=";

fn is_symbol_start_char(c: char) -> bool {
    c.is_alphanumeric() || SYMBOL_CHARS.contains(c)
}

fn is_number_start_char(c: char) -> bool {
    c.is_ascii_digit() || c == '.' || c == '-'
}

fn is_number_char(c: char) -> bool {
    c.is_ascii_digit() || c == '.' || c == '_' || c == '-'
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric() || SYMBOL_CHARS.contains(c)
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace() || c == ',' // This is for ease of reading map literals
}