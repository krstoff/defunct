package parser

import "fmt"
import "io"

type Parser struct {
	lex *Lexer
	err error
}

type Ast interface {
	PPrint(indent int, builder io.Writer)
	Accept(v Visitor)
}
type BinOpCall struct {
	Op    Operator
	Left  Ast
	Right Ast
}
type FunCall struct {
	Name Ast
	Args []Ast
}
type LetStmt struct {
	Ident Identifier
	Expr  Ast
}
type ReturnStmt struct {
	Expr Ast
}
type ExprStmt struct {
	Expr Ast
}
type BlockStmt []Ast
type FunDef struct {
	Name Identifier
	Args []Ast
	Body []Ast
}
type Module struct {
	Defs []FunDef
}

func NewParser(lex *Lexer) Parser {
	var p Parser
	p.lex = lex
	return p
}

func expect[T comparable](t T, p *Parser) (T, error) {
	tok, err := p.lex.NextToken()
	if err != nil {
		return t, fmt.Errorf("Expected %v, errored: %s", tok, err.Error())
	}
	value, ok := tok.(T)
	if !ok || value != t {
		return t, fmt.Errorf("Expected %v, but found %T %v", t, tok, tok)
	}
	return t, nil
}

func require[T comparable](_ T, p *Parser) (T, error) {
	var value T
	tok, err := p.lex.NextToken()
	if err != nil {
		return value, fmt.Errorf("Expected %v, errored: %s", tok, err.Error())
	}
	value, ok := tok.(T)
	if !ok {
		return value, fmt.Errorf("Expected %v, but found %T %v", tok, tok, tok)
	}
	return value, nil
}

func trimSemicolon(p *Parser) {
	tok, err := p.lex.PeekToken()
	if err != nil {
		return
	}
	value, ok := tok.(Delimeter)
	if ok && value == Semicolon {
		_, _ = p.lex.NextToken()
	}
}

func (p *Parser) Module() (Ast, error) {
	var err error
	var def Ast
	defs := make([]FunDef, 0)
	for err != nil {
		def, err = p.Definition()
		d, _ := def.(FunDef)
		defs = append(defs, d)
	}
	if IsEof(err) {
		return Module {
			Defs: defs,
		}, nil
	} else {
		return nil, err
	}
}

func (p *Parser) Expression() (Ast, error) {
	return p.expression(0)
}

func (p *Parser) Definition() (Ast, error) {
	tok, err := p.lex.PeekToken()
	if err != nil {
		return nil, err
	}
	switch tok {
	case Defun:
		return p.parseDefun()
	}
	return nil, fmt.Errorf("Expected a definition, found %T %v", tok, tok)
}

// may return a single semicolon. callers need to prune no-op statements
func (p *Parser) Statement() (Ast, error) {
	tok, err := p.lex.PeekToken()
	if err != nil {
		return nil, err
	}
	if keyword, ok := tok.(Reserved); ok {
		switch keyword {
		case Let:
			return p.parseLet()
		case Return:
			return p.parseReturn()
		case End:
			return nil, fmt.Errorf("Expected a statement but found keyword end. Empty bodies are not allowed.")
		}
	}
	if delimeter, ok := tok.(Delimeter); ok && delimeter == Semicolon {
		_, _ = p.lex.NextToken()
		return delimeter, nil
	}
	expr, err := p.Expression()

	return ExprStmt { Expr: expr }, err
}

func (p *Parser) Block() (Ast, error) {
	_, err := expect(Do, p)
	if err != nil {
		return nil, err
	}
	
	nextToken, err := p.lex.PeekToken()
	if err != nil {
		return nil, fmt.Errorf("parseDefun: %w", err)
	}

	body := make([]Ast, 0)

	for nextToken != End {
		stmt, err := p.Statement()
		if err != nil {
			return nil, err
		}
		if stmt != Semicolon {
			body = append(body, stmt)
		}
		nextToken, err = p.lex.PeekToken()
		if err != nil {
			return nil, err
		}
	}

	_, err = expect(End, p)

	return BlockStmt(body), nil
}

