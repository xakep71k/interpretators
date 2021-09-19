package interpreteter

type Eater interface {
	Current() Token
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
	return e.lexer.Next()
}

func (e *_Eater) Current() Token {
	return e.lexer.Current()
}
