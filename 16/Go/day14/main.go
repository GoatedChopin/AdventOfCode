package main

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"slices"
	"strconv"
)

type Hash struct {
	s       string
	from    int
	expires int
}

func ValidHash(h string) bool {
	return false
}

func CountChars(h string) ([]rune, []rune) {
	threes, fives := make([]rune, 0), make([]rune, 0)
	var a, b, c, d, e rune
	for i, char := range h {
		if i == 0 {
			a = char
		} else if i == 1 {
			b = char
		} else if i == 2 {
			c = char
			if a == b && b == c {
				threes = append(threes, a)
			}
		} else if i == 3 {
			d = char
		} else if i == 4 {
			e = char
			if a == b && b == c {
				fmt.Printf("%v, %v, %v\n", h, a, i)
				if !slices.Contains(threes, a) {
					threes = append(threes, a)
				}
				if c == d && d == e {
					if !slices.Contains(fives, a) {
						fives = append(fives, a)
					}
				}
			}
			a = b
			b = c
			c = d
			d = e
		} else {
			if a == b && b == c {
				if !slices.Contains(threes, a) {
					threes = append(threes, a)
				}
				if c == d && d == e {
					if !slices.Contains(fives, a) {
						fives = append(fives, a)
					}
				}
			}
			// fmt.Printf("%c%c%c%c%c\n", a, b, c, d, e)
			a = b
			b = c
			c = d
			d = e
			e = char
		}
	}
	if a == b && b == c {
		if !slices.Contains(threes, a) {
			threes = append(threes, a)
		}
		if c == d && d == e {
			if !slices.Contains(fives, a) {
				fives = append(fives, a)
			}
		}
	}
	return threes, fives
}

func Hashes(num int, salt string) int {
	valid := make(map[string]bool, 0)
	validIndexes := make([]int, 0)
	waiting := make(map[rune][]Hash)
	i := 0
	for len(validIndexes) < num {
		bytes := md5.Sum([]byte(strconv.Itoa(i) + salt))
		hash := hex.EncodeToString(bytes[:])
		threes, fives := CountChars(hash)
		if len(threes) > 0 {
			c := threes[0]
			waiting[c] = append(waiting[c], Hash{hash, i, i + 1000})
		}
		for _, c := range fives {
			for _, waitingHash := range waiting[c] {
				if waitingHash.expires >= i && !valid[waitingHash.s] && !slices.Contains(validIndexes, waitingHash.from) {
					valid[waitingHash.s] = true
					validIndexes = append(validIndexes, waitingHash.from)
					fmt.Printf("Got %v at %v (char %c)\n\tfrom %v at %v\n", waitingHash.s, waitingHash.expires, c, hash, i)
				}
			}
			waiting[c] = make([]Hash, 0)
		}
		i++
	}
	// fmt.Printf("%v\n", valid)
	return validIndexes[len(validIndexes)-1]
}

func main() {
	fmt.Print("Starting day 12\n")
	salt := "zpqevtbw"
	part1 := Hashes(64, salt)
	fmt.Printf("Part 1: %v\n", part1)
}
