package expr

import (
	"interpreteter/internal/interpreteter"
)

func factor(eater interpreteter.Eater) (int, error) {
	tkn := eater.Current()
	err := eater.Eat(interpreteter.INTEGER)
	if err != nil {
		return 0, err
	}
	return tkn.Value(), nil
}
