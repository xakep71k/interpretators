package interpreteter

type Eater interface {
	Lexer
	Eat(string) error
}

func NewEater(lexer Lexer) Eater {
	return &_Eater{
		lexer: lexer,
	}
}

type _Eater struct {
	lexer Lexer
}

func (e *_Eater) Eat(kind string) error {
	if e.lexer.Current().Type() != kind {
		return ErrInvalidSyntax
	}
	return e.Next()
}

func (e *_Eater) Current() Token {
	return e.lexer.Current()
}

func (e *_Eater) Next() error {
	return e.lexer.Next()
}
