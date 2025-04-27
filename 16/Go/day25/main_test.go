package main

import (
	"fmt"
	"strings"
	"testing"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func Test(t *testing.T) {
	t.Run("FindPath", func(t *testing.T) {
		instructions := adv.GetInput("25", true, "\n", true)
		splitInstructions := make([][]string, len(instructions))
		for i, s := range instructions {
			args := strings.Split(s, " ")
			splitInstructions[i] = args
		}
		els := 0
		for i := range Follow(0, splitInstructions) {
			fmt.Printf("%v, ", i)
			els++
			if els >= 20 {
				break
			}
		}
		// if got := AlternatingSignal(); got != 14 {
		// 	t.Errorf("%v = %v, want %v", "FindPath", got, 14)
		// }
	})
}
