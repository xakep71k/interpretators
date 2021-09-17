package token

func _IsSpace(ch rune) bool {
	switch ch {
	case '\t', ' ':
		return true
	}
	return false
}
