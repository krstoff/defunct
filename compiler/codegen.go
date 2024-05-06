package compiler

import p "defunct/parser"
import "fmt"

const (
	ConstOp byte = iota
	AddOp
	MulOp
	SubOp
	DivOp
)

type Bytecode struct {
	Constants []Value
	Bytes []byte
}

type Emitter struct {
	Constants []Value
	Bytes []byte
}

func NewEmitter() *Emitter {
	e := new(Emitter)
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
func (e *Emitter) VisitIdentifier(_ p.Identifier) {

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
func (e *Emitter) VisitLetStmt(_ p.LetStmt) {

}
func (e *Emitter) VisitReturnStmt(_ p.ReturnStmt) {

}
func (e *Emitter) VisitFunDef(_ p.FunDef) {

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
		default:
			panic("Unknown opcode encountered.")
		}
	}
	return s
}