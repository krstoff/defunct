package interpreter

import "testing"
// import p "defunct/parser"
import c "defunct/compiler"
import "os"

// func TestExpression(t *testing.T) {
// 	lex := p.StringLexer("(1 * 2) + (3 * 4)")
// 	parser := p.NewParser(lex)
// 	ast, err := parser.Expression()
// 	if err != nil {
// 		t.Error(err.Error())
// 	}
// 	emitter := c.NewEmitter()
// 	ast.Accept(emitter)
// 	bytecode := emitter.Done()

// 	vm := NewVm()
// 	vm.Load(bytecode)
// 	vm.Run()
// 	if vm.Result() != 14.0 {
// 		t.Fail()
// 	}
// }

// func TestLocals(t *testing.T) {
// 	lexer := p.StringLexer(`
// 	do
// 	    let x = 5
// 		let y = 10
// 		2 *(x + y)
// 	end`)
// 	parser := p.NewParser(lexer)
// 	ast, err := parser.Block()
// 	if err != nil {
// 		t.Error(err.Error())
// 	}
// 	emitter := c.NewEmitter()
// 	ast.Accept(emitter)
// 	bytecode := emitter.Done()
	
// 	vm := NewVm()
// 	vm.Load(bytecode)
// 	vm.Run()
// 	// todo
// }

func TestFunctions(t *testing.T) {
	file, err := os.Open("../samples/fib.fun")
	if err != nil {
		t.Errorf("Could not open sample file: %s", err.Error())
		return
	}
	defer file.Close()
	vm := NewVm()
	definitions, err := c.Compile(vm.Globals(), file)
	if err != nil {
		t.Error(err.Error())
	}
	f, ok := definitions["add"]
	if !ok {
		t.Error("File was not successfully compiled.")
		return
	}
	vm.Run(f, 2.0, 3.0)
	if vm.Result() != 320.0 {
		t.Fail() 
	}
}

