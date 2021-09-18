package token

import (
	"bytes"
	"interpreteter/internal/interpreteter"
	"unicode"
)

func _New(value int, kind string) interpreteter.Token {
	return &_Impl{
		_Type:  kind,
		_Value: value,
	}
}

func _MakeFromBuffer(input *bytes.Buffer) (interpreteter.Token, error) {
	ch, _, err := input.ReadRune()
	if err != nil {
		return nil, err
	}
	if unicode.IsDigit(ch) {
		return _New(int(ch)-int('0'), interpreteter.INTEGER), nil
	}

	switch ch {
	case '+':
		return _New(int(ch), interpreteter.PLUS), nil
	case '\n':
		return _New(int(ch), interpreteter.NEWLINE), nil
	}

	return nil, interpreteter.ErrInvalidSyntax
}
