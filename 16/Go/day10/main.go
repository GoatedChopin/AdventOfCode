package main

import (
	"errors"
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

type Robot struct {
	low, high       int
	hasLow, hasHigh bool
}

func (r *Robot) Pop(s string) (int, error) {
	if s == "low" {
		if r.hasLow {
			val := r.low
			r.low = -1
			r.hasLow = false
			return val, nil
		}
		return -1, errors.New("robot does not have low val")
	}
	if s == "high" {
		if r.hasHigh {
			val := r.high
			r.high = -1
			r.hasHigh = false
			return val, nil
		}
		return -1, errors.New("robot does not have high val")
	}
	return -1, errors.New("must pass in low or high for s")
}

func (r *Robot) Push(i int) (int, error) {
	if (*r).hasLow && (*r).hasHigh {
		(*r).hasLow, (*r).hasHigh = true, true
		if i < (*r).low {
			out := (*r).high
			(*r).high = (*r).low
			(*r).low = i
			return out, nil
		} else if i > (*r).high {
			out := (*r).low
			(*r).low = (*r).high
			(*r).high = i
			return out, nil
		}
	} else if (*r).hasLow {
		(*r).hasLow, (*r).hasHigh = true, true
		if i < (*r).low {
			out := -1
			(*r).high = (*r).low
			(*r).low = i
			return out, nil
		} else {
			out := -1
			(*r).high = i
			return out, nil
		}
	} else if (*r).hasHigh {
		(*r).hasLow, (*r).hasHigh = true, true
		if i > (*r).high {
			out := -1
			(*r).low = (*r).high
			(*r).high = i
			return out, nil
		} else {
			out := -1
			(*r).low = i
			return out, nil
		}
	} else {
		(*r).low = i
		(*r).hasLow = true
		return -1, nil
	}
	return -1, nil
}

func MakeRobots(lines []string) []Robot {
	maxRobots := 1
	for _, line := range lines {
		splitLine := strings.Split(line, " ")
		if splitLine[0] == "bot" {
			botIndex, err := strconv.Atoi(splitLine[1])
			if err != nil {
				panic("bad botIndex")
			}
			if botIndex > maxRobots {
				maxRobots = botIndex
			}
		}
	}
	robots := make([]Robot, maxRobots+1)
	for _, line := range lines {
		splitLine := strings.Split(line, " ")
		if splitLine[0] == "bot" {
			botIndex, err := strconv.Atoi(splitLine[1])
			if err != nil {
				panic("bad botIndex")
			}
			robots[botIndex] = Robot{}
		}
	}
	return robots
}

func SimulateRobots(actions []string, robots []Robot, part int) int {
	for {
		for _, action := range actions {
			splitLine := strings.Split(action, " ")
			if splitLine[0] == "value" {
				botIndex, err := strconv.Atoi(splitLine[5])
				if err != nil {
					panic("bad botIndex")
				}
				val, err := strconv.Atoi(splitLine[1])
				if err != nil {
					panic("bad val")
				}
				robots[botIndex].Push(val)
				continue
			}
			if splitLine[2] == "gives" {
				givingBot, err := strconv.Atoi(splitLine[1])
				if err != nil {
					panic("bad botIndex")
				}
				if !robots[givingBot].hasLow || !robots[givingBot].hasHigh {
					continue
				}
				if part == 1 && robots[givingBot].low == 17 && robots[givingBot].high == 61 {
					return givingBot
				}
				// low
				lowVal, err := robots[givingBot].Pop("low")
				if err != nil {
					panic("bot doesn't have low")
				}
				if splitLine[5] == "bot" {
					botIndex, _ := strconv.Atoi(splitLine[6])
					robots[botIndex].Push(lowVal)
				} else if splitLine[5] == "output" {
					// handle output bin if needed for part 2
				}

				// high
				highVal, err := robots[givingBot].Pop("high")
				if err != nil {
					panic("bot doesn't have high")
				}
				if splitLine[10] == "bot" {
					botIndex, _ := strconv.Atoi(splitLine[11])
					robots[botIndex].Push(highVal)
				} else if splitLine[10] == "output" {
					// handle output bin if needed for part 2
				}
			}
		}
	}
	return -1
}

func main() {
	fmt.Printf("Starting day 10\n")
	inputs := adv.GetInput("10", true, "\n", true)
	robots := MakeRobots(inputs)
	part1 := SimulateRobots(inputs, robots, 1)
	fmt.Printf("%v\n", part1)
}
