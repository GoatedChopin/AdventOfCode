package main

import (
	"crypto/md5"
	"encoding/hex"
	"errors"
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func fiveZeros(hash string) (rune, error) {
	for i, c := range hash {
		if i < 5 && c != '0' {
			return ' ', errors.New("hash does not begin with 5 zeros")
		}
		if i == 5 {
			return c, nil
		}
	}
	return ' ', errors.New("unexpected control flow")
}

func placement(hash string) (int, rune, error) {
	position := 0
	for i, c := range hash {
		if i < 5 && c != '0' {
			return 0, ' ', errors.New("hash does not begin with 5 zeros")
		}
		if i == 5 {
			p := c - '0'
			if p < 0 || p > 7 {
				return int(p), ' ', errors.New("invalid position character")
			}
			position = int(p)
		}
		if i == 6 {
			return position, c, nil
		}
	}
	return 0, ' ', errors.New("unexpected control flow")
}

func GetMD5Hash(btext []byte) string {
	hash := md5.Sum(btext)
	return hex.EncodeToString(hash[:])
}

func search(text string, n uint) string {
	var i uint64 = 0
	pass := ""
	for uint(len(pass)) < n {
		attempt := []byte(text + strconv.FormatUint(i, 10))
		md := GetMD5Hash(attempt)
		nchar, err := fiveZeros(md)
		i++
		if err != nil {
			continue
		}
		pass += string(nchar)
	}
	return pass
}

func research(text string, n uint) string {
	var i uint64 = 0
	var indexes uint = 0
	pass := make([]string, 8)
	for indexes < n {
		attempt := []byte(text + strconv.FormatUint(i, 10))
		md := GetMD5Hash(attempt)
		position, nchar, err := placement(md)
		i++
		if err != nil {
			continue
		}
		if pass[position] == "" {
			indexes++
			pass[position] = string(nchar)
		}
	}
	return strings.Join(pass, "")
}

func main() {
	input := strings.TrimSpace(adv.GetInput("5", false, "\n", true)[0])
	fmt.Printf("%v\n", input)
	// fmt.Printf("Answer 1: %v\n", search(input, 8))
	position, char, err := placement("000001545345")
	fmt.Printf("%v, %c, %v\n", position, char, err)
	fmt.Printf("Answer 2: %v\n", research(input, 8))
}
