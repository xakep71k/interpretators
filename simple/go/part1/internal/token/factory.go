package token

import (
	"fmt"
	"interpreteter/internal/interpreteter"
	"io"
	"unicode"
)

func _MakeFromIO(reader io.RuneReader) (interpreteter.Token, error) {
	char, _, err := reader.ReadRune()
	if err != nil {
		return _Impl{}, err
	}

	return _MakeFromRune(char)
}

func _MakeFromRune(char rune) (interpreteter.Token, error) {
	token := _Impl{}
	if unicode.IsDigit(char) {
		token._Type = interpreteter.INTEGER
		token._Value = int(char) - int('0')
		return token, nil
	}

	if char == '+' {
		token._Type = interpreteter.PLUS
		token._Value = int(char)
		return token, nil
	}

	if char == '\n' {
		token._Type = interpreteter.NEWLINE
		token._Value = int(char)
		return token, nil
	}

	return token, fmt.Errorf("unknown character '%s'", string(char))
}
