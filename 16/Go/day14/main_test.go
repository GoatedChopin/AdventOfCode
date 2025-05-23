package main

import (
	"crypto/md5"
	"encoding/hex"
	"strconv"

	"testing"
)

// TODO: if we can move 2 items up, don't bother with the 1-item moves.
// AND, if we can move just 1 item down, don't bother with the 2-item moves.
func Test_dayFourteen00(t *testing.T) {
	tests := []struct {
		name   string
		inputs string
		want   int
	}{
		{"actual", "abc", 22728},
	}
	// three, fives := CountChars("hhhaaaaabbccdksll")
	// fmt.Printf("%c | %v", three, fives)
	t.Run("stretch", func(t *testing.T) {
		bytes := md5.Sum([]byte(strconv.Itoa(1) + "abc"))
		hash := hex.EncodeToString(bytes[:])
		if got := stretchedHash(1, "abc", 1); got != hash {
			t.Errorf("1abc = %v, want %v", got, hash)
		}
	})
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Hashes(64, tt.inputs, 1); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.inputs, got, tt.want)
			}
		})
	}
}
