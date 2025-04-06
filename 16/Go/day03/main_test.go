package main

import (
	"testing"
)

func Test_dayThree(t *testing.T) {
	tests := []struct {
		name  string
		input []string
		part  uint
		want  int
	}{
		{"actual", []string{"5 10 25"}, 1, 0},
		{"actual", []string{"4  5  6"}, 1, 1},
		{"actual", []string{"101 301 501", "102 302 502", "103 303 503", "201 401 601", "202 402 602", "203 403 603"}, 1, 3},
		{"actual", []string{"101 301 501", "102 302 502", "103 303 503", "201 401 601", "202 402 602", "203 403 603"}, 2, 6},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.part == 1 {
				if got := Triangles(tt.input); got != tt.want {
					t.Errorf("Triangles() = %v, want %v", got, tt.want)
				}
			} else {
				if got := VertTriangles(tt.input); got != tt.want {
					t.Errorf("VertTriangles() = %v, want %v", got, tt.want)
				}
			}
		})
	}
}
