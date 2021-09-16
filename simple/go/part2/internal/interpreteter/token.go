package interpreteter

const (
	INTEGER = "0-9"
	PLUS    = "+"
	NEWLINE = "\\n"
)

type Token interface {
	Type() string
	Value() int
}

type TokenReader interface {
	Read() (Token, error)
}
