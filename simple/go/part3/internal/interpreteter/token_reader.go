package interpreteter

type Reader interface {
	Current() Token
	Next() error
}
