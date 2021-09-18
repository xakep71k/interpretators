package interpreteter

type Lexer interface {
	Current() Token
	Next() error
}
