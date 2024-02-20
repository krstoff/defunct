package main

type Symbol int
type SymbolTable struct {
	table map[string]Symbol
	n int
}

func (st *SymbolTable) inc() int {
	st.n += 1
	return st.n
}

func (st *SymbolTable) Intern(s string) Symbol {
	n, ok := st.table[s]
	if ok {
		return n
	}
	st.table[s] = Symbol(st.inc())
	return Symbol(st.n)
}

func NewSymbolTable() SymbolTable {
	var st SymbolTable
	st.table = make(map[string]Symbol)
	return st
}