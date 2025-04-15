package main

import (
	"fmt"
	"strings"
	"testing"
)

const inputs = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar"

func Test_daySix(t *testing.T) {
	tests := []struct {
		name  string
		input []string
		want  string
	}{
		{"actual", strings.Split(inputs, "\n"), "easter"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := AverageChars(strings.Split(inputs, "\n"), 6); got != tt.want {
				fmt.Printf("%v\n", "Oh my god")
				t.Errorf("IsDummy() = %v, want %v", got, tt.want)
			}
		})
	}
}
