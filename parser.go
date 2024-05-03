package main
import "fmt"

type Parser struct {
	lex *Lexer
	err error
}

type Ast interface {}
type BinOpCall struct {
	Op Operator
	Left Ast
	Right Ast
}

func NewParser(lex *Lexer) Parser {
	var p Parser
	p.lex = lex
	return p
}

func (p *Parser) expression(prec int) (Ast, error) {
	var err error
	var expr Ast
	lex := p.lex

	token, err := lex.PeekToken()
	if err != nil { return nil, err }

	expr, err = token.ParsePrefix(p)
	if err != nil { return nil, err }

	token, err = lex.PeekToken()
	if IsEof(err) {
		return expr, nil
	}
	if err != nil { return nil, err }
	
	for token.Precedence() > prec {
		if err != nil { return nil, err }
		expr, err = token.ParseInfix(expr, p)
		token, err = lex.PeekToken()
	}
	return expr, err
}

func (token Reserved) ParsePrefix(parser *Parser) (Ast, error) {
	return nil, nil
}
func (token Delimeter) ParsePrefix(parser *Parser) (Ast, error) {
	return nil, nil
}
func (token Identifier) ParsePrefix(parser *Parser) (Ast, error) {
	return parser.lex.NextToken()
}
func (token StringLit) ParsePrefix(parser *Parser) (Ast, error) {
	return parser.lex.NextToken()
}
func (token Operator) ParsePrefix(parser *Parser) (Ast, error) {
	return nil, nil
}
func (token NumLit) ParsePrefix(parser *Parser) (Ast, error) {
	return parser.lex.NextToken()
}

func (token Reserved) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	return nil, fmt.Errorf("ParseInfix: Expected an infix operator, found a reserved word %v", token)
}
func (token Delimeter) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	return nil, nil
}
func (token Identifier) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	return nil, fmt.Errorf("ParseInfix: Expected an infix operator, found an identifier %v", token)
}
func (token StringLit) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	return nil, fmt.Errorf("ParseInfix: Expected an infix operator, found a string literal %s", token)
}
func (token Operator) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	right, err := parser.expression(token.Precedence())
	return BinOpCall {
		Op: token,
		Left: left,
		Right: right,
	}, err
}
func (token NumLit) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	return nil, fmt.Errorf("ParseInfix: Expected an infix operator, found a number %v", token)
}

func (token Reserved) Precedence() int {
	return 0
}
func (token Delimeter) Precedence() int {
	if token == OpenParen || token == OpenBracket { return 4 } else { return 0 }
}
func (token Identifier) Precedence() int {
	return 0
}
func (token StringLit) Precedence() int {
	return 0
}
func (token Operator) Precedence() int {
	switch token {
	case Dot: return 4
	case Mul, Div: return 2
	case Add, Sub: return 1
	default: return 0
	}
}
func (token NumLit) Precedence() int {
	return 0
}