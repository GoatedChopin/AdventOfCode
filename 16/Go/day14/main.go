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

func stretchedHash(i int, salt string, times int) string {
	hash := salt + strconv.Itoa(i)
	for j := 0; j < times; j++ {
		sum := md5.Sum([]byte(hash))
		hash = hex.EncodeToString(sum[:])
		// fmt.Printf("%v\n", hash)
	}
	return hash
}

func CountChars(h string) (rune, []rune) {
	var firstTriple rune
	var fives []rune
	for i := range len(h) - 2 {
		if h[i] == h[i+1] && h[i+1] == h[i+2] {
			if firstTriple == 0 {
				firstTriple = rune(h[i])
			}
			// check for five here instead? No! Leave it for next loop
			break
		}
	}
	for i := range len(h) - 4 {
		if h[i] == h[i+1] && h[i+1] == h[i+2] && h[i+2] == h[i+3] && h[i+3] == h[i+4] {
			fives = append(fives, rune(h[i]))
		}
	}
	return firstTriple, fives
}

func Hashes(num int, salt string, part int) int {
	hashTimes := 1
	if part == 2 {
		hashTimes = 2017
	}
	valid := make(map[int]bool)
	validIndexes := []int{}
	waiting := make(map[rune][]Hash)
	i := 0

	for len(validIndexes) < num {
		hash := stretchedHash(i, salt, hashTimes)
		triplet, fives := CountChars(hash)

		for _, c := range fives {
			for _, h := range waiting[c] {
				if h.expires >= i && !valid[h.from] {
					valid[h.from] = true
					validIndexes = append(validIndexes, h.from)
					fmt.Printf("Valid: %v at %d (found at %d)\n", h.s, h.from, i)
				}
			}
			waiting[c] = make([]Hash, 0)
		}
		if triplet != 0 {
			waiting[triplet] = append(waiting[triplet], Hash{hash, i, i + 1000})
		}
		i++
	}
	slices.Sort(validIndexes)
	return validIndexes[num-1]
}

func main() {
	fmt.Print("Starting day 12\n")
	salt := "zpqevtbw"
	part1 := Hashes(64, salt, 1)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := Hashes(64, salt, 2)
	fmt.Printf("Part 2: %v\n", part2)
}
