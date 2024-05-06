package interpreter

import "testing"
import p "defunct/parser"
import c "defunct/compiler"

func TestVm(t *testing.T) {
	lex := p.StringLexer("(1 * 2) + (3 * 4)")
	parser := p.NewParser(lex)
	ast, err := parser.Expression()
	if err != nil {
		t.Error(err.Error())
	}
	emitter := c.NewEmitter()
	ast.Accept(emitter)
	bytecode := emitter.Done()

	vm := NewVm()
	vm.Load(bytecode)
	vm.Run()
	if vm.Result() != 14.0 {
		t.Fail()
	}
}

