package token

import (
	"interpreteter/internal/interpreteter"
)

type Eater interface {
	Reader
	Eat(string) error
}

func NewEater(reader Reader) Eater {
	return &_Eater{
		reader: reader,
	}
}

type _Eater struct {
	reader Reader
}

func (e *_Eater) Eat(kind string) error {
	if e.reader.Current().Type() != kind {
		return interpreteter.ErrInvalidSyntax
	}
	return e.Next()
}

func (e *_Eater) Current() interpreteter.Token {
	return e.reader.Current()
}

func (e *_Eater) Next() error {
	return e.reader.Next()
}
