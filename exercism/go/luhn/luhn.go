package luhn

import (
	sc "strconv"
	s "strings"
)

// Validates a number according to the Luhn algorithm (https://en.wikipedia.org/wiki/Luhn_algorithm)
func Valid(number string) bool {
	spacelessNumber := s.Replace(number, " ", "", -1)
	if !ok(spacelessNumber) {
		return false
	}
	spacelessNumber = reverse(spacelessNumber) // makes the next step easier

	doubledNumber := doubleDigits(spacelessNumber)

	sumDigits := sum(doubledNumber)
	return sumDigits%10 == 0
}

// Creates the string with all the doubled digits.
func doubleDigits(initNumber string) string {
	var doubleSb s.Builder

	for i, ch := range initNumber {
		if i%2 != 0 {
			doubleSb.WriteRune(double(ch))
		} else {
			doubleSb.WriteRune(ch)
		}
	}

	return doubleSb.String()
}

// Double a single character (rune), as long as that rune represents a digit.
func double(ch rune) rune {
	n := int(ch - '0') // good ol' C-style trick

	m := 2 * n
	if m > 9 {
		m -= 9
	}

	return rune(m + '0')
}

// Sums all the digits making up a string (supposedly a number).
func sum(number string) int {
	sum := 0

	for _, ch := range number {
		n := int(ch - '0')
		sum += n
	}

	return sum
}

// Reverses a string char by char.
func reverse(str string) string {
	var rev s.Builder

	for i := len(str) - 1; i >= 0; i-- {
		rev.WriteByte(str[i])
	}

	return rev.String()
}

// Checks if the number string received in input is valid (aka has length greater than 1 and is only made up of digits).
func ok(number string) bool {
	if len(number) <= 1 {
		return false
	}
	if _, err := sc.ParseInt(number, 10, 64); err != nil {
		return false
	}

	return true
}
