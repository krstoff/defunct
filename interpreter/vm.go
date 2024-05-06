package interpreter

import c "defunct/compiler"
import "fmt"
const maxStackSize = 65535

type Vm struct { 
	ip int
	code c.Bytecode
	valueStack []c.Value
}

func NewVm() *Vm {
	vm := new(Vm)
	return vm
}

func (vm *Vm) Load(code c.Bytecode) {
	vm.ip = 0
	vm.code = code
}

const debugStack = true
func (vm *Vm) Run() {
	for {
		if debugStack { fmt.Println(vm.valueStack) }
		if vm.ip >= len(vm.code.Bytes) { break }
		switch vm.code.Bytes[vm.ip] {
		case c.ConstOp: vm.constant()
		case c.AddOp: vm.add()
		case c.SubOp: vm.sub()
		case c.MulOp: vm.mul()
		case c.DivOp: vm.div()
		case c.LoadOp: vm.load()
		case c.PopOp: _ = vm.pop()
		}
		vm.ip += 1
	}
}

func (vm *Vm) Result() c.Value {
	return vm.valueStack[0]
}

func (vm *Vm) pop() c.Value {
	value := vm.valueStack[len(vm.valueStack) - 1]
	vm.valueStack = vm.valueStack[:len(vm.valueStack)- 1]
	return value
}

func (vm *Vm) push(v c.Value) {
	vm.valueStack = append(vm.valueStack, v)
}

func (vm *Vm) constant() {
	vm.ip += 1
	slot := vm.code.Bytes[vm.ip]
	arg := vm.code.Constants[int(slot)]
	vm.push(arg)
}

func (vm *Vm) add() {
	lv := vm.pop()
	l, ok := lv.(float64)
	if !ok { panic("type error")}
	rv := vm.pop()
	r, ok := rv.(float64)
	vm.push(l + r)
}
func (vm *Vm) sub() {
	lv := vm.pop()
	l, ok := lv.(float64)
	if !ok { panic("type error")}
	rv := vm.pop()
	r, ok := rv.(float64)
	vm.push(l - r)
}
func (vm *Vm) mul() {
	lv := vm.pop()
	l, ok := lv.(float64)
	if !ok { panic("type error")}
	rv := vm.pop()
	r, ok := rv.(float64)
	vm.push(l * r)
}
func (vm *Vm) div() {
	lv := vm.pop()
	l, ok := lv.(float64)
	if !ok { panic("type error")}
	rv := vm.pop()
	r, ok := rv.(float64)
	vm.push(l / r)
}

func (vm *Vm) load() {
	vm.ip += 1
	slot := vm.code.Bytes[vm.ip]
	arg := vm.valueStack[int(slot)]
	vm.push(arg)
}