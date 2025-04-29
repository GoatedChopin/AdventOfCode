package main

import (
	"strings"
	"testing"
)

func Test(t *testing.T) {
	t.Run("Rotate", func(t *testing.T) {

		s := strings.Split("abcd", "")
		Rotate(&s, 1)
		if strings.Join(s, "") != "dabc" {
			t.Errorf("%v = %v, want %v", "FindPath", strings.Join(s, ""), "dabc")
		}
		Rotate(&s, -1)
		if strings.Join(s, "") != "abcd" {
			t.Errorf("%v = %v, want %v", "FindPath", strings.Join(s, ""), "abcd")
		}
		Rotate(&s, 2)
		if strings.Join(s, "") != "cdab" {
			t.Errorf("%v = %v, want %v", "FindPath", strings.Join(s, ""), "cdab")
		}
		Rotate(&s, -2)
		if strings.Join(s, "") != "abcd" {
			t.Errorf("%v = %v, want %v", "FindPath", strings.Join(s, ""), "abcd")
		}
	})
}
