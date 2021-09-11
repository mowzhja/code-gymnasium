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
	double := len(number)%2 == 0

	for _, r := range number {
		n := int(r - '0')

		if double {
			n = timesTwo(n)
		}

		sum += n
		double = !double
	}

	return sum%10 == 0
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
