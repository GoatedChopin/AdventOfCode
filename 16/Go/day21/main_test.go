package main

import (
	"fmt"
	"strings"
	"testing"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func Test(t *testing.T) {
	t.Run("Rotate", func(t *testing.T) {

		inputs := adv.GetInput("21", true, "\n", true)
		part2 := Unscramble(inputs, "fbgdceah")
		fmt.Printf("Part 2: %v\n", part2)
	})
	t.Run("Rotate", func(t *testing.T) {

		s := strings.Split("abcd", "")
		Rotate(&s, 1)
		if strings.Join(s, "") != "dabc" {
			t.Errorf("%v = %v, want %v", "Rotate 0", strings.Join(s, ""), "dabc")
		}
		Rotate(&s, -1)
		if strings.Join(s, "") != "abcd" {
			t.Errorf("%v = %v, want %v", "Rotate 1", strings.Join(s, ""), "abcd")
		}
		Rotate(&s, 2)
		if strings.Join(s, "") != "cdab" {
			t.Errorf("%v = %v, want %v", "Rotate 2", strings.Join(s, ""), "cdab")
		}
		Rotate(&s, -2)
		if strings.Join(s, "") != "abcd" {
			t.Errorf("%v = %v, want %v", "Rotate 3", strings.Join(s, ""), "abcd")
		}
		i := strings.Split("swap letter b with letter c", " ")
		Execute(i, &s)
		if strings.Join(s, "") != "acbd" {
			t.Errorf("%v = %v, want %v", i[0], strings.Join(s, ""), "acbd")
		}
		i = strings.Split("reverse positions 1 through 2", " ")
		Execute(i, &s)
		if strings.Join(s, "") != "abcd" {
			t.Errorf("%v = %v, want %v", i[0], strings.Join(s, ""), "abcd")
		}
		i = strings.Split("rotate based on position of letter c", " ")
		Execute(i, &s)
		if strings.Join(s, "") != "cdab" {
			t.Errorf("%v = %v, want %v", i[0], strings.Join(s, ""), "cdab")
		}
		i = strings.Split("swap position 1 with position 2", " ")
		Execute(i, &s)
		if strings.Join(s, "") != "cadb" {
			t.Errorf("%v = %v, want %v", i[0], strings.Join(s, ""), "cadb")
		}
		i = strings.Split("move position 0 to position 3", " ")
		Execute(i, &s)
		if strings.Join(s, "") != "adbc" {
			t.Errorf("%v = %v, want %v", i[0], strings.Join(s, ""), "adbc")
		}
		i = strings.Split("move position 1 to position 2", " ")
		Execute(i, &s)
		if strings.Join(s, "") != "abdc" {
			t.Errorf("%v = %v, want %v", i[0], strings.Join(s, ""), "abdc")
		}
		i = strings.Split("move position 3 to position 0", " ")
		Execute(i, &s)
		if strings.Join(s, "") != "cabd" {
			t.Errorf("%v = %v, want %v", i[0], strings.Join(s, ""), "cabd")
		}
	})
}
