package main

import (
	"testing"
)

func Test_dayOne(t *testing.T) {
	tests := []struct {
		name  string
		input []string
		want  int
	}{
		{"actual", []string{"R5", "L5", "R5", "R3"}, 12},
		{"actual", []string{"R2", "R2", "R2"}, 2},
		{"actual", []string{"R2", "L3"}, 5},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Walk(tt.input, 1); got != tt.want {
				t.Errorf("taxicab() = %v, want %v", got, tt.want)
			}
		})
	}
}