func (p *Parser) expression(prec int) (Ast, error) {
	var err error
	var expr Ast
	lex := p.lex

	token, err := lex.PeekToken()
	if err != nil {
		return nil, err
	}

	expr, err = token.ParsePrefix(p)
	if err != nil {
		return nil, err
	}

	token, err = lex.PeekToken()
	if IsEof(err) {
		return expr, nil
	}
	if err != nil {
		return nil, err
	}

	for token.Precedence() > prec {
		expr, err = token.ParseInfix(expr, p)
		if err != nil {
			return nil, err
		}
		token, err = lex.PeekToken()
		if err != nil {
			if IsEof(err) {
				err = nil
			}
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
	_, _ = parser.lex.NextToken()
	return token, nil
}
func (token StringLit) ParsePrefix(parser *Parser) (Ast, error) {
	_, _ = parser.lex.NextToken()
	return token, nil
}
func (token Delimeter) ParsePrefix(parser *Parser) (Ast, error) {
	switch token {
	case OpenParen:
		_, _ = parser.lex.NextToken()
		exp, err := parser.expression(0)
		if err != nil {
			return nil, err
		}
		tok, err := parser.lex.NextToken()
		if err != nil {
			return nil, err
		}
		if tok != CloseParen {
			return nil, fmt.Errorf("ParsePrefix: Expected a closing parentheses, found %v", token)
		}
		return exp, nil
	}
	return nil, fmt.Errorf("ParsePrefix: Expected a prefix operator, found %v", token)
}
func (token NumLit) ParsePrefix(parser *Parser) (Ast, error) {
	_, _ = parser.lex.NextToken()
	return token, nil
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
	return BinOpCall{
		Op:    token,
		Left:  left,
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
	if token == OpenParen || token == OpenBracket {
		return 4
	} else {
		return 0
	}
}
func (token Identifier) Precedence() int {
	return 0
}
func (token StringLit) Precedence() int {
	return 0
}
func (token Operator) Precedence() int {
	switch token {
	case Dot:
		return 4
	case Mul, Div:
		return 2
	case Add, Sub:
		return 1
	default:
		return 0
	}
}
func (token NumLit) Precedence() int {
	return 0
}

func (p *Parser) functionCall(left Ast) (Ast, error) {
	var err error
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
	_, err = p.lex.NextToken() // ')'
	if err != nil {
		err = fmt.Errorf("Expected ')', errored with: %s", err.Error())
	}
	return FunCall{
		Name: left,
		Args: args,
	}, err
}

func (p *Parser) parseLet() (Ast, error) {
	_, err := expect(Let, p)
	if err != nil {
		return nil, err
	}

	var ident Identifier
	ident, err = require(ident, p)
	if err != nil {
		return nil, err
	}

	_, err = expect(Equals, p)
	if err != nil {
		return nil, err
	}

	expr, err := p.Expression()
	if err != nil {
		return nil, err
	}

	trimSemicolon(p)
	return LetStmt{
		Ident: ident,
		Expr:  expr,
	}, nil
}

func (p *Parser) parseReturn() (Ast, error) {
	_, err := expect(Return, p)
	expr, err := p.Statement()
	if err != nil {
		return nil, err
	}
	return ReturnStmt{Expr: expr}, nil
}

func (p *Parser) parseDefun() (Ast, error) {
	_, err := expect(Defun, p)
	var name Identifier
	name, err = require(name, p)
	if err != nil {
		return nil, fmt.Errorf("parseDefun: %w", err)
	}
	_, err = expect(OpenParen, p)
	if err != nil {
		return nil, fmt.Errorf("parseDefun: %w", err)
	}

	nextToken, err := p.lex.PeekToken()
	if err != nil {
		return nil, fmt.Errorf("parseDefun: %w", err)
	}

	args := make([]Ast, 0)

	for nextToken != CloseParen {
		if len(args) >= 1 {
			_, err = expect(Comma, p)
			if err != nil {
				return nil, err
			}
		}
		var ident Identifier
		ident, err := require(ident, p)
		if err != nil {
			return nil, err
		}
		args = append(args, ident)
		nextToken, err = p.lex.PeekToken()
		if err != nil {
			return nil, err
		}
	}

	_, err = expect(CloseParen, p)
	_, err = expect(Equals, p)
	if err != nil {
		return nil, err
	}

	nextToken, err = p.lex.PeekToken()
	if err != nil {
		return nil, fmt.Errorf("parseDefun: %w", err)
	}

	body := make([]Ast, 0)

	for nextToken != End {
		stmt, err := p.Statement()
		if err != nil {
			return nil, err
		}
		if stmt != Semicolon {
			body = append(body, stmt)
		}
		nextToken, err = p.lex.PeekToken()
		if err != nil {
			return nil, err
		}
	}

	_, err = expect(End, p)

	return FunDef{
		Name: name,
		Args: args,
		Body: body,
	}, nil
}