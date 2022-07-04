package accumulate

func Accumulate(strings []string, f func(string) string) (res []string) {
	var accumulated []string

	for _, str := range strings {
		accumulated = append(accumulated, f(str))
	}

	return accumulated
}
