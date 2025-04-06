package main

import (
	"testing"
)

func Test_dayTwo(t *testing.T) {
	tests := []struct {
		name  string
		input []string
		part  uint
		want  string
	}{
		{"actual", []string{"ULL", "RRDDD", "LURDL", "UUUUD"}, 1, "1985"},
		{"actual", []string{"ULL", "RRDDD", "LURDL", "UUUUD"}, 2, "5DB3"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Pinpad(tt.input, tt.part); got != tt.want {
				t.Errorf("Pinpad() = %v, want %v", got, tt.want)
			}
		})
	}
}
