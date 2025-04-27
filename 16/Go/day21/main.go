package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func Init(r *map[string]int) {
	(*r)["a"] = 0
	(*r)["b"] = 0
	(*r)["c"] = 0
	(*r)["d"] = 0
}

func getOrLookup(s string, r *map[string]int) int {
	v, ok := (*r)[s]
	if ok {
		return v
	}
	c, err := strconv.Atoi(s)
	if err != nil {
		panic("BAD VAL AT getOrLookup")
	}
	return c
}

func getInt(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic("BAD INT VAL")
	}
	return i
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
		a, b := instruction[1], instruction[2]
		v1, ok1 := (*r)[a]
		_, ok2 := (*r)[b]
		if ok1 && ok2 {
			(*r)[b] = v1
		} else if ok2 {
			value, err := strconv.Atoi(a)
			if err != nil {
				panic("BAD INT VALUE AT cpy")
			}
			(*r)[b] = value
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
	return string(r)
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
