package token

type _Impl struct {
	_Type  string
	_Value int
}

func (t _Impl) Type() string {
	return t._Type
}

func (t _Impl) Value() int {
	return t._Value
}
