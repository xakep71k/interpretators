package expr

import (
	"interpreteter/internal/interpreteter"
)

func expr(eater interpreteter.Eater) (int, error) {
	result, err := factor(eater)
	if err != nil {
		return 0, err
	}

	const MUL = interpreteter.MUL
	const MINUS = interpreteter.DIV
	for tkn := eater.Current(); tkn.Type() == MUL || tkn.Type() == MINUS; tkn = eater.Current() {
		if tkn.Type() == MUL {
			if err := eater.Eat(MUL); err != nil {
				return 0, err
			}

			nextResult, err := factor(eater)
			if err != nil {
				return 0, err
			}
			result = result * nextResult
		} else if tkn.Type() == MINUS {
			if err := eater.Eat(MINUS); err != nil {
				return 0, err
			}

			nextResult, err := factor(eater)
			if err != nil {
				return 0, err
			}
			result = result / nextResult
		}
	}

	return result, nil
}
