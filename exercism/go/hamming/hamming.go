package hamming

import "errors"

func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("the strings must have the same length")
	}

	distance := 0
	for i := range a {
		if a[i] != b[i] {
			distance += 1
		}
	}

	return distance, nil
}
