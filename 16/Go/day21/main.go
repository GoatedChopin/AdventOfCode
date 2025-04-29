package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func getInt(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic("BAD INT VAL")
	}
	return i
}

func Rotate(s *[]string, i int) {
	for i > len(*s) {
		i -= len(*s)
	}
	if i == 0 {
		return
	} else if i > 0 {
		ns := make([]string, len(*s))
		for si := range i {
			ns[si] = (*s)[(len(*s) - i + si)]
		}
		for si := range len(*s) - i {
			ns[i+si] = (*s)[si]
		}
		copy(*s, ns)
	} else {
		i = -i
		ns := make([]string, len(*s))
		for si := range len(*s) - i {
			ns[si] = (*s)[i+si]
		}
		for si := range i {
			ns[len(*s)-i+si] = (*s)[si]
		}
		copy(*s, ns)
	}
}

func Execute(instruction []string, r *[]string) {
	switch instruction[0] {
	case "swap":
		if instruction[1] == "letter" {
			ai, bi := -1, -1
			a, b := instruction[2], instruction[5]
			for i, c := range *r {
				if c == a {
					ai = i
				} else if c == b {
					bi = i
				}
				if ai != -1 && bi != -1 {
					(*r)[ai], (*r)[bi] = (*r)[bi], (*r)[ai]
				}
			}
		} else if instruction[1] == "position" {
			a, b := getInt(instruction[2]), getInt(instruction[5])
			(*r)[a], (*r)[b] = (*r)[b], (*r)[a]
		} else {
			panic("BAD STR VAL AT swap")
		}
	case "move":
		a, b := getInt(instruction[2]), getInt(instruction[5])
		if b < a {
			a, b = b, a
		}
		aval := (*r)[a]
		*r = slices.Concat((*r)[:a], (*r)[a+1:b], []string{aval}, (*r)[b+1:])
	case "reverse":
		a, b := getInt(instruction[2]), getInt(instruction[4])
		if b < a {
			a, b = b, a
		}
		slices.Reverse((*r)[a:b])
	case "rotate":
		if instruction[1] == "based" {
			char := instruction[6]
			ind := 0
			for i, c := range *r {
				if c == char {
					ind = i
				}
			}
			Rotate(r, ind)
		} else {
			dir, amount := instruction[1], getInt(instruction[2])
			if dir == "left" {
				amount = -amount
			}
			Rotate(r, amount)
		}
	default:
		return
	}
}

func Follow(splitInstructions [][]string, s string) string {
	r := strings.Split(s, "")
	i := 0
	for i < len(splitInstructions) {
		// fmt.Printf("%v -> %v\n", instructions[i], Registers)
		args := splitInstructions[i]
		Execute(args, &r)
		i++
	}
	return strings.Join(r, "")
}

func Scramble(instructions []string, input string) string {
	splitInstructions := make([][]string, len(instructions))
	for i, s := range instructions {
		args := strings.Split(s, " ")
		splitInstructions[i] = args
	}
	s := Follow(splitInstructions, input)
	return s
}

func main() {
	fmt.Print("Starting day 21\n")
	inputs := adv.GetInput("21", true, "\n", true)
	part1 := Scramble(inputs, "abcdefgh")
	fmt.Printf("Part 1: %v\n", part1)
}
