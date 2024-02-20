package main

type Entry struct {
	name string
}
type Symbol *Entry
type SymbolTable struct {
	table map[string] *Entry
	parent *SymbolTable
}

func (st *SymbolTable) Intern(s string) Symbol {
	entry, ok := st.table[s]
	if ok {
		return entry
	}
	e := &Entry {
		name: s,
	}
	st.table[s] = e
	return Symbol(e)
}

func NewSymbolTable(parent *SymbolTable) SymbolTable {
	var st SymbolTable
	st.table = make(map[string] *Entry)
	st.parent = parent
	return st
}