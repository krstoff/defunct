package compiler

import p "defunct/parser"
import "fmt"

const (
	ConstOp byte = iota
	AddOp
	MulOp
	SubOp
	DivOp
	LoadOp
	PopOp
)

type LocalInfo struct {
	name p.Identifier
	depth int
}

type Locals struct {
	depth int
	idents []LocalInfo
}

func (l *Locals) EnterScope() {
	l.depth += 1
}

func (l *Locals) ExitScope(e *Emitter) {
	if l.depth == 0 { return }
	l.depth -= 1
	i := len(l.idents) - 1
	for i >= 0 && l.idents[i].depth > l.depth {
		i--
		e.Write(PopOp)
	}
	l.idents = l.idents[:i + 1]
}

func (l *Locals) Push(newLocal p.Identifier) {
	if len(l.idents) == 255 {
		panic("Max number of locals reached")
	}
	l.idents = append(l.idents, LocalInfo {
		name: newLocal,
		depth: l.depth,
	})
}

func (l *Locals) OffsetOf(newLocal p.Identifier) (int, bool) {
	var offset int
	for i := len(l.idents) - 1; i >= 0; i-- {
		if l.idents[i].name == newLocal {
			return i, true
		}
	}
	return offset, false
}


type Bytecode struct {
	Constants []Value
	Bytes []byte
}

type Emitter struct {
	Constants []Value
	Bytes []byte
	Locals Locals
}

func NewEmitter() *Emitter {
	e := new(Emitter)
	e.Locals = Locals {
		depth: 0,
		idents: make([]LocalInfo, 0),
	}
	return e
}

func (e *Emitter) Write(b byte) {
	e.Bytes = append(e.Bytes, b)
}

func (e *Emitter) Done() Bytecode {
	return Bytecode {
		Constants: e.Constants,
		Bytes: e.Bytes,
	}
}

func (e *Emitter) VisitReserved(_ p.Reserved) {
}
func (e *Emitter) VisitDelimeter(_ p.Delimeter) {

}
func (e *Emitter) VisitIdentifier(ident p.Identifier) {
	offset, ok := e.Locals.OffsetOf(ident)
	if !ok {
		panic("variable used before declared")
	}
	e.Write(LoadOp)
	e.Write(byte(offset))
}
func (e *Emitter) VisitStringLit(_ p.StringLit) {

}
func (e *Emitter) VisitNumLit(nl p.NumLit) {
	n := float64(nl)
	index := len(e.Constants)
	e.Constants = append(e.Constants, n)
	if index > 255 { panic("Maximum number of program literals reached.")}
	e.Write(ConstOp)
	e.Write(byte(index))
}
func (e *Emitter) VisitOperator(op p.Operator) {
	switch op {
	case p.Add: e.Write(AddOp)
	case p.Mul: e.Write(MulOp)
	case p.Sub: e.Write(SubOp)
	case p.Div: e.Write(DivOp)
	default: panic("Unsupported")
	}
}
func (e *Emitter) VisitBinOpCall(bop p.BinOpCall) {
	bop.Left.Accept(e)
	bop.Right.Accept(e)
	bop.Op.Accept(e)
}
func (e *Emitter) VisitFunCall(_ p.FunCall) {

}
func (e *Emitter) VisitLetStmt(ls p.LetStmt) {
	e.Locals.Push(ls.Ident)
	ls.Expr.Accept(e)
}
func (e *Emitter) VisitReturnStmt(_ p.ReturnStmt) {

}
func (e *Emitter) VisitFunDef(_ p.FunDef) {

}
func (e *Emitter) VisitBlockStmt(bs p.BlockStmt) {
	e.Locals.EnterScope()
	body := []p.Ast(bs)
	for _, stmt := range body {
		stmt.Accept(e)
	}
	e.Locals.ExitScope(e)
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