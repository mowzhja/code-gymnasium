package main

import (
	"bytes"
	"exec"
	"strconv"
)

// from https://stackoverflow.com/questions/30226438/generate-all-permutations-in-go
func nextPerm(p []int) {
	for i := len(p) - 1; i >= 0; i-- {
		if i == 0 || p[i] < len(p)-i-1 {
			p[i]++
			return
		}
		p[i] = 0
	}
}

func getPerm(orig, p []int) []int {
	result := append([]int{}, orig...)
	for i, v := range p {
		// Python style!
		result[i], result[i+v] = result[i+v], result[i]
	}

	return result
}

func pass_through_amp(phase_setting int, in_signal int) int {
	var out bytes.Buffer

	cmd := exec.Command("path_to_interpeter", phase_setting, in_signal)
	cmd.Stdout = &out

	err := cmd.Run()
	if err != nil {
		panic(err)
	}

	out_thrust, err := strconv.Atoi(out.String())
	if err != nil {
		panic(err)
	}

	return out_thrust
}
