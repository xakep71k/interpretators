package token

import (
	"bytes"
	"interpreteter/internal/interpreteter"
	"strconv"
	"strings"
	"unicode"
)

func _New(value int, kind string) interpreteter.Token {
	return &_Impl{
		_Type:  kind,
		_Value: value,
	}
}

func _MakeFromBuffer(input *bytes.Buffer) (interpreteter.Token, error) {
	for {
		ch, _, err := input.ReadRune()
		if err != nil {
			return nil, err
		}
		if _IsSpace(ch) {
			continue
		}

		if unicode.IsDigit(ch) {
			input.UnreadRune()
			return _MakeIntegerFromBuffer(input)
		}

		switch ch {
		case '+':
			return _New(int(ch), interpreteter.PLUS), nil
		case '-':
			return _New(int(ch), interpreteter.MINUS), nil
		case '\n':
			return _New(int(ch), interpreteter.NEWLINE), nil
		}
		return nil, interpreteter.ErrInvalidSyntax
	}
}

func _MakeIntegerFromBuffer(input *bytes.Buffer) (interpreteter.Token, error) {
	var buf strings.Builder

	for {
		ch, _, err := input.ReadRune()
		if err != nil {
			return nil, err
		}
		if !unicode.IsDigit(ch) {
			break
		}
		buf.WriteRune(ch)
	}

	input.UnreadRune()

	value, err := strconv.ParseInt(buf.String(), 10, 32)
	if err != nil {
		return nil, err
	}

	return _New(int(value), interpreteter.INTEGER), nil
}
