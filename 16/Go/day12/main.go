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

func Execute(instruction []string) int {
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
			return jump
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
	default:
		return 0
	}
	return 0
}

func Follow(instructions []string, part int) int {
	Init()
	if part == 1 {
		Registers["c"] = 1
	}
	i := 0
	fmt.Printf("Running instructions: %v\n", instructions)
	for i < len(instructions) {
		// fmt.Printf("%v -> %v\n", instructions[i], Registers)
		args := strings.Split(instructions[i], " ")
		jump := Execute(args)
		if jump == 0 {
			i++
		} else {
			i += jump
		}
	}
	return Registers["a"]
}

func main() {
	fmt.Print("Starting day 12\n")
	inputs := adv.GetInput("12", true, "\n", true)
	part1 := Follow(inputs, 0)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := Follow(inputs, 1)
	fmt.Printf("Part 2: %v\n", part2)
}
