package interpreter

import c "defunct/compiler"
import "fmt"
import "os"

const maxStackSize = 65535
const debugStack = true

type CallFrame struct {
	ip int
	offset int
	code *c.Bytecode
}

type Vm struct { 
	callFrames []CallFrame
	valueStack []c.Value
	globals    map[c.Value]c.Value
}

func NewVm() *Vm {
	vm := new(Vm)
	vm.callFrames = make([]CallFrame, 0)
	vm.valueStack = make([]c.Value, 0)
	vm.globals = make(map[c.Value]c.Value)
	return vm
}

func (vm *Vm) takeOperand() int {
	frame := vm.getFrame()
	frame.ip += 1
	return int(frame.code.Bytes[int(frame.ip)])
}

func (vm *Vm) Globals() map[c.Value]c.Value {
	return vm.globals
}

func (vm *Vm) getFrame() *CallFrame {
	return &vm.callFrames[len(vm.callFrames) - 1]
}

func (vm *Vm) pushFrame(offset int, code *c.Bytecode) {
	frame := CallFrame {
		ip: 0,
		offset: offset,
		code: code,
	}
	vm.callFrames = append(vm.callFrames, frame)
	code = vm.getFrame().code
	
}

func (vm *Vm) popFrame() {
	vm.callFrames = vm.callFrames[:len(vm.callFrames) - 1]
}

func (vm *Vm) Run(code *c.Bytecode, initialArgs ...c.Value) {
	for _, arg := range initialArgs {
		vm.push(arg)
	}
	vm.pushFrame(0, code)

mainLoop:
	for {
		frame := vm.getFrame()
		if debugStack { vm.DumpState() }
		if frame.ip >= len(frame.code.Bytes) { break }
		switch frame.code.Bytes[frame.ip] {
		case c.ConstOp: vm.constant()
		case c.AddOp: vm.add()
		case c.SubOp: vm.sub()
		case c.MulOp: vm.mul()
		case c.DivOp: vm.div()
		case c.LoadOp: vm.load()
		case c.PopOp: _ = vm.pop()
		case c.CallOp: vm.call()
		case c.Ret0Op: if vm.ret0() { break mainLoop }
		case c.Ret1Op: if vm.ret1() { break mainLoop }
		case c.LoadGlobalOp: vm.loadGlobal()
		case c.HaltOp: break mainLoop
		}
		frame.ip += 1
	}
}

func (vm *Vm) DumpState() {
	frame := vm.getFrame()
	code := frame.code
	c.Decode(code.Bytes[frame.ip:], code.Constants, os.Stdout)
	fmt.Printf("%-10s", " ")
	fmt.Println(vm.valueStack)
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
	slot := vm.takeOperand()
	arg := vm.getFrame().code.Constants[slot]
	vm.push(arg)
}

func (vm *Vm) add() {
	lv := vm.pop()
	l, ok := lv.(float64)
	if !ok { panic("type error")}
	rv := vm.pop()
	r, ok := rv.(float64)
	if !ok { panic("type error")}
	vm.push(l + r)
}
func (vm *Vm) sub() {
	lv := vm.pop()
	l, ok := lv.(float64)
	if !ok { panic("type error")}
	rv := vm.pop()
	r, ok := rv.(float64)
	if !ok { panic("type error")}
	vm.push(l - r)
}
func (vm *Vm) mul() {
	lv := vm.pop()
	l, ok := lv.(float64)
	if !ok { panic("type error")}
	rv := vm.pop()
	r, ok := rv.(float64)
	if !ok { panic("type error")}
	vm.push(l * r)
}
func (vm *Vm) div() {
	lv := vm.pop()
	l, ok := lv.(float64)
	if !ok { panic("type error")}
	rv := vm.pop()
	r, ok := rv.(float64)
	if !ok { panic("type error")}
	vm.push(l / r)
}

func (vm *Vm) load() {
	slot := vm.takeOperand()
	arg := vm.valueStack[slot + vm.getFrame().offset]
	vm.push(arg)
}

func (vm *Vm) call() {
	numArgs := vm.takeOperand()
	callee := vm.pop()
	offset := len(vm.valueStack) - int(numArgs)

	if code, ok := callee.(*c.Bytecode); ok {
		vm.pushFrame(offset, code)
	} else {
		panic("Type error: called something that was not a function")
	}
}

func (vm *Vm) ret0() bool {
	frame := vm.getFrame()
	for len(vm.valueStack) > frame.offset {
		_ = vm.pop()
	}
	vm.popFrame()
	if len(vm.callFrames) == 0 { return true }
	return false
}

func (vm *Vm) ret1() bool {
	frame := vm.getFrame()
	val := vm.pop()
	for len(vm.valueStack) > frame.offset {
		_ = vm.pop()
	}
	vm.popFrame()
	vm.push(val)
	if len(vm.callFrames) == 0 { return true }
	return false
}

func (vm *Vm) loadGlobal() {
	slot := vm.takeOperand()
	name := vm.getFrame().code.Constants[slot]
	value, ok := vm.globals[name]
	if !ok {
		panic("Global variable undefined")
	}
	vm.push(value)
}