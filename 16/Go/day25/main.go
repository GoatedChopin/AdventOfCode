package main

import (
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

var Registers map[string]int = make(map[string]int)

func Init() {
	Registers["a"] = 0
	Registers["b"] = 0
	Registers["c"] = 0
	Registers["d"] = 0
}

func getOrLookup(s string) int {
	v, ok := Registers[s]
	if ok {
		return v
	}
	c, err := strconv.Atoi(s)
	if err != nil {
		panic("BAD VAL AT getOrLookup")
	}
	return c
}

func Execute(instruction []string) (int, int) {
	switch instruction[0] {
	case "inc":
		Registers[instruction[1]]++
	case "dec":
		Registers[instruction[1]]--
	case "jnz":
		// Check that registers has instruction[1] and that it's not 0 OR that it's a nonzero number
		value, ok := Registers[instruction[1]]
		if (ok && value != 0) || (!ok && instruction[1] != "0") {
			jump, err := strconv.Atoi(instruction[2])
			if err != nil {
				panic("BAD INT VALUE AT jnz")
			}
			return jump, -1
		}
	case "cpy":
		a, b := instruction[1], instruction[2]
		v1, ok1 := Registers[a]
		_, ok2 := Registers[b]
		if ok1 && ok2 {
			Registers[b] = v1
		} else if ok2 {
			value, err := strconv.Atoi(a)
			if err != nil {
				panic("BAD INT VALUE AT cpy")
			}
			Registers[b] = value
		}
	case "out":
		v := getOrLookup(instruction[1])
		return 0, v
	default:
		return 0, -1
	}
	return 0, -1
}

func Follow(ch chan int, aVal int, instructions []string, part int) <-chan int {
	ch = make(chan int)
	func(ch chan int) {
		Init()
		Registers["a"] = aVal
		splitInstructions := make([][]string, len(instructions))
		for i, s := range instructions {
			args := strings.Split(s, " ")
			splitInstructions[i] = args
		}
		i := 0
		fmt.Printf("Running instructions: %v\n", instructions)
		for i < len(instructions) {
			// fmt.Printf("%v -> %v\n", instructions[i], Registers)
			args := splitInstructions[i]
			jump, out := Execute(args)
			if args[0] == "out" {
				ch <- out
			}
			if jump == 0 {
				i++
			} else {
				i += jump
			}
		}
	}(ch)
	return ch
}

func main() {
	fmt.Print("Starting day 12\n")
	inputs := adv.GetInput("12", true, "\n", true)
	part1 := Follow(inputs, 0)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := Follow(inputs, 1)
	fmt.Printf("Part 2: %v\n", part2)
}
