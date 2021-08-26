package expr

import (
	"interpreteter/internal/interpreteter"
)

func Calc(reader interpreteter.TokenReader) (int, error) {
	expectedSequenceTokens := []string{
		interpreteter.INTEGER,
		interpreteter.PLUS,
		interpreteter.INTEGER,
		interpreteter.NEWLINE,
	}
	tokens, err := _CollectTokens(reader, expectedSequenceTokens)
	if err != nil {
		return 0, err
	}

	left := tokens[0]
	right := tokens[2]

	return left.Value() + right.Value(), nil
}
