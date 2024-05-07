package compiler

import p "defunct/parser"
import "fmt"
import "os"
import "io"
import "bufio"

const (
	ConstOp byte = iota
	AddOp
	MulOp
	SubOp
	DivOp
	LoadOp
	LoadGlobalOp
	PopOp
	CallOp
	HaltOp
	Ret0Op
	Ret1Op

)

func Compile(globals map[Value]Value, file *os.File) (map[string]*Bytecode, error) {
	lexer := p.NewLexer(bufio.NewReader(file), nil)
	parser := p.NewParser(lexer)
	definitions := make(map[string]*Bytecode)
	var err error
	var ast p.Ast
	for err == nil {
		ast, err = parser.Definition()
		if fd, ok := ast.(p.FunDef); ok {
			name := fd.Name.Name()
			compiler := NewCompiler(globals)
			fd.Accept(compiler)
			bytecode := compiler.Finish()
			definitions[name] = &bytecode
			globals[fd.Name] = &bytecode
			Disassemble(name, bytecode.Bytes, bytecode.Constants, os.Stdout)
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
	Globals map[Value]Value
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

func (c *Compiler) FindGlobal(v Value) (Value, bool) {
	obj, ok := c.Globals[v]
	if ok {
		return obj, true
	} else {
		return nil, false
	}
}

func (c *Compiler) PushConstant(value Value) {
	index := len(c.Constants)
	c.Constants = append(c.Constants, value)
	if index > 255 { panic("Maximum number of program literals reached.")}
	c.Write(ConstOp)
	c.Write(byte(index))
}

func NewCompiler(globals map[Value]Value) *Compiler {
	c := new(Compiler)
	c.idents = make([]LocalInfo, 0)
	c.Constants = make([]Value, 0)
	c.Bytes = make([]byte, 0)
	c.Globals = globals
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
		obj, ok := c.FindGlobal(ident)
		if !ok {
			panic("Variable undefined.")
		}
		c.PushConstant(obj)
		return
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
func Disassemble(name string, bytes []byte, constants []Value, w io.Writer) {
	fmt.Fprintf(w, "ASSEMBLY %s\n", name)
	for i := 0; i < len(bytes); i++ {
		switch bytes[i] {
		case ConstOp:
			arg := bytes[i + 1]
			if bc, ok := constants[arg].(*Bytecode); ok {
				fmt.Fprintf(w, "const %s", ":" + bc.Name.Name())
			} else {
				fmt.Fprintf(w, "const %v", constants[arg])
			}
			i+=1
		case MulOp:
			fmt.Fprintf(w, "opmul ")
		case AddOp:
			fmt.Fprintf(w, "opadd ")
		case SubOp:
			fmt.Fprintf(w, "opsub ")
		case DivOp:
			fmt.Fprintf(w, "opdiv ")
		case LoadOp:
			arg := bytes[i + 1]
			i += 1
			fmt.Fprintf(w, "load $%v ", arg)
		case PopOp:
			fmt.Fprintf(w, "pop ")
		case CallOp:
			fmt.Fprintf(w, "call #%d", bytes[i + 1])
			i +=1 	
		case Ret0Op: fmt.Fprintf(w, "return")
		case Ret1Op: fmt.Fprintf(w, "return1")
		case LoadGlobalOp: fmt.Fprintf(w, "loadGlobal")
		default:
			panic("Unknown opcode encountered.")
		}
		fmt.Fprintf(w, "\n")
	}
	fmt.Fprintf(w, "\n")
}

func Decode(bytes []byte, constants []Value, w io.Writer) {
	switch bytes[0] {
	case ConstOp:
		arg := bytes[1]
		if bc, ok := constants[arg].(*Bytecode); ok {
			fmt.Fprintf(w, "const %s", ":" + bc.Name.Name())
		} else {
			fmt.Fprintf(w, "const %v", constants[arg])
		}
	case MulOp:
		fmt.Fprintf(w, "opmul ")
	case AddOp:
		fmt.Fprintf(w, "opadd ")
	case SubOp:
		fmt.Fprintf(w, "opsub ")
	case DivOp:
		fmt.Fprintf(w, "opdiv ")
	case LoadOp:
		arg := bytes[1]
		fmt.Fprintf(w, "load $%v ", arg)
	case PopOp:
		fmt.Fprintf(w, "pop ")
	case CallOp:
		fmt.Fprintf(w, "call #%d", bytes[1])
	case Ret0Op: fmt.Fprintf(w, "return")
	case Ret1Op: fmt.Fprintf(w, "return1")
	case LoadGlobalOp: fmt.Fprintf(w, "loadGlobal")
	default:
		panic("Unknown opcode encountered.")
	}
}	