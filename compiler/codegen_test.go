package compiler

import "testing"
import p "defunct/parser"

func TestOpCodes(t *testing.T) {
	lex := p.StringLexer("(1 * 2) + (3 * 4)")
	parser := p.NewParser(lex)
	ast, err := parser.Expression()
	if err != nil {
		t.Error(err.Error())
	}
	emitter := NewEmitter()
	ast.Accept(emitter)
	str := Disassemble(emitter.Bytes, emitter.Constants)
	expected := 
`const 1
const 2
mul
const 3
const 4
mul
add
`
	if expected != str {
		t.Errorf("%s", str)
	}
}