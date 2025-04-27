package main

import (
	"fmt"
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

func Execute(instruction []string, r *map[string]int) (int, int) {
	switch instruction[0] {
	case "inc":
		(*r)[instruction[1]]++
	case "dec":
		(*r)[instruction[1]]--
	case "jnz":
		// Check that (*r) has instruction[1] and that it's not 0 OR that it's a nonzero number
		value, ok := (*r)[instruction[1]]
		if (ok && value != 0) || (!ok && instruction[1] != "0") {
			jump, err := strconv.Atoi(instruction[2])
			if err != nil {
				panic("BAD INT VALUE AT jnz")
			}
			return jump, -1
		}
	case "cpy":
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
	case "out":
		v := getOrLookup(instruction[1], r)
		return 0, v
	default:
		return 0, -1
	}
	return 0, -1
}

func Follow(aVal int, splitInstructions [][]string) <-chan int {
	var registers map[string]int = make(map[string]int)
	ch := make(chan int)
	go func(ch chan int) {
		defer close(ch)
		Init(&registers)
		registers["a"] = aVal
		i := 0
		for i < len(splitInstructions) {
			// fmt.Printf("%v -> %v\n", instructions[i], Registers)
			args := splitInstructions[i]
			jump, out := Execute(args, &registers)
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

func AlternatingSignal(s []int) bool {
	for i := range len(s) - 1 {
		b := s[i]
		opp := 0
		if b == 0 {
			opp = 1
		} else if b == 1 {
			opp = 0
		} else {
			return false
		}
		if s[i+1] != opp {
			return false
		}
	}
	return true
}

func FindLowestSignal(instructions []string) int {
	l := 0
	splitInstructions := make([][]string, len(instructions))
	for i, s := range instructions {
		args := strings.Split(s, " ")
		splitInstructions[i] = args
	}
	for {
		buff := make([]int, 0)
		for ni := range Follow(l, splitInstructions) {
			buff = append(buff, ni)
			if len(buff) >= 20 {
				break
			}
		}
		if AlternatingSignal(buff) {
			break
		}
		l++
	}
	return l
}

func main() {
	fmt.Print("Starting day 25\n")
	inputs := adv.GetInput("25", true, "\n", true)
	part1 := FindLowestSignal(inputs)
	fmt.Printf("Part 1: %v\n", part1)
}
