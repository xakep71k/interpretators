package interpreteter

const (
	INTEGER = "0-9"
	PLUS    = "+"
	MINUS   = "-"
	NEWLINE = "\\n"
)

type Token interface {
	Type() string
	Value() int
}
