package expr

import (
	"fmt"
	"interpreteter/internal/interpreteter"
)

func _CollectTokens(reader interpreteter.TokenReader, types []string) ([]interpreteter.Token, error) {
	tokens := make([]interpreteter.Token, len(types))

	for i := 0; i < len(tokens); i++ {
		token, err := reader.Read()
		if err != nil {
			return nil, err
		}

		if types[i] != token.Type() {
			return nil, fmt.Errorf("unexpected token %s", token.Type())
		}

		tokens[i] = token
	}

	return tokens, nil
}
