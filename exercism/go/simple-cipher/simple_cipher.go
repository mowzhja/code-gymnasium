package cipher

import (
	"strings"
)

type Caesar struct {
	shift int32
}

func NewCaesar() Caesar {
	return Caesar{shift: 5}
}

func (c *Caesar) Encode(pt string) string {
	var ct strings.Builder

	clean_pt := clean_input(pt)
	for _, r := range clean_pt {
		delta := ((r + c.shift) % 97) % 26 // 'a' == 97
		ct.WriteRune('a' + delta)
	}

	return ct.String()
}

func (c *Caesar) Decode(ct string) string {
	clean_ct := clean_input(ct)
}

// Transforms the input string into a lowercase one with no punctuation.
func clean_input(s string) string {
	// obtained from python -> string.punctuation
	invalid := "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"

	s = strings.ReplaceAll(s, " ", "")
	s = strings.ToLower(s)
	for _, r := range invalid {
		s = strings.ReplaceAll(s, string(r), "")
	}

	return s
}
