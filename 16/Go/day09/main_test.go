package main

import (
	"testing"
)

func Test_dayNine00(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{"actual", "advent", "advent"},
		{"actual", "A(1x5)BC", "ABBBBBC"},
		{"actual", "(3x3)XYZ", "XYZXYZXYZ"},
		{"actual", "A(2x2)BCD(2x2)EFG", "ABCBCDEFEFG"},
		{"actual", "(6x1)(1x3)A", "(1x3)A"},
		{"actual", "X(8x2)(3x3)ABCY", "X(3x3)ABC(3x3)ABCY"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Decompress(tt.input); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.input, got, tt.want)
			}
		})
	}
}
