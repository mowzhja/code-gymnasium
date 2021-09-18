package cipher

import (
	"strings"
)

type Caesar struct {
	shift int32
}

func NewCaesar() Cipher {
	return Caesar{shift: 3} // value used in tests
}

func (c Caesar) Encode(pt string) string {
	var ct strings.Builder

	clean_pt := clean_input(pt)
	for _, r := range clean_pt {
		delta := ((r - 'a') + c.shift) % 26
		ct.WriteRune('a' + delta)
	}

	return ct.String()
}

func (c Caesar) Decode(ct string) string {
	var pt strings.Builder

	clean_ct := clean_input(ct)
	for _, r := range clean_ct {
		delta := ((r - 'a') - c.shift) % 26
		pt.WriteRune('a' + delta)
	}

	return pt.String()
}

func NewShift(new_shift int) Cipher {
	if new_shift <= -26 || new_shift == 0 || new_shift >= 26 {
		return nil
	}

	return Caesar{shift: int32(new_shift)}
}

func NewVigenere(key string) Cipher {
	return nil
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
