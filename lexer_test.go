package main

import "testing"
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
		n, ok := tok.(*NumLit)
		if !ok {
			t.Errorf("Tried to parse a number but did not receive a NumLit token")
		}
		return float64(*n)
	}
    r := NewRunes(strings.NewReader("124"))
	tok, _ := lexNumber(r)

	n := getNumber(tok)
	if n != 124.0 {
		t.Errorf("Tried to parse 124, found %f instead", n)
	}

	r = NewRunes(strings.NewReader("754f"))
	tok, _ = lexNumber(r)
	n = getNumber(tok)
	if n != 754.0 {
		t.Errorf("Tried to parse 754, found %f instead", n)
	}
}