package main

import (
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

var Registers map[string]int = make(map[string]int)
var Overrides map[int]int = make(map[int]int)

func Init() {
	Registers["a"] = 0
	Registers["b"] = 0
	Registers["c"] = 0
	Registers["d"] = 0
}

func Execute(instruction []string, tgl bool) (int, int) {
	switch instruction[0] {
	case "inc":
		change := 1
		if tgl {
			change = -change
		}
		Registers[instruction[1]] += change
	case "dec":
		change := -1
		if tgl {
			change = -change
		}
		Registers[instruction[1]] += change
	case "tgl":
		if tgl {
			return Execute([]string{"inc", instruction[1]}, false)
		}
		v1, ok := Registers[instruction[1]]
		if ok {
			if v1 > 0 {
				return 0, v1
			} else {
				return Execute([]string{"inc", instruction[1]}, false)
			}
		}
		i, err := strconv.Atoi(instruction[1])
		if err != nil {
			panic("BAD INT VALUE AT tgl")
		}
		return 0, i
	case "jnz":
		if tgl {
			return Execute([]string{"cpy", instruction[1], instruction[2]}, false)
		}
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
		if tgl {
			return Execute([]string{"jnz", instruction[1], instruction[2]}, false)
		}
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
		return 0, -1
	}
	return 0, -1
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
		jump, tgl := Execute(args, Overrides[i] > 0)
		if tgl > -1 {
			Overrides[i+tgl]++
		}
		if jump == 0 {
			i++
		} else {
			i += jump
		}
	}
	return Registers["a"]
}

func main() {
	fmt.Print("Starting day 23\n")
	inputs := adv.GetInput("23", true, "\n", true)
	part1 := Follow(inputs, 0)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := Follow(inputs, 1)
	fmt.Printf("Part 2: %v\n", part2)
}
