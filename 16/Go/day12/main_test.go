package main

import (
	"testing"
)

// TODO: if we can move 2 items up, don't bother with the 1-item moves.
// AND, if we can move just 1 item down, don't bother with the 2-item moves.
func Test_dayTwelve00(t *testing.T) {
	tests := []struct {
		name   string
		inputs []string
		want   int
	}{
		{"actual", []string{
			"cpy 41 a",
			"inc a",
			"inc a",
			"dec a",
			"jnz a 2",
			"dec a",
		}, 42},
		{"actual", []string{
			"cpy 41 b",
			"inc a",
			"dec b",
			"jnz b -2",
			"dec a",
		}, 40},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Follow(tt.inputs, 0); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.inputs, got, tt.want)
			}
		})
	}
}
