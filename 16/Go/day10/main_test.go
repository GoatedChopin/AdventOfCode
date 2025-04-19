package main

import (
	"testing"
)

func Test_dayNine00(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  int
	}{
		{"actual", "advent", "advent"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Walk(tt.input); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.input, got, tt.want)
			}
		})
	}
}
