package main

import (
	"fmt"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func RenderTiles(t [][]bool) {
	for r := range len(t) {
		for c := range len(t[r]) {
			if t[r][c] {
				fmt.Print(".")
			} else {
				fmt.Print("^")
			}
		}
		fmt.Print("\n")
	}
}

func isTrap(left, center, right bool) bool {
	return (left && center && !right) ||
		(!left && center && right) ||
		(left && !center && !right) ||
		(!left && !center && right)
}

func BuildTiles(s string, i int) [][]bool {
	m := make([][]bool, i)
	m[0] = make([]bool, len(s))
	for i, c := range s {
		if c == '^' {
			m[0][i] = false
		} else {
			m[0][i] = true
		}
	}
	for r := 1; r < i; r++ {
		m[r] = make([]bool, len(s))
		for i := range s {
			left, right := true, true
			if i > 0 {
				left = m[r-1][i-1]
			}
			if i < len(s)-1 {
				right = m[r-1][i+1]
			}
			m[r][i] = !isTrap(left, m[r-1][i], right)
		}
	}
	return m
}

func SafeTiles(s string, i int) int {
	safe := 0
	tiles := BuildTiles(s, i)
	for r := range i {
		for c := range len(s) {
			if tiles[r][c] {
				safe++
			}
		}
	}
	return safe
}

func main() {
	fmt.Print("Starting day 18\n")
	input := adv.GetInput("18", false, "", true)
	part1 := SafeTiles(input[0], 40)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := SafeTiles(input[0], 400000)
	fmt.Printf("Part 1: %v\n", part2)
}
