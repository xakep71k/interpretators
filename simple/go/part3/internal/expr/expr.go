package expr

import (
	"interpreteter/internal/interpreteter"
	"interpreteter/internal/token"
)

func expr(eater token.Eater) (int, error) {
	result, err := term(eater)
	if err != nil {
		return 0, err
	}

	const PLUS = interpreteter.PLUS
	const MINUS = interpreteter.MINUS
	for tkn := eater.Current(); tkn.Type() == PLUS || tkn.Type() == MINUS; tkn = eater.Current() {
		if tkn.Type() == PLUS {
			if err := eater.Eat(PLUS); err != nil {
				return 0, err
			}

			nextResult, err := term(eater)
			if err != nil {
				return 0, err
			}
			result = result + nextResult
		} else if tkn.Type() == MINUS {
			if err := eater.Eat(MINUS); err != nil {
				return 0, err
			}

			nextResult, err := term(eater)
			if err != nil {
				return 0, err
			}
			result = result - nextResult
		}
	}

	return result, nil
}
