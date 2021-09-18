package expr

import (
	"bytes"
	"interpreteter/internal/interpreteter"
	"interpreteter/internal/lexer"
	"interpreteter/internal/token"
)

func Calc(buffer *bytes.Buffer) (int, error) {
	tokenFactory := token.NewFactory()
	lexer, err := lexer.New(tokenFactory, buffer)
	if err != nil {
		return 0, err
	}
	eater := interpreteter.NewEater(lexer)
	return expr(eater)
}
