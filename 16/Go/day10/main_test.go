package main

import (
	"testing"
)

func Test_dayTen00(t *testing.T) {
	tests := []struct {
		name   string
		inputs []string
		want   int
	}{
		{"actual", []string{
			"HM,LM",
			"HG",
			"LG",
			"--",
		}, 11},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := WalkElevatorConstraint(0, 0, Building(tt.inputs)); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.inputs, got, tt.want)
			}
		})
	}
}
