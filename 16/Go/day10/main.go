package main

import (
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

type Robot struct {
	low, high       int
	hasLow, hasHigh bool
}

func (r *Robot) Push(i int) {
	if !r.hasLow && !r.hasHigh {
		r.low = i
		r.hasLow = true
	} else if r.hasLow && !r.hasHigh {
		if i < r.low {
			r.high = r.low
			r.low = i
		} else {
			r.high = i
		}
		r.hasHigh = true
	} else {
		panic("Push called when bot already has two values")
	}
}

func (r *Robot) Pop() (int, int) {
	low, high := r.low, r.high
	r.hasLow, r.hasHigh = false, false
	r.low, r.high = -1, -1
	return low, high
}

type Instruction struct {
	botID             int
	lowType, highType string
	lowID, highID     int
}

func mustAtoi(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return i
}

func SimulateRobots(lines []string, part int) int {
	robots := make(map[int]*Robot)
	outputs := make(map[int]int)
	var valueInstructions []string
	var actionQueue []Instruction

	// Separate value and bot instructions
	for _, line := range lines {
		parts := strings.Fields(line)
		if parts[0] == "value" {
			valueInstructions = append(valueInstructions, line)
		} else {
			botID := mustAtoi(parts[1])
			lowType := parts[5]
			lowID := mustAtoi(parts[6])
			highType := parts[10]
			highID := mustAtoi(parts[11])
			actionQueue = append(actionQueue, Instruction{botID, lowType, highType, lowID, highID})
			if robots[botID] == nil {
				robots[botID] = &Robot{}
			}
		}
	}

	// Apply initial value instructions
	for _, line := range valueInstructions {
		parts := strings.Fields(line)
		val := mustAtoi(parts[1])
		botID := mustAtoi(parts[5])
		if robots[botID] == nil {
			robots[botID] = &Robot{}
		}
		robots[botID].Push(val)
	}

	// Process action queue
	progress := true
	for progress {
		progress = false
		newQueue := []Instruction{}
		for _, instr := range actionQueue {
			bot := robots[instr.botID]
			if bot.hasLow && bot.hasHigh {
				low, high := bot.Pop()

				if part == 1 && ((low == 17 && high == 61) || (low == 61 && high == 17)) {
					return instr.botID
				}

				if part == 2 {
					b0, ok := outputs[0]
					if ok {
						b1, ok := outputs[1]
						if ok {
							b2, ok := outputs[2]
							if ok {
								return b0 * b1 * b2
							}
						}
					}
				}

				if instr.lowType == "bot" {
					if robots[instr.lowID] == nil {
						robots[instr.lowID] = &Robot{}
					}
					robots[instr.lowID].Push(low)
				} else {
					outputs[instr.lowID] = low
				}

				if instr.highType == "bot" {
					if robots[instr.highID] == nil {
						robots[instr.highID] = &Robot{}
					}
					robots[instr.highID].Push(high)
				} else {
					outputs[instr.highID] = high
				}
				progress = true
			} else {
				newQueue = append(newQueue, instr)
			}
		}
		actionQueue = newQueue
	}

	return -1
}

func main() {
	fmt.Println("Starting day 10")
	lines := adv.GetInput("10", true, "\n", true)
	result := SimulateRobots(lines, 1)
	fmt.Printf("Part 1 result: %d\n", result)
	part2 := SimulateRobots(inputs, 2)
	fmt.Printf("Part 2 result: %v\n", part2)
}
