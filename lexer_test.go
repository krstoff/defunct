package main

import "testing"
import "reflect"
import "strings"

func  TestRuneScanner(t *testing.T) {
    r := NewRunes(strings.NewReader("abcd"))
	c, _ := r.PeekRune()
	if c != 'a' {
		t.Fail()
	}
	c, _ = r.ReadRune()
	c, _ = r.ReadRune()
	c, _ = r.ReadRune()
	c, _ = r.ReadRune()
	if c != 'd' {
		t.Errorf("Expected 'd', got %c", c)
	}

	_, err := r.ReadRune()
	if err == nil {
		t.Errorf("Expected err at end of input")
	}
}

func TestLexNumber(t *testing.T) {
	getNumber := func(tok Token) float64 {
		n, ok := tok.(NumLit)
		if !ok {
			t.Errorf("Tried to parse a number but did not receive a NumLit token")
		}
		return float64(n)
	}
    r := NewRunes(strings.NewReader("124.52"))
	tok, _ := lexNumber(r)

	n := getNumber(tok)
	if n != 124.52 {
		t.Errorf("Tried to parse 124.52, found %f instead", n)
	}

	r = NewRunes(strings.NewReader("754furb"))
	tok, err := lexNumber(r)
	if err == nil {
		t.Error("Tried to parse 754furb, did not error")
	}
}

func TestReadToken(t *testing.T) {
	var err error
	runes := func(s string) Runes {
		return NewRunes(strings.NewReader(s))
	}
	numbersString := runes("1 ( 2.52 (=) fifty defun 312 4")
	expected := []Token {
		NumLit(1.0),
		Delimeter(OpenParen),
		NumLit(2.52),
		Delimeter(OpenParen), Delimeter(Equals), Delimeter(CloseParen),
		Identifier{ sym: st.Intern("fifty")}, Reserved(Defun),
		NumLit(312.0), NumLit(4),
	}
	tokens := []Token {}
	for err == nil {
		var tok Token
		tok, err = ReadToken(numbersString)
		if err != nil {
			if err.Error() != "EOF" {
				t.Error(err.Error())
			}
			break
		}

		tokens = append(tokens, tok)
	}

	if !reflect.DeepEqual(expected, tokens) {
		t.Errorf("Did not lex the tokens that were expected. %#v", tokens)
	}
}