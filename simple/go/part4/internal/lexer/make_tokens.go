package lexer

import (
	"bytes"
	"interpreteter/internal/interpreteter"
	"strconv"
	"strings"
	"unicode"
)

func (impl *_Impl) _MakeFromBuffer(input *bytes.Buffer) (interpreteter.Token, error) {
	for {
		ch, _, err := input.ReadRune()
		if err != nil {
			return nil, err
		}
		if _IsSpace(ch) {
			continue
		}

		if unicode.IsDigit(ch) {
			if err := input.UnreadRune(); err != nil {
				return nil, err
			}
			return impl._MakeIntegerFromBuffer(input)
		}

		switch ch {
		case '*':
			return impl.tokenFactory.NewToken(int(ch), interpreteter.MUL), nil
		case '/':
			return impl.tokenFactory.NewToken(int(ch), interpreteter.DIV), nil
		case '\n':
			return impl.tokenFactory.NewToken(int(ch), interpreteter.NEWLINE), nil
		}
		return nil, interpreteter.ErrInvalidSyntax
	}
}

func (impl *_Impl) _MakeIntegerFromBuffer(input *bytes.Buffer) (interpreteter.Token, error) {
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

	return impl.tokenFactory.NewToken(int(value), interpreteter.INTEGER), nil
}
