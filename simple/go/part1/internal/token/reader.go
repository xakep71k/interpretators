package token

import (
	"interpreteter/internal/interpreteter"
	"io"
)

func NewReader(reader io.RuneReader) interpreteter.TokenReader {
	return _Reader{
		_Reader: reader,
	}
}

type _Reader struct {
	_Reader io.RuneReader
}

func (r _Reader) Read() (interpreteter.Token, error) {
	return _MakeFromIO(r._Reader)
}
