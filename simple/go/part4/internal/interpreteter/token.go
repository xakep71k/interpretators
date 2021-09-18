package interpreteter

const (
	INTEGER = "0-9"
	MUL     = "*"
	DIV     = "/"
	NEWLINE = "\\n"
)

type Token interface {
	Type() string
	Value() int
}

type TokenFactory interface {
	NewToken(value int, kind string) Token
}
