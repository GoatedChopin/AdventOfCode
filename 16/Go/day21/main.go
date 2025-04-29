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
					return
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
		aval := (*r)[a]
		nr := make([]string, len(*r))
		i := 0
		ni := 0
		skipped := false
		for i < len(*r) && ni < len(nr) {
			if i == a && !skipped {
				i++
				skipped = true
				continue
			} else if ni == b {
				nr[ni] = aval
				ni++
				continue
			}
			nr[ni] = (*r)[i]
			i++
			ni++
		}
		if nr[len(nr)-1] == "" {
			nr[len(nr)-1] = (*r)[len(nr)-1]
		}
		nr[b] = aval
		copy(*r, nr)
	case "reverse":
		a, b := getInt(instruction[2]), getInt(instruction[4])
		if b < a {
			a, b = b, a
		}
		slices.Reverse((*r)[a : b+1])
	case "rotate":
		if instruction[1] == "based" {
			char := instruction[6]
			ind := 0
			for i, c := range *r {
				if c == char {
					ind = i
				}
			}
			steps := 1 + ind
			if ind >= 4 {
				steps++
			}
			Rotate(r, steps)
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
