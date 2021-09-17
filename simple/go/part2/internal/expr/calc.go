package expr

import (
	"bytes"
	"interpreteter/internal/token"
)

func Calc(buffer *bytes.Buffer) (int, error) {
	reader, err := token.NewReader(buffer)
	if err != nil {
		return 0, err
	}
	eater := token.NewEater(reader)
	return expr(eater)
}
