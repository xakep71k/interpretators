package interpreteter

const (
	INTEGER = "INTEGER"
	PLUS    = "PLUS"
	NEWLINE = "NEWLINE"
)

type Token interface {
	Type() string
	Value() int
}

type TokenReader interface {
	Read() (Token, error)
}
