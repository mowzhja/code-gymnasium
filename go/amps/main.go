package main

import "fmt"

func main() {
	var perms [][]int
	set := []int{0, 1, 2, 3, 4}
	in, max := 0, 0

	for p := make([]int, len(set)); p[0] < len(p); nextPerm(p) {
		perms = append(perms, getPerm(set, p))
	}

	for _, p := range perms {
		// pass the permutation through each amplifier
		for a := 0; a < 5; a++ {
			out := pass_through_amp(p[a], in)
			in = out
			if out > max {
				max = out
			}
		}
	}

	fmt.Println("Maximum thrust =>", max)
}
