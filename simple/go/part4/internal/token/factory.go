package token

import (
	"interpreteter/internal/interpreteter"
)

type Factory struct {
}

func NewFactory() *Factory {
	return &Factory{}
}

func (f *Factory) Create(value int, kind string) interpreteter.Token {
	return &_Impl{
		_Type:  kind,
		_Value: value,
	}
}
