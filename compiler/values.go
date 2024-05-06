package compiler
import "defunct/parser"

type Value interface {}

type Bytecode struct {
	Name parser.Identifier
	Args int
	Constants []Value
	Bytes []byte
}