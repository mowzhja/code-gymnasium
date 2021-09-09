package luhn

import (
	"strconv"
	"strings"
)

// Validates a number according to the Luhn algorithm (https://en.wikipedia.org/wiki/Luhn_algorithm)
func Valid(number string) bool {
	number = strings.ReplaceAll(number, " ", "")
	if !ok(number) {
		return false
	}

	sum := 0
	isEvenLength := len(number)%2 == 0

	for i, r := range number {
		sum += digit(r, i, isEvenLength)
	}

	return sum%10 == 0
}

// Double a single character (rune), as long as that rune represents a digit.
func digit(r rune, i int, isEvenLength bool) int {
	n := int(r - '0') // good ol' C-style trick

	if isEvenLength {
		if i%2 == 0 {
			return timesTwo(n)
		}
	} else {
		if i%2 != 0 {
			return timesTwo(n)
		}
	}

	return n
}

// Doubles the input and subtracts 9 if necessary.
func timesTwo(n int) int {
	n *= 2

	if n > 9 {
		return n - 9
	}
	return n
}

// Checks if the number string received in input is valid (aka is length greater than 1 and is only made up of digits).
func ok(number string) bool {
	if len(number) <= 1 {
		return false
	}
	if _, err := strconv.ParseInt(number, 10, 64); err != nil {
		return false
	}

	return true
}
