package luhn

import (
	"strconv"
	"strings"
)

// Validates a number according to the Luhn algorithm (https://en.wikipedia.org/wiki/Luhn_algorithm)
func Valid(number string) bool {
	spacelessNumber := strings.ReplaceAll(number, " ", "")
	if !ok(spacelessNumber) {
		return false
	}
	doubledNumber := doubleDigits(spacelessNumber)

	luhnSum := sumDigits(doubledNumber)
	return luhnSum%10 == 0
}

// Creates the string with all the doubled digits.
func doubleDigits(spacelessNumber string) string {
	if len(spacelessNumber)%2 == 0 {
		return evenLengthDouble(spacelessNumber)
	}
	return oddLengthDouble(spacelessNumber)
}

// Takes care of digit doubling if the length of the original string is even.
func evenLengthDouble(spacelessNumber string) string {
	var doubleSb strings.Builder

	for i := 0; i < len(spacelessNumber); i++ {
		if i%2 == 0 {
			doubleSb.WriteByte(doubleChar(spacelessNumber[i]))
		} else {
			doubleSb.WriteByte(spacelessNumber[i])
		}
	}

	return doubleSb.String()
}

// Takes care of digit doubling if the length of the original string is odd.
func oddLengthDouble(spacelessNumber string) string {
	var doubleSb strings.Builder

	for i := 0; i < len(spacelessNumber); i++ {
		if i%2 != 0 {
			doubleSb.WriteByte(doubleChar(spacelessNumber[i]))
		} else {
			doubleSb.WriteByte(spacelessNumber[i])
		}
	}

	return doubleSb.String()
}

// Double a single character (rune), as long as that rune represents a digit.
func doubleChar(ch byte) byte {
	n := int(ch - '0') // good ol' C-style trick

	m := 2 * n
	if m > 9 {
		m -= 9
	}

	return byte(m + '0')
}

// Sums all the digits making up a string (supposedly a number).
func sumDigits(number string) int {
	sum := 0

	for _, ch := range number {
		n := int(ch - '0')
		sum += n
	}

	return sum
}

// Checks if the number string received in input is valid (aka has length greater than 1 and is only made up of digits).
func ok(number string) bool {
	if len(number) <= 1 {
		return false
	}
	if _, err := strconv.ParseInt(number, 10, 64); err != nil {
		return false
	}

	return true
}
