package main

import (
	"testing"
)

// TODO: if we can move 2 items up, don't bother with the 1-item moves.
// AND, if we can move just 1 item down, don't bother with the 2-item moves.
func Test_dayThirteen00(t *testing.T) {
	tests := []struct {
		name   string
		inputs []int
		want   int
	}{
		{"actual", []int{
			1, 1, 7, 4,
		}, 11},
	}
	maze := CreateMaze(10, 10, 10)
	VisualizeMaze(maze)
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := ShortestPath(tt.inputs[0], tt.inputs[1], tt.inputs[2], tt.inputs[3], 10); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.inputs, got, tt.want)
			}
		})
	}
}
