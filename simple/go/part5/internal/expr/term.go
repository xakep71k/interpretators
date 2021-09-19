package expr

import "interpreteter/internal/interpreteter"

func term(eater interpreteter.Eater) (int, error) {
	result, err := factor(eater)
	if err != nil {
		return 0, err
	}

	const MUL = interpreteter.MUL
	const DIV = interpreteter.DIV
	for tkn := eater.Current(); tkn.Type() == MUL || tkn.Type() == DIV; tkn = eater.Current() {
		if tkn.Type() == MUL {
			if err := eater.Eat(MUL); err != nil {
				return 0, err
			}

			nextResult, err := factor(eater)
			if err != nil {
				return 0, err
			}
			result = result * nextResult
		} else if tkn.Type() == DIV {
			if err := eater.Eat(DIV); err != nil {
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
