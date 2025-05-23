package main

import (
	"testing"
)

func Test_daySeven00(t *testing.T) {
	tests := []struct {
		name  string
		input []string
		want  int
	}{
		{"actual", []string{"rect 3x2", "rotate column x=1 by 1", "rotate row y=0 by 4", "rotate column x=1 by 1"}, 6},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := FollowInstructions(tt.input); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.input, got, tt.want)
			}
		})
	}
}
