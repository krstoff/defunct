package main

import "io"
import "fmt"
// import "bufio"
// import "errors"
import "strconv"
import "unicode"

type Runes struct {
	peeked rune
	err error
	r io.RuneReader
}

func NewRunes(r io.RuneReader) Runes {
	var runes Runes
	runes.r = r
	return runes
}

func (runes *Runes) PeekRune() (rune, error) {
	if runes.peeked != 0  || runes.err != nil{
		return runes.peeked, runes.err
	}
	c, _, err := runes.r.ReadRune()
	runes.peeked = c
	runes.err = err
	return runes.peeked, runes.err
}

func (runes *Runes) ReadRune() (rune, error) {
	if runes.peeked !=0 || runes.err != nil {
		peeked := runes.peeked
		runes.peeked = 0
		return peeked, runes.err
	}
	runes.peeked = 0
	runes.err = nil
	var c rune
	c, _, runes.err = runes.r.ReadRune()
	return c, runes.err
}

type InvalidCharacter rune
func (i *InvalidCharacter) Error() string {
	return fmt.Sprintf("Invalid character detected: %U", rune(*i))
}

type Token interface {
	Source() (int, int) // row, col
}

type Delimeter int
type Reserved int
type Identifier struct {
	sym Symbol
}
type StringLit string
type NumLit float64

const (
	Defun Reserved = iota
	End
	Return
	Let
	If
	Then
	Else
)

var keywords =  map[string]Reserved {
	"defun": Defun,
	"end": End,
	"return": Return,
	"let": Let,
	"if": If,
	"then": Then,
	"else": Else,
}

var st = NewSymbolTable(nil)

const (
	OpenParen Delimeter = iota
	CloseParen
	Equals
)


// todo
func (token Reserved) Source() (int, int) {
	return 0, 0
}
func (token Delimeter) Source() (int, int) {
	return 0, 0
}
func (token Identifier) Source() (int, int) {
	return 0, 0
}
func (token StringLit) Source() (int, int) {
	return 0, 0
}
func (token NumLit) Source() (int, int) {
	return 0, 0
}

type Lexer struct {
	runes Runes
	row, col int
	st *SymbolTable
}

// func NewLexer(runes Runes, st *SymbolTable) *Lexer {
// 	lexer := new(Lexer)
// 	lexer.runes = runes
// 	lexer.st = st
// 	return lexer
// }

// func (lex *Lexer) PeekRune() (rune, error) {
// 	r, e := lex.runes.PeekRune()
// 	return r, e
// }

// func (lex *Lexer) ReadRune() (rune, error) {
// 	r, e := lex.runes.ReadRune()
// 	if e == nil {
// 		if r == '\n' {
// 			lex.row += 1
// 			lex.col = 0
// 		} else {
// 			lex.col += 1
// 		}
// 	}
// 	return r, e
// }

// func (lex *Lexer) UnreadRune() {
// 	lex.runes.UnreadRune()
// 	r, e := lex.runes.PeekRune()
// 	if r == '\n' {
// 		lex.row -= 1
// 		lex.col = 0
// 	} else {
// 		lex.col -= 1
// 	}
// }

// func ReadAllTokens(r io.Reader) []Token {
// 	runes := bufio.NewReader(r)
// 	tokens := make([]Token)
	
// }

/// Returns nil, bufio.ErrFinalToken on end of input
func ReadToken(runes Runes) (Token, error) {
	r, err := runes.PeekRune()
	if err != nil { goto readErr }

	// trim whitespace
	for isWhitespace(r) {
		_, err = runes.ReadRune()
		if err != nil { goto readErr }
		r, err = runes.PeekRune()
	}

	r, err = runes.PeekRune()
	switch {
	case r == '(':
		_, _ = runes.ReadRune()
		return OpenParen, nil
    case r == ')':
		_, _ = runes.ReadRune()
		return CloseParen, nil
	case r == '=':
		_, _ = runes.ReadRune()
		return Equals, nil
	case isNumberStartChar(r):
		return lexNumber(runes)
	case isIdentChar(r):
		return lexIdentOrReserved(runes)

	default:
		ivc := InvalidCharacter(r)
		return nil, &ivc
	}

	readErr:
		return nil, err
}

func isWhitespace(r rune) bool {
	return unicode.IsSpace(r)
}

func isNumberStartChar(c rune) bool {
	return unicode.IsDigit(c)	
}

func isIdentChar(c rune) bool {
	return unicode.IsLetter(c) || unicode.IsDigit(c) || c == '_'
}

func lexNumber(runes Runes) (Token, error) {
	var c rune
	var err error
	digits := ""

	for c, _ = runes.PeekRune(); isIdentChar(c) || c == '.'; c, _ = runes.PeekRune() {
		c, err = runes.ReadRune()
		if err != nil { return nil, err }
		digits = digits + string(c)
	}

	if digits == "" {
		panic("Called lexNumber on a non-number string.")
	}

	n, err := strconv.ParseFloat(digits, 64)
	if err != nil {
		err = fmt.Errorf("lexNumber: invalid number literal %s", digits)
		return nil, err
	}
	t := NumLit(n)
	return t, nil
}


func lexIdentOrReserved(runes Runes) (Token, error) {
	var c rune
	var err error
	ident := ""

	for c, _ = runes.PeekRune(); isIdentChar(c); c, _ = runes.PeekRune() {
		c, err = runes.ReadRune()
		if err != nil { return nil, err }
		ident = ident + string(c)
	}

	if ident == "" {
		panic("Called lexIdentOrReserved on a non-ident string.")
	}

	key, ok := keywords[ident]
	if ok {
		return key, nil
	} else {
		i := Identifier { sym: st.Intern(ident) }
		return i, nil
	}
}