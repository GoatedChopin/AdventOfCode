package main

import (
	"testing"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func Test(t *testing.T) {
	// t.Run("Follow", func(t *testing.T) {
	// 	if got := Follow([]string{"cpy 2 a", "tgl a", "tgl a", "tgl a", "cpy 1 a", "dec a", "dec a"}, 1); got != 3 {
	// 		t.Errorf("%v = %v, want %v", "5", got, 3)
	// 	}
	// })
	t.Run("Follow", func(t *testing.T) {
		if got := Follow(adv.GetInput("23", true, "\n", true), 1); got != 3 {
			t.Errorf("%v = %v, want %v", "5", got, 3)
		}
	})
}
