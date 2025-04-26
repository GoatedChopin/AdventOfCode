package main

import (
	"fmt"
	"strings"
	"time"
)

func Hash(s string) string {
	var builder strings.Builder
	chars := []rune(s)
	// fmt.Printf("Hashing ")
	for i := 0; i < len(chars)-1; i += 2 {
		// fmt.Printf("%c%c  ", chars[i], chars[i+1])
		if chars[i] == chars[i+1] {
			builder.WriteByte('1')
		} else {
			builder.WriteByte('0')
		}
	}
	h := builder.String()
	if len(h)%2 == 0 {
		// fmt.Printf("%v\t%v\n", h, len(h))
		return Hash(h)
	} else {
		return h
	}
}

func reverse(s string) string {
	chars := []rune(s)
	for i, j := 0, len(chars)-1; i < j; i, j = i+1, j-1 {
		chars[i], chars[j] = chars[j], chars[i]
	}
	return string(chars)
}

func flip(s string) string {
	chars := []rune(s)
	for i, c := range s {
		if c == '1' {
			chars[i] = '0'
		} else {
			chars[i] = '1'
		}
	}
	return string(chars)
}

func Dragon(input string, size int) string {
	percent := (float32(len(input)) / float32(size)) * 100.0
	fmt.Printf("%v\t%v percent complete\n", time.Now().Format(time.ANSIC), percent)
	if len(input) >= size {
		return Hash(input[:size])
	}
	next := reverse(input)
	flipped := flip(next)
	return Dragon(input+"0"+flipped, size)
}

func main() {
	fmt.Print("Starting day 16\n")
	input := "11101000110010100"
	part1 := Dragon(input, 272)
	fmt.Printf("Part1: %v\n", part1)
	part2 := Dragon(input, 35651584)
	fmt.Printf("Part2: %v\n", part2)
}
