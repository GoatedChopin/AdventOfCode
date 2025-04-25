package main

import (
	"fmt"

	"container/list"
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

func FrontalWhiteElephant(n int) int {
	left := list.New()
	right := list.New()

	for i := 1; i <= n; i++ {
		if i <= n/2 {
			left.PushBack(i)
		} else {
			right.PushFront(i)
		}
	}

	for left.Len() > 0 && right.Len() > 0 {
		if left.Len() > right.Len() {
			left.Remove(left.Back())
		} else {
			right.Remove(right.Back())
		}

		// Rotate
		right.PushFront(left.Remove(left.Front()))
		left.PushBack(right.Remove(right.Back()))
	}

	if left.Len() > 0 {
		return left.Front().Value.(int)
	}
	return right.Front().Value.(int)
}

func main() {
	fmt.Print("Starting day 15\n")
	part1 := WhiteElephant(3014603)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := FrontalWhiteElephant(3014603)
	fmt.Printf("Part 2: %v\n", part2)
}
