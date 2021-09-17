package expr

import (
	"interpreteter/internal/interpreteter"
	"interpreteter/internal/token"
	"io"
)

func expr(eater token.Eater) (int, error) {
	left := eater.Current()
	if err := eater.Eat(interpreteter.INTEGER); err != nil {
		return 0, err
	}

	if err := eater.Eat(interpreteter.PLUS); err != nil {
		return 0, err
	}

	right := eater.Current()
	if err := eater.Eat(interpreteter.INTEGER); err != nil {
		return 0, err
	}

	if err := eater.Eat(interpreteter.NEWLINE); err != nil && err != io.EOF {
		return 0, err
	}

	return left.Value() + right.Value(), nil
}
