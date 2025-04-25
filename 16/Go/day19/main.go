package main

import (
	"fmt"
)

func WhiteElephant(i int) int {
	p := 0
	players := i
	elves := make([]int, i)
	for e := range elves {
		elves[e] = 1
	}
	for players > 1 {
		np := p + 1
		if np >= i {
			np = 0
		}
		for elves[np] == 0 && np != p {
			np++
			if np >= i {
				np = 0
			}
		}
		if np == p {
			break
		}
		elves[p] += elves[np]
		elves[np] = 0
		// fmt.Printf("%v\n", elves)
		players--
		p++
		if p >= i {
			p = 0
		}
		for elves[p] == 0 {
			p++
			if p >= i {
				p = 0
			}
		}
	}
	return p + 1
}

func FrontalWhiteElephant(i int) int {
	p := 0
	players := i
	elves := make([]int, i)
	for e := range elves {
		elves[e] = 1
	}
	for players > 1 {
		passes := (players / 2) - 1
		np := p + 1
		if np >= i {
			np -= i
		}
		for (elves[np] == 0 || passes > 0) && np != p {
			if np == p {
				panic("Why the fuck")
			}
			if elves[np] == 1 {
				passes--
			}
			np++
			if np >= i {
				np = 0
			}
		}
		if np == p {
			break
		}
		elves[p] += elves[np]
		elves[np] = 0
		if players%500 == 0 {
			fmt.Printf("%v -> %v -> %v\n", players, p, np)
		}
		players--
		p++
		if p >= i {
			p = 0
		}
		for elves[p] == 0 {
			p++
			if p >= i {
				p = 0
			}
		}
	}
	return p + 1
}

func main() {
	fmt.Print("Starting day 15\n")
	part1 := WhiteElephant(3014603)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := FrontalWhiteElephant(3014603)
	fmt.Printf("Part 2: %v\n", part2)
}
