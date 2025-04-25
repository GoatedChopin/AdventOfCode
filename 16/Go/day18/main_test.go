package main

import (
	"testing"
)

// TODO: if we can move 2 items up, don't bother with the 1-item moves.
// AND, if we can move just 1 item down, don't bother with the 2-item moves.
func Test_dayFourteen00(t *testing.T) {
	tests := []struct {
		name   string
		inputs []string
		want   int
	}{
		{"actual", []string{"5,4", "2,1"}, 5},
	}

	RenderTiles(BuildTiles(".^^.^.^^^^", 10))
	t.Run("Pathfinding", func(t *testing.T) {
		if got := SafeTiles(".^^.^.^^^^", 10); got != 38 {
			t.Errorf("%v = %v, want %v", ".^^.^.^^^^", got, 38)
		}
	})
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// if got := FindFirstDrop(tt.inputs); got != tt.want {
			// 	t.Errorf("%v = %v, want %v", tt.inputs, got, tt.want)
			// }
		})
	}
}
