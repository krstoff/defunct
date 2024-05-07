package parser

import "io"
import "fmt"
import "strings"

const indent_width int = 2

func (token Reserved) PPrint(indent int, w io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	switch token {
	case Defun:
		fmt.Fprintf(w, "%sdefun", tab)
	case End:
		fmt.Fprintf(w, "%send", tab)
	case Return:
		fmt.Fprintf(w, "%sreturn", tab)
	case Let:
		fmt.Fprintf(w, "%slet", tab)
	case If:
		fmt.Fprintf(w, "%sif", tab)
	case Then:
		fmt.Fprintf(w, "%sthen", tab)
	case Else:
		fmt.Fprintf(w, "%selse", tab)
	default:
		panic("Tried to pretty print something not printable.")
	}
}

func (token Delimeter) PPrint(indent int, w io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	fmt.Fprint(w, tab)
	switch token {
	case OpenParen:
		fmt.Fprintf(w, "%s(", tab)
	case CloseParen:
		fmt.Fprintf(w, "%s)", tab)
	case OpenBracket:
		fmt.Fprintf(w, "%s[", tab)
	case CloseBracket:
		fmt.Fprintf(w, "%s]", tab)
	case Equals:
		fmt.Fprintf(w, "%s=", tab)
	case Comma:
		fmt.Fprintf(w, "%s,", tab)
	case Semicolon:
		fmt.Fprint(w, ";", tab)
	default:
		panic("Tried to pretty print something not printable.")
	}
}
func (token Identifier) PPrint(indent int, w io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	fmt.Fprint(w, tab)
	fmt.Fprintf(w, ":"+token.sym.Name)
}
func (token StringLit) PPrint(indent int, w io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	fmt.Fprint(w, tab)
	fmt.Fprintf(w, "\""+string(token)+"\"")
}
func (token NumLit) PPrint(indent int, w io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	fmt.Fprint(w, tab)
	fmt.Fprintf(w, "%v", float64(token))
}
func (token Operator) PPrint(indent int, w io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	fmt.Fprint(w, tab)
	switch token {
	case Add:
		fmt.Fprint(w, "+")
	case Sub:
		fmt.Fprint(w, "-")
	case Mul:
		fmt.Fprint(w, "*")
	case Div:
		fmt.Fprint(w, "/")
	default:
		panic("Tried to pretty print something not printable.")
	}
}

func (bop BinOpCall) PPrint(indent int, b io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	fmt.Fprintf(b, "%sOp", tab)
	bop.Op.PPrint(0, b)
	fmt.Fprint(b, "\n")
	bop.Left.PPrint(indent+1, b)
	fmt.Fprint(b, "\n")
	bop.Right.PPrint(indent+1, b)

}
func (fc FunCall) PPrint(indent int, b io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	fmt.Fprintf(b, "%sCall", tab)
	fc.Name.PPrint(0, b)
	for _, arg := range fc.Args {
		fmt.Fprint(b, "\n")
		arg.PPrint(indent+1, b)
	}
}
func (ls LetStmt) PPrint(indent int, b io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	fmt.Fprintf(b, "%sLet ", tab)
	ls.Ident.PPrint(0, b)
	fmt.Fprint(b, "\n")
	ls.Expr.PPrint(indent+1, b)
}
func (rs ReturnStmt) PPrint(indent int, b io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	fmt.Fprintf(b, "%sReturn\n", tab)
	rs.Expr.PPrint(indent+1, b)

}
func (fd FunDef) PPrint(indent int, b io.Writer) {
	fmt.Fprint(b, "\n")
	tab := strings.Repeat(" ", indent*indent_width)
	tab2 := strings.Repeat(" ", (indent+1)*indent_width)
	fmt.Fprintf(b, "%sFunDef", tab)
	fd.Name.PPrint(0, b)
	fmt.Fprintf(b, "\n%s(", tab2)
	for i, arg := range fd.Args {
		if i > 0 {
			fmt.Fprintf(b, ", ")
		}
		arg.PPrint(0, b)
	}
	fmt.Fprintf(b, ")\n")
	for _, stmt := range fd.Body {
		stmt.PPrint(indent+1, b)
		fmt.Fprint(b, "\n")
	}
}
func (bs BlockStmt) PPrint(indent int, b io.Writer) {
	tab := strings.Repeat(" ", indent*indent_width)
	fmt.Fprintf(b, "%sDo\n", tab)
	for _, stmt := range []Ast(bs) {
		stmt.PPrint(indent+1, b)
		fmt.Fprint(b, "\n")
	}
}
func (es ExprStmt) PPrint(ident int, b io.Writer) {
	es.Expr.PPrint(ident, b)
}

func (m Module) PPrint(ident int, b io.Writer) {
	for _, d := range m.Defs {
		d.PPrint(ident, b)
		fmt.Fprintf(b, "\n")
	}
}