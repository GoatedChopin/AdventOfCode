package main

import (
	"fmt"
	"testing"
)

func Test_daySeven00(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  bool
	}{
		{"actual", "abba", true},
		{"actual", "abab", false},
		{"actual", "ioxxoj", true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := hasAbba(tt.input); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.input, got, tt.want)
			}
		})
	}
}

func Test_daySeven01(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  bool
	}{
		{"actual", "abba[mnop]qrst", true},
		{"actual", "abcd[bddb]xyyx", false},
		{"actual", "aaaa[qwer]tyui", false},
		{"actual", "ioxxoj[asdfgh]zxcvbn", true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := validIpv7(tt.input); got != tt.want {
				fmt.Printf("%v\n", tt.input)
				t.Errorf("Abba = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_daySeven02(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  bool
	}{
		{"actual", "aba[bab]xyz", true},
		{"actual", "xyx[xyx]xyx", false},
		{"actual", "aaa[kek]eke", true},
		{"actual", "zazbz[bzb]cdb", true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := validSSL(tt.input); got != tt.want {
				fmt.Printf("%v\n", tt.input)
				t.Errorf("SSL = %v, want %v", got, tt.want)
			}
		})
	}
}
