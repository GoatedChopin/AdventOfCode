package main

import (
	"fmt"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

type seq struct {
	s        string
	hypernet bool
}

func abba(s string) bool {
	chars := make(map[string]bool)
	arr := strings.Split(s, "")
	for i := 0; i < len(arr)/2; i++ {
		chars[arr[i]] = true
		if arr[i] != arr[len(arr)-(i+1)] {
			return false
		}
	}
	return len(chars) == 2
}

func hasAbba(s string) bool {
	var a, b, c rune
	for i, d := range s {
		if i == 0 {
			a = d
			// fmt.Printf("a -> %c%c%c%c\n", a, b, c, d)
		} else if i == 1 {
			b = d
			// fmt.Printf("b -> %c%c%c%c\n", a, b, c, d)
		} else if i == 2 {
			c = d
			// fmt.Printf("c -> %c%c%c%c\n", a, b, c, d)
		} else if i > 2 {
			// fmt.Printf("d -> %c%c%c%c\n", a, b, c, d)
			if a == d && b == c && a != b {
				return true
			}
			a = b
			b = c
			c = d
		}
	}
	return false
}

func validIpv7(s string) bool {
	var ranges []seq
	ind := 0
	current := ""
	for c := range strings.SplitSeq(s, "") {
		if c == "[" {
			ind++
			ranges = append(ranges, seq{current, false})
			current = ""
			ind++
			continue
		} else if c == "]" {
			ind++
			ranges = append(ranges, seq{current, true})
			current = ""
			ind++
			continue
		}
		current += c
	}
	fmt.Printf("%v\n", ranges)
	total := 0
	for _, r := range ranges {
		ab := hasAbba(r.s)
		if r.hypernet && ab {
			return false
		} else if ab {
			total++
		}
	}
	return total > 0
}

func ipv7Count(lines []string) int {
	total := 0
	for _, line := range lines {
		if validIpv7(line) {
			total++
		}
	}
	return total
}

func main() {
	fmt.Print("Starting day 7\n")
	inputs := adv.GetInput("7", true, "\n", true)
	part1 := ipv7Count(inputs)
	fmt.Printf("%v\n", part1)
}
