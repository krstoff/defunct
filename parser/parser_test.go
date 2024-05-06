package parser

import "testing"
import "reflect"
import "os"
import "strings"
import "bufio"

func TestParseLiterals(t *testing.T) {
	lex := StringLexer("500")
	parser := NewParser(lex)
	ast, err := parser.Expression()
	if err != nil {
		t.Error(err.Error())
		return

	}

	n, ok := ast.(NumLit)
	if !ok || int(n) != 500.0 {
		t.Errorf("Failed to parse number %v.", ast)
	}
}

func TestBinOperators(t *testing.T) {
	lex := StringLexer("1 * 2 + 3 / 4 - 5")
	parser := NewParser(lex)
	ast, err := parser.Expression()
	if err != nil {
		t.Error(err.Error())
	}
	expected := BinOpCall{
		Op: Sub,
		Left: BinOpCall{
			Op: Add,
			Left: BinOpCall{
				Op:    Mul,
				Left:  NumLit(1),
				Right: NumLit(2),
			},
			Right: BinOpCall{
				Op:    Div,
				Left:  NumLit(3),
				Right: NumLit(4),
			},
		},
		Right: NumLit(5),
	}

	if !reflect.DeepEqual(expected, ast) {
		t.Errorf("Did not parse the tree that was expected. %#v", ast)
	}
}

func TestFunctionCalls(t *testing.T) {
	lex := StringLexer("halt() free(address) list(2, 3, 4, 5)")
	parser := NewParser(lex)
	ast1, err1 := parser.Expression()
	ast2, err2 := parser.Expression()
	ast3, err3 := parser.Expression()

	if err1 != nil {
		t.Errorf("Failed to parse function call. %s", err1.Error())
		return
	}
	if err2 != nil {
		t.Errorf("Failed to parse function call. %s", err2.Error())
		return
	}
	if err3 != nil {
		t.Errorf("Failed to parse function call. %s", err3.Error())
		return
	}

	expected1 := FunCall{
		Name: Identifier{sym: lex.st.Intern("halt")},
		Args: []Ast{},
	}

	expected2 := FunCall{
		Name: Identifier{sym: lex.st.Intern("free")},
		Args: []Ast{Identifier{sym: lex.st.Intern("address")}},
	}

	expected3 := FunCall{
		Name: Identifier{sym: lex.st.Intern("list")},
		Args: []Ast{NumLit(2), NumLit(3), NumLit(4), NumLit(5)},
	}

	if !reflect.DeepEqual(expected1, ast1) {
		t.Errorf("Did not parse the tree that was expected. %#v", ast1)
	}
	if !reflect.DeepEqual(expected2, ast2) {
		t.Errorf("Did not parse the tree that was expected. %#v", ast2)
	}
	if !reflect.DeepEqual(expected3, ast3) {
		t.Errorf("Did not parse the tree that was expected. %#v", ast3)
	}
}

func TestParens(t *testing.T) {
	lex := StringLexer("1 * (2 + 3) * free(willie)")
	parser := NewParser(lex)
	ast, err := parser.Expression()
	if err != nil {
		t.Error(err.Error())
		return
	}

	expected := BinOpCall{
		Op: Mul,
		Left: BinOpCall{
			Op:   Mul,
			Left: NumLit(1),
			Right: BinOpCall{
				Op:    Add,
				Left:  NumLit(2),
				Right: NumLit(3),
			},
		},
		Right: FunCall{
			Name: Identifier{sym: lex.st.Intern("free")},
			Args: []Ast{Identifier{sym: lex.st.Intern("willie")}},
		},
	}

	if !reflect.DeepEqual(expected, ast) {
		t.Errorf("Did not parse the tree that was expected. %#v", ast)
	}
}

func TestFunctions(t *testing.T) {
	// defun doubleAdd(x, y) =
	//   let sum = x + y
	//   (2 * sum)
	// end
	file, err := os.Open("../samples/fib.fun")
	if err != nil {
		t.Errorf("Could not open sample file: %s", err.Error())
		return
	}
	defer file.Close()
	lexer := NewLexer(bufio.NewReader(file), nil)
	parser := NewParser(lexer)
	ast, err := parser.Definition()
	if err != nil {
		t.Errorf("Could not parse definition: %s", err.Error())
	}

	expected := FunDef{
		Name: Identifier{sym: lexer.st.Intern("doubleAdd")},
		Args: []Ast{Identifier{sym: lexer.st.Intern("x")}, Identifier{sym: lexer.st.Intern("y")}},
		Body: []Ast{
			LetStmt{
				Ident: Identifier{sym: lexer.st.Intern("sum")},
				Expr: BinOpCall{
					Op:    Add,
					Left:  Identifier{sym: lexer.st.Intern("x")},
					Right: Identifier{sym: lexer.st.Intern("y")},
				},
			},
			ReturnStmt{
				Expr: BinOpCall{
					Op:    Mul,
					Left:  NumLit(2),
					Right: Identifier{sym: lexer.st.Intern("sum")},
				},
			},
		},
	}

	if !reflect.DeepEqual(expected, ast) {
		t.Error("Did not parse the tree that was expected.\n")
		var w strings.Builder
		ast.PPrint(0, &w)
		t.Error(w.String())
	}
}
