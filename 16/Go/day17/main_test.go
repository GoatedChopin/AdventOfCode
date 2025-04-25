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
	t.Run("Pathfinding", func(t *testing.T) {
		if got := Pathfind("ihgpwlah"); got != "DDRRRD" {
			t.Errorf("%v = %v, want %v", "ihgpwlah", got, "DDRRRD")
		}
		if got := Pathfind("kglvqrro"); got != "DDUDRLRRUDRD" {
			t.Errorf("%v = %v, want %v", "kglvqrro", got, "DDUDRLRRUDRD")
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
