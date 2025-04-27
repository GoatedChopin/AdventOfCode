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
		change := 1
		Registers[instruction[1]] += change
	case "dec":
		change := -1
		Registers[instruction[1]] += change
	case "tgl":
		v1, ok := Registers[instruction[1]]
		if ok {
			return 0, v1
		}
		i, err := strconv.Atoi(instruction[1])
		if err != nil {
			panic("BAD INT VALUE AT tgl")
		}
		return 0, i
	case "jnz":
		v1 := getOrLookup(instruction[1])
		v2 := getOrLookup(instruction[2])
		if v1 != 0 {
			return v2, -1
		} else {
			return 0, -1
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
		return 0, -1
	}
	return 0, -1
}

func toggleInstruction(instr []string) []string {
	switch instr[0] {
	case "inc":
		return []string{"dec", instr[1]}
	case "dec", "tgl":
		return []string{"inc", instr[1]}
	case "jnz":
		return []string{"cpy", instr[1], instr[2]}
	case "cpy":
		return []string{"jnz", instr[1], instr[2]}
	default:
		return instr
	}
}

func Follow(instructions []string, partI int) int {
	Init()
	Registers["a"] = partI
	i := 0
	splitInstructions := make([][]string, len(instructions))
	for i, s := range instructions {
		args := strings.Split(s, " ")
		splitInstructions[i] = args
	}
	fmt.Printf("Running instructions: %v\n", instructions)
	for i < len(instructions) {
		args := splitInstructions[i]
		// fmt.Printf("%v\t%v\t%d\n", Registers, args, i)
		jump, tgl := Execute(args)
		if tgl > -1 && tgl+i < len(splitInstructions) {
			splitInstructions[i+tgl] = toggleInstruction(splitInstructions[i+tgl])
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
	part1 := Follow(inputs, 7)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := Follow(inputs, 12)
	fmt.Printf("Part 2: %v\n", part2)
}
