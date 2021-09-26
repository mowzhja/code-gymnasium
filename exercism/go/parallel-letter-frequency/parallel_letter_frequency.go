package letter

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

func ConcurrentFrequency(strings []string) FreqMap {
	freqChannel := make(chan FreqMap)
	defer close(freqChannel)

	for _, s := range strings {
		go func(s string) {
			freqChannel <- Frequency(s)
		}(s)
	}

	globalFreqMap := FreqMap{}

	for range strings { // neat trick
		for letter, frequency := range <-freqChannel {
			globalFreqMap[letter] += frequency
		}
	}

	return globalFreqMap
}
