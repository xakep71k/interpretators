package token

import (
	"bytes"
	"interpreteter/internal/interpreteter"
)

func NewReader(buffer *bytes.Buffer) (interpreteter.Reader, error) {
	reader := &_Reader{
		buffer: buffer,
	}
	if err := reader.Next(); err != nil {
		return nil, err
	}
	return reader, nil
}

type _Reader struct {
	currentToken interpreteter.Token
	buffer       *bytes.Buffer
}

func (e *_Reader) Current() interpreteter.Token {
	return e.currentToken
}

func (e *_Reader) Next() error {
	token, err := _MakeFromBuffer(e.buffer)
	if err != nil {
		return err
	}
	e.currentToken = token
	return nil
}
