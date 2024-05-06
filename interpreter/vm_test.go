package interpreter

import "testing"
import p "defunct/parser"
import c "defunct/compiler"

func TestExpression(t *testing.T) {
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

func TestLocals(t *testing.T) {
	lexer := p.StringLexer(`
	do
	    let x = 5
		let y = 10
		x + y
	end`)
	parser := p.NewParser(lexer)
	ast, err := parser.Block()
	if err != nil {
		t.Error(err.Error())
	}
	emitter := c.NewEmitter()
	ast.Accept(emitter)
	bytecode := emitter.Done()
	
	vm := NewVm()
	vm.Load(bytecode)
	vm.Run()
	if vm.valueStack[2] != 15.0 {
		t.Fail()
	}
}

