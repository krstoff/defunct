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
	r io.RuneScanner
}

func NewRunes(r io.RuneScanner) Runes {
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

func (runes *Runes) UnreadRune() {
	runes.peeked = '0'
	runes.err = nil
	runes.r.UnreadRune()
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
type Identifier Symbol
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

var st = NewSymbolTable()

const (
	OpenParen Delimeter = iota
	CloseParen
	Equals
)


// todo
func (r *Reserved) Source() (int, int) {
	return 0, 0
}
func (r *Identifier) Source() (int, int) {
	return 0, 0
}
func (r *StringLit) Source() (int, int) {
	return 0, 0
}
func (r *NumLit) Source() (int, int) {
	return 0, 0
}

// func ReadAllTokens(r io.Reader) []Token {
// 	runes := bufio.NewReader(r)
// 	tokens := make([]Token)
	
// }

// /// Returns nil, bufio.ErrFinalToken on end of input
// func ReadToken(runes RuneReader) Token, error {
// 	r, err := runes.ReadRune()
// 	if err { goto readErr }

// 	// trim whitespace
// 	for isWhitespace(r) {
// 		r, err := runes.ReadRune()
// 		if err { goto readErr }
// 	}

// 	switch {
// 	case r == '(':
// 		return OpenParen, nil
//     case r == ')':
// 		return CloseParen, nil
// 	case r == '=':
// 		return Equals, nil
// 	case isNumberStartChar(c):
// 		runes.UnreadRune(c)
// 		return lexNumber(runes)
// 	case isIdentChar(c):
// 		runes.UnreadRune(c)
// 		return lexIdentOrReserved(runes)

// 	default:
// 		return nil, InvalidCharacter(c)
// 	}

// 	readErr:
// 		return nil, err
// }

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

	for c, _ = runes.PeekRune(); unicode.IsDigit(c); c, _ = runes.PeekRune() {
		c, err = runes.ReadRune()
		if err != nil { return nil, err }
		digits = digits + string(c)
	}

	if digits == "" {
		panic("Called lexNumber on a non-number string.")
	}

	n, _ := strconv.ParseFloat(digits, 64)
	t := NumLit(n)
	return &t, nil
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
		return &key, nil
	} else {
		i := Identifier(st.Intern(ident))
		return &i, nil
	}
}