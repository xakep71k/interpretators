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

	opToken := eater.Current()
	if opToken.Type() == interpreteter.PLUS {
		if err := eater.Eat(interpreteter.PLUS); err != nil {
			return 0, err
		}
	} else {
		if err := eater.Eat(interpreteter.MINUS); err != nil {
			return 0, err
		}
	}

	right := eater.Current()
	if err := eater.Eat(interpreteter.INTEGER); err != nil {
		return 0, err
	}

	var result int
	if opToken.Type() == interpreteter.PLUS {
		result = left.Value() + right.Value()
	} else {
		result = left.Value() - right.Value()
	}

	if err := eater.Eat(interpreteter.NEWLINE); err != nil && err != io.EOF {
		return 0, err
	}
	return result, nil
}
