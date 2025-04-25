package main

import (
	"testing"
)

// TODO: if we can move 2 items up, don't bother with the 1-item moves.
// AND, if we can move just 1 item down, don't bother with the 2-item moves.
func Test_dayFourteen00(t *testing.T) {
	// t.Run("WhiteElephant", func(t *testing.T) {
	// 	if got := WhiteElephant(5); got != 3 {
	// 		t.Errorf("%v = %v, want %v", "5", got, 3)
	// 	}
	// })
	// t.Run("FrontalWhiteElephant", func(t *testing.T) {
	// 	if got := FrontalWhiteElephant(5); got != 2 {
	// 		t.Errorf("%v = %v, want %v", "5", got, 2)
	// 	}
	// })
	t.Run("FrontalWhiteElephant", func(t *testing.T) {
		if got := FrontalWhiteElephant(3014603); got != 2 {
			t.Errorf("%v = %v, want %v", "5", got, 2)
		}
	})
}
