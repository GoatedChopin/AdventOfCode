package main

import (
	"fmt"
	"strings"

	"github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

var Buttons = [][]string{
	{"1", "2", "3"},
	{"4", "5", "6"},
	{"7", "8", "9"},
}

var TwoButtons = [][]string{
	{"!", "!", "1", "!", "!"},
	{"!", "2", "3", "4", "!"},
	{"5", "6", "7", "8", "9"},
	{"!", "A", "B", "C", "!"},
	{"!", "!", "D", "!", "!"},
}

var Dirs = map[string][]int{
	"R": {0, 1},  // North
	"L": {0, -1}, // East
	"U": {-1, 0}, // South
	"D": {1, 0},  // West
}

func Move(position []int, direction []int, lowBound int, upBound int) []int {
	newPosition := make([]int, len(position))
	copy(newPosition, position)
	for i, d := range direction {
		next := newPosition[i] + d
		if next < lowBound {
			continue
		} else if next > upBound {
			continue
		}
		newPosition[i] += d
	}
	return newPosition
}

func Pinpad(input []string, part uint) string {
	out := ""
	position := []int{}
	buttons := [][]string{}
	lowBound := 0
	upBound := 0
	if part == 1 {
		buttons = Buttons
		position = []int{1, 1}
		upBound = 2
	} else {
		buttons = TwoButtons
		position = []int{2, 0}
		upBound = 4
	}
	for _, line := range input {
		for dir := range strings.SplitSeq(line, "") {
			vector := Dirs[dir]
			fmt.Printf("\t%v + %v => ", position, vector)
			newPosition := Move(position, vector, lowBound, upBound)
			if buttons[newPosition[0]][newPosition[1]] != "!" {
				position = newPosition
			} else {
				fmt.Printf("Skipping move, position still: %v\n", position)
				continue
			}
			fmt.Printf("%v\n", position)
		}
		fmt.Printf("%v -> %v\t%v\n", position, buttons[position[0]][position[1]], line)
		out += buttons[position[0]][position[1]]
	}
	return out
}

func main() {
	input := adv.GetInput("2", true, "\n", true)
	answer := Pinpad(input, 1)
	fmt.Printf("Answer 1: %v\n\n", answer)
	answerTwo := Pinpad(input, 2)
	fmt.Printf("Answer 2: %v\n", answerTwo)
}
