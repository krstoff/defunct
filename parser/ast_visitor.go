package parser

type Visitor interface {
	VisitReserved(Reserved)
	VisitDelimeter(Delimeter)
	VisitIdentifier(Identifier)
	VisitStringLit(StringLit)
	VisitNumLit(NumLit)
	VisitOperator(Operator)
	VisitBinOpCall(BinOpCall)
	VisitFunCall(FunCall)
	VisitLetStmt(LetStmt)
	VisitReturnStmt(ReturnStmt)
	VisitFunDef(FunDef)
	VisitBlockStmt(BlockStmt)
	VisitExprStmt(ExprStmt)
}

func (token Reserved) Accept(v Visitor) {
	v.VisitReserved(token)
}
func (token Delimeter) Accept(v Visitor) {
	v.VisitDelimeter(token)
}
func (token Identifier) Accept(v Visitor) {
	v.VisitIdentifier(token)
}
func (token StringLit) Accept(v Visitor) {
	v.VisitStringLit(token)
}
func (token NumLit) Accept(v Visitor) {
	v.VisitNumLit(token)
}
func (token Operator) Accept(v Visitor) {
	v.VisitOperator(token)
}

func (bop BinOpCall) Accept(v Visitor) {
	v.VisitBinOpCall(bop)
}
func (fc FunCall) Accept(v Visitor) {
	v.VisitFunCall(fc)
}
func (ls LetStmt) Accept(v Visitor) {
	v.VisitLetStmt(ls)
}
func (rs ReturnStmt) Accept(v Visitor) {
	v.VisitReturnStmt(rs)
}
func (fd FunDef) Accept(v Visitor) {
	v.VisitFunDef(fd)
}
func (bs BlockStmt) Accept(v Visitor) {
	v.VisitBlockStmt(bs)
}
func (es ExprStmt) Accept(v Visitor) {
	v.VisitExprStmt(es)
}