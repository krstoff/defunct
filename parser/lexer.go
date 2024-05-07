package parser

import "io"
import "fmt"
import "errors"
import "bufio"
import "strconv"
import "unicode"
import "strings"

import . "defunct/symbols"

type Runes struct {
	peeked rune
	err    error
	r      io.RuneReader
}

func NewRunes(r io.RuneReader) Runes {
	var runes Runes
	runes.r = r
	return runes
}

func (runes *Runes) PeekRune() (rune, error) {
	if runes.peeked != 0 || runes.err != nil {
		return runes.peeked, runes.err
	}
	c, _, err := runes.r.ReadRune()
	runes.peeked = c
	runes.err = err
	return runes.peeked, runes.err
}

func (runes *Runes) ReadRune() (rune, error) {
	if runes.peeked != 0 || runes.err != nil {
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
	ParsePrefix(*Parser) (Ast, error)
	ParseInfix(Ast, *Parser) (Ast, error)
	Precedence() int
	PPrint(indent int, w io.Writer)
}

type Delimeter int
type Reserved int
type Operator int
type Identifier struct {
	sym Symbol
}
func (i *Identifier) Name() string {
	return i.sym.Name
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
	Do
)

var keywords = map[string]Reserved{
	"defun":  Defun,
	"end":    End,
	"return": Return,
	"let":    Let,
	"if":     If,
	"then":   Then,
	"else":   Else,
	"do": 	  Do,
}

const (
	OpenParen Delimeter = iota
	CloseParen
	OpenBracket
	CloseBracket
	Equals
	Comma
	Semicolon
)

const (
	Add Operator = iota
	Sub
	Mul
	Div
	Dot
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
func (token Operator) Source() (int, int) {
	return 0, 0
}

func IsEof(err error) bool {
	return errors.Is(err, bufio.ErrFinalToken) || errors.Is(err, io.EOF)
}

type Lexer struct {
	runes       Runes
	row, col    int
	st          *SymbolTable
	src         string
	peeked      Token
	err         error
	semicolonOk bool
}

func NewLexer(r io.RuneReader, st *SymbolTable) *Lexer {
	runes := NewRunes(r)
	lexer := new(Lexer)
	lexer.runes = runes
	if st == nil {
		st = NewSymbolTable(nil)
	}
	lexer.st = st
	return lexer
}

func StringLexer(s string) *Lexer {
	r := strings.NewReader(s)
	st := NewSymbolTable(nil)
	lex := NewLexer(r, st)
	lex.src = s
	return lex
}

func (lex *Lexer) PeekRune() (rune, error) {
	r, e := lex.runes.PeekRune()
	return r, e
}

func (lex *Lexer) ReadRune() (rune, error) {
	r, e := lex.runes.ReadRune()
	if e == nil {
		if r == '\n' {
			lex.row += 1
			lex.col = 0
		} else {
			lex.col += 1
		}
	}
	return r, e
}

func (lex *Lexer) PeekToken() (Token, error) {
	if lex.peeked != nil || lex.err != nil {
		return lex.peeked, lex.err
	}
	t, err := lex.readToken()
	lex.peeked = t
	lex.err = err
	return lex.peeked, lex.err
}

func (lex *Lexer) NextToken() (Token, error) {
	if lex.peeked != nil || lex.err != nil {
		peeked := lex.peeked
		lex.peeked = nil
		return peeked, lex.err
	}
	lex.peeked = nil
	lex.err = nil
	var t Token
	t, lex.err = lex.readToken()
	return t, lex.err
}

// / Returns nil, bufio.ErrFinalToken on end of input. or io.Eof. I can't tell.
func (lex *Lexer) readToken() (Token, error) {
	r, err := lex.PeekRune()
	if err != nil {
		goto readErr
	}

	// trim whitespace
	for isWhitespace(r) {
		if r == '\n' && lex.semicolonOk {
			_, err = lex.ReadRune()
			lex.semicolonOk = false
			return Semicolon, nil
		}
		_, err = lex.ReadRune()
		if err != nil {
			goto readErr
		}
		r, err = lex.PeekRune()
	}

	r, err = lex.PeekRune()
	if err != nil {
		goto readErr
	}
	switch {
	case r == ';':
		_, _ = lex.ReadRune()
		return Semicolon, nil
	case r == '(':
		_, _ = lex.ReadRune()
		return OpenParen, nil
	case r == ')':
		_, _ = lex.ReadRune()
		lex.semicolonOk = true
		return CloseParen, nil
	case r == '=':
		_, _ = lex.ReadRune()
		return Equals, nil
	case r == '[':
		_, _ = lex.ReadRune()
		return OpenBracket, nil
	case r == ']':
		lex.semicolonOk = true
		_, _ = lex.ReadRune()
		return CloseBracket, nil
	case r == ',':
		_, _ = lex.ReadRune()
		return Comma, nil
	case r == '+':
		_, _ = lex.ReadRune()
		return Add, nil
	case r == '-':
		_, _ = lex.ReadRune()
		return Sub, nil
	case r == '*':
		_, _ = lex.ReadRune()
		return Mul, nil
	case r == '/':
		_, _ = lex.ReadRune()
		return Div, nil
	case r == '.':
		_, _ = lex.ReadRune()
		return Dot, nil
	case isNumberStartChar(r):
		return lex.lexNumber()
	case isIdentChar(r):
		return lex.lexIdentOrReserved()

	default:
		ivc := InvalidCharacter(r)
		return nil, &ivc
	}

readErr:
	return nil, fmt.Errorf("readToken: %w", err)
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

func (lex *Lexer) lexNumber() (Token, error) {
	lex.semicolonOk = true
	var c rune
	var err error
	digits := ""

	for c, _ = lex.PeekRune(); isIdentChar(c) || c == '.'; c, _ = lex.PeekRune() {
		c, err = lex.ReadRune()
		if err != nil {
			return nil, err
		}
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

func (lex *Lexer) lexIdentOrReserved() (Token, error) {
	var c rune
	var err error
	ident := ""

	for c, _ = lex.PeekRune(); isIdentChar(c); c, _ = lex.PeekRune() {
		c, err = lex.ReadRune()
		if err != nil {
			return nil, err
		}
		ident = ident + string(c)
	}

	if ident == "" {
		panic("Called lexIdentOrReserved on a non-ident string.")
	}

	key, ok := keywords[ident]
	if ok {
		if key == Return {
			lex.semicolonOk = true
		}
		return key, nil
	} else {
		i := Identifier{sym: lex.st.Intern(ident)}
		lex.semicolonOk = true
		return i, nil
	}
}

