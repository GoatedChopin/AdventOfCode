package main

import (
	"fmt"
	"testing"
)

func Test_dayFour00(t *testing.T) {
	tests := []struct {
		name  string
		input string
		part  uint
		want  bool
	}{
		{"actual", "aaaaa-bbb-z-y-x-123[abxyz]", 1, false},
		{"actual", "not-a-real-room-404[oarel]", 1, false},
		{"actual", "totally-real-room-200[decoy]", 1, true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := IsDummy(NewDatum(tt.input).encryptedName, NewDatum(tt.input).checksum); got != tt.want {
				fmt.Printf("%v\n", NewDatum(tt.input))
				t.Errorf("IsDummy() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_dayFour01(t *testing.T) {
	tests := []struct {
		name  string
		input []string
		want  uint
	}{
		{"actual", []string{"aaaaa-bbb-z-y-x-123[abxyz]", "a-b-c-d-e-f-g-h-987[abcde]", "not-a-real-room-404[oarel]", "totally-real-room-200[decoy]"}, 1514},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := NotDummies(tt.input); got != tt.want {
				t.Errorf("IsDummy() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_dayFour02(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		rotate int
		want   string
	}{
		{"actual", "qzmt-zixmtkozy-ivhz", 343, "very encrypted name"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Rotate(tt.input, tt.rotate); got != tt.want {
				t.Errorf("IsDummy() = %v, want %v", got, tt.want)
			}
		})
	}
}
