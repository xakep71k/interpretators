package expr

import (
	"interpreteter/internal/interpreteter"
)

func Calc(reader interpreteter.TokenReader) (int, error) {
	expectedSequenceTypes := []string{
		interpreteter.INTEGER,
		interpreteter.PLUS,
		interpreteter.INTEGER,
		interpreteter.NEWLINE,
	}
	tokens, err := _CollectTokens(reader, expectedSequenceTypes)
	if err != nil {
		return 0, err
	}

	left := tokens[0]
	right := tokens[2]

	return left.Value() + right.Value(), nil
}
