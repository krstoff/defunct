package compiler

import p "defunct/parser"
import "fmt"
import "os"
import "bufio"

const (
	ConstOp byte = iota
	AddOp
	MulOp
	SubOp
	DivOp
	LoadOp
	PopOp
	CallOp
	HaltOp
	Ret0Op
	Ret1Op
)

func Compile(file *os.File) (map[string]*Bytecode, error) {
	lexer := p.NewLexer(bufio.NewReader(file), nil)
	parser := p.NewParser(lexer)
	definitions := make(map[string]*Bytecode)
	var err error
	var ast p.Ast
	for err == nil {
		ast, err = parser.Definition()
		if fd, ok := ast.(p.FunDef); ok {
			name := fd.Name.Name()
			compiler := NewCompiler()
			fd.Accept(compiler)
			bytecode := compiler.Finish()
			definitions[name] = &bytecode
		}
	}
    if !p.IsEof(err) {
		return nil, err
	}
	return definitions, nil
} 

type Compiler struct {
	depth int
	idents []LocalInfo
	Name p.Identifier
	Args int
	Constants []Value
	Bytes []byte
}

type LocalInfo struct {
	name p.Identifier
	depth int
}

func (c *Compiler) EnterScope() {
	c.depth += 1
}

func (c *Compiler) ExitScope() {
	if c.depth == 0 { return }
	c.depth -= 1
	i := len(c.idents) - 1
	for i >= 0 && c.idents[i].depth > c.depth {
		i--
		c.Write(PopOp)
	}
	c.idents = c.idents[:i + 1]
}

func (c *Compiler) PushLocal(newLocal p.Identifier) {
	if len(c.idents) >= 255 {
		panic("Max number of locals reached")
	}
	c.idents = append(c.idents, LocalInfo {
		name: newLocal,
		depth: c.depth,
	})
}

func (c *Compiler) OffsetOf(newLocal p.Identifier) (int, bool) {
	var offset int
	for i := len(c.idents) - 1; i >= 0; i-- {
		if c.idents[i].name == newLocal {
			return i, true
		}
	}
	return offset, false
}

func NewCompiler() *Compiler {
	c := new(Compiler)
	c.idents = make([]LocalInfo, 0)
	c.Constants = make([]Value, 0)
	c.Bytes = make([]byte, 0)
	return c
}

func (c *Compiler) Write(b byte) {
	c.Bytes = append(c.Bytes, b)
}

func (c *Compiler) Finish() Bytecode {
	return Bytecode {
		Constants: c.Constants,
		Bytes: c.Bytes,
		Name: c.Name,
		Args: c.Args,
	}
}

func (c *Compiler) VisitReserved(_ p.Reserved) {
}
func (c *Compiler) VisitDelimeter(_ p.Delimeter) {

}
func (c *Compiler) VisitIdentifier(ident p.Identifier) {
	offset, ok := c.OffsetOf(ident)
	if !ok {
		panic("variable used before declared")
	}
	c.Write(LoadOp)
	c.Write(byte(offset))
}
func (c *Compiler) VisitStringLit(_ p.StringLit) {

}
func (c *Compiler) VisitNumLit(nl p.NumLit) {
	n := float64(nl)
	index := len(c.Constants)
	c.Constants = append(c.Constants, n)
	if index > 255 { panic("Maximum number of program literals reached.")}
	c.Write(ConstOp)
	c.Write(byte(index))
}
func (c *Compiler) VisitOperator(op p.Operator) {
	switch op {
	case p.Add: c.Write(AddOp)
	case p.Mul: c.Write(MulOp)
	case p.Sub: c.Write(SubOp)
	case p.Div: c.Write(DivOp)
	default: panic("Unsupported")
	}
}
func (c *Compiler) VisitBinOpCall(bop p.BinOpCall) {
	bop.Left.Accept(c)
	bop.Right.Accept(c)
	bop.Op.Accept(c)
}
func (c *Compiler) VisitFunCall(fc p.FunCall) {
	numArgs := len(fc.Args)
	for _, arg := range fc.Args {
		arg.Accept(c)
	}
	fc.Name.Accept(c)
	c.Write(CallOp)
	c.Write(byte(numArgs))
}
func (c *Compiler) VisitLetStmt(ls p.LetStmt) {
	ls.Expr.Accept(c)
	c.PushLocal(ls.Ident)
}
func (c *Compiler) VisitReturnStmt(rs p.ReturnStmt) {
	if rs.Expr == nil || rs.Expr == p.Semicolon {
		c.Write(Ret0Op)
	} else {
		rs.Expr.Accept(c)
		c.Write(Ret1Op)
	}
}
func (c *Compiler) VisitFunDef(fd p.FunDef) {
	c.Args = len(fd.Args)
	c.Name = fd.Name
	c.EnterScope()
	for _, id := range fd.Args {
		if id, ok := id.(p.Identifier); ok {
			c.PushLocal(id)
		} else { panic("Expected only identifiers in function definition") }
	}
	for _, stmt := range fd.Body {
		stmt.Accept(c)
	}
	c.ExitScope()
}
func (c *Compiler) VisitModule(_ p.Module) {
	// for def := range m.Defs {
	// 	e2 := NewCompiler()
	// 	placeholder := new(Bytecode)
	// 	def.Accept(e2)
	// 	*placeholder = e2.Done()

	// 	index := len(e.Constants)
	// 	e.Constants = append(e.Constants, placeholder)
	// 	e.Write(ConstOp)
	// 	e.Write(byte(index))
	// 	e.Locals.Push(def.Name)
	// }
}
func (c *Compiler) VisitBlockStmt(bs p.BlockStmt) {
	c.EnterScope()
	body := []p.Ast(bs)
	for _, stmt := range body {
		stmt.Accept(c)
	}
	c.ExitScope()
}
func (c *Compiler) VisitExprStmt(es p.ExprStmt) {
	es.Expr.Accept(c)
	c.Write(PopOp)
}

func Disassemble(bytes []byte, constants []Value) string {
	s := ""
	for i := 0; i < len(bytes); i++ {
		switch bytes[i] {
		case ConstOp:
			arg := bytes[i + 1]
			i += 1
			s = s + fmt.Sprintf("const %v\n", constants[arg])
		case MulOp:
			s = s + "mul\n"
		case AddOp:
			s = s + "add\n"
		case SubOp:
			s = s + "sub\n"
		case DivOp:
			s = s + "div\n"
		case LoadOp:
			arg := bytes[i + 1]
			i += 1
			s = s + fmt.Sprintf("load $%v\n", arg)
		case PopOp:
			s = s + "pop\n"
		default:
			panic("Unknown opcode encountered.")
		}
	}
	return s
}