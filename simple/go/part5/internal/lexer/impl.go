package lexer

import (
	"bytes"
	"interpreteter/internal/interpreteter"
)

func New(tokenFactory interpreteter.TokenFactory, buffer *bytes.Buffer) (interpreteter.Lexer, error) {
	impl := &_Impl{
		buffer:       buffer,
		tokenFactory: tokenFactory,
	}
	if err := impl.Next(); err != nil {
		return nil, err
	}
	return impl, nil
}

type _Impl struct {
	currentToken interpreteter.Token
	buffer       *bytes.Buffer
	tokenFactory interpreteter.TokenFactory
}

func (impl *_Impl) Current() interpreteter.Token {
	return impl.currentToken
}

func (impl *_Impl) Next() error {
	token, err := impl._MakeFromBuffer(impl.buffer)
	if err != nil {
		return err
	}
	impl.currentToken = token
	return nil
}
