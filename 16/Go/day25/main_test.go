package main

import (
	"testing"
)

func Test(t *testing.T) {
	t.Run("FindPath", func(t *testing.T) {
		walls, pois, startPoint := ParseGrid([]string{"###########", "#0.1.....2#", "#.#######.#", "#4.......3#", "###########"})
		if got := Follow(walls, pois, startPoint); got != 14 {
			t.Errorf("%v = %v, want %v", "FindPath", got, 14)
		}
	})
}
