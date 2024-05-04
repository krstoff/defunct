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
type FunCall struct {
	Name Ast
	Args []Ast
}

func NewParser(lex *Lexer) Parser {
	var p Parser
	p.lex = lex
	return p
}

func (p *Parser) Expression() (Ast, error) {
	return p.expression(0)
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
		expr, err = token.ParseInfix(expr, p)
		if err != nil && !IsEof(err) { return nil, err }
		token, err = lex.PeekToken()
		if err != nil { 
			if IsEof(err) { err = nil }
			break
		}
	}
	return expr, err
}

func (token Reserved) ParsePrefix(parser *Parser) (Ast, error) {
	return nil, fmt.Errorf("ParsePrefix: Expected a prefix operator, found %v", token)
}
func (token Operator) ParsePrefix(parser *Parser) (Ast, error) {
	return nil, fmt.Errorf("ParsePrefix: Expected a prefix operator, found %v", token)
}
func (token Identifier) ParsePrefix(parser *Parser) (Ast, error) {
	return parser.lex.NextToken()
}
func (token StringLit) ParsePrefix(parser *Parser) (Ast, error) {
	return parser.lex.NextToken()
}
func (token Delimeter) ParsePrefix(parser *Parser) (Ast, error) {
	switch token {
	case OpenParen:
		_, _ = parser.lex.NextToken()
		exp, err := parser.expression(0)
		if err != nil { return nil, err }
		tok, err := parser.lex.NextToken()
		if err != nil { return nil, err }
		if tok != CloseParen {
			return nil, fmt.Errorf("ParsePrefix: Expected a closing parentheses, found %v", token)
		}
		return exp, nil
	}
	return nil, fmt.Errorf("ParsePrefix: Expected a prefix operator, found %v", token)
}
func (token NumLit) ParsePrefix(parser *Parser) (Ast, error) {
	return parser.lex.NextToken()
}

func (token Reserved) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	return nil, fmt.Errorf("ParseInfix: Expected an infix operator, found a reserved word %v", token)
}
func (token Delimeter) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	switch token {
	case OpenParen:
		return parser.functionCall(left)
	}
	return nil, fmt.Errorf("ParseInfix: Expected an infix operator, found %v", token)
}
func (token Identifier) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	return nil, fmt.Errorf("ParseInfix: Expected an infix operator, found an identifier %v", token)
}
func (token StringLit) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	return nil, fmt.Errorf("ParseInfix: Expected an infix operator, found a string literal %s", token)
}
func (token Operator) ParseInfix(left Ast, parser *Parser) (Ast, error) {
	_, _ = parser.lex.NextToken()
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

func (p *Parser) functionCall(left Ast) (Ast, error) {
	args := make([]Ast, 0)
	_, _ = p.lex.NextToken() // '('

	for next, err := p.lex.PeekToken(); next != CloseParen; next, err = p.lex.PeekToken() {
		if err != nil {
			return nil, err
		}
		if len(args) >= 1 {
			tok, err := p.lex.NextToken()
			if err != nil {
				return nil, err
			}
			if tok != Comma {
				return nil, fmt.Errorf("Expected ',', found %v", tok)
			}
		}
		arg, err := p.Expression()
		if err != nil {
			return nil, err
		}
		args = append(args, arg)
	}
	_, _ = p.lex.NextToken() // ')'
	return FunCall {
		Name: left,
		Args: args,
	}, nil
}