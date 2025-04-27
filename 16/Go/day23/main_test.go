package main

import (
	"testing"
)

func Test(t *testing.T) {
	t.Run("Follow", func(t *testing.T) {
		if got := Follow([]string{"cpy 2 a", "tgl a", "tgl a", "tgl a", "cpy 1 a", "dec a", "dec a"}, 1); got != 3 {
			t.Errorf("%v = %v, want %v", "5", got, 3)
		}
	})
}
