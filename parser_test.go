package main

import "testing"
// import "reflect"

func TestParseLiterals(t *testing.T) {
	lex := stringLexer("500")
	parser := NewParser(lex)
	ast, err := parser.expression(0)
	if err != nil {
		t.Error(err.Error()); return
		
	}

	n, ok := ast.(NumLit)
	if !ok || int(n) != 500.0 {
		t.Errorf("Failed to parse number %v.", ast)
	}
}

// func TestBinOperators(t *testing.T) {
// 	lex := stringLexer("1 * 2 + 3 / 4 - 5")
// 	parser := NewParser(lex)
// 	ast, err := parser.expression(0)
// 	if err != nil {
// 		t.Error(err.Error())
// 	}
// 	expected := BinOpCall {
// 		Op: Sub,
// 		Left: BinOpCall {
// 			Op: Add,
// 			Left: BinOpCall {
// 				Op: Mul,
// 				Left: NumLit(1),
// 				Right: NumLit(2),
// 			},
// 			Right: BinOpCall {
// 				Op: Div,
// 				Left: NumLit(3),
// 				Right: NumLit(4),
// 			},
// 		},
// 		Right: NumLit(5),
// 	}

	
// 	if !reflect.DeepEqual(expected, ast) {
// 		t.Errorf("Did not parse the tree that was expected. %#v", ast)
// 	}
// }