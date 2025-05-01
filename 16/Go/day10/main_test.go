package main

import (
	"fmt"
	"testing"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func Test(t *testing.T) {
	t.Run("FindPath", func(t *testing.T) {
		fmt.Printf("Starting day 10\n")
		inputs := adv.GetInput("10", true, "\n", true)
		robots := MakeRobots(inputs)
		part1 := SimulateRobots(inputs, robots, 1)
		fmt.Printf("%v\n", part1)
	})
}
