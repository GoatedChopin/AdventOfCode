package main

import (
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

// Extended Euclidean algorithm
func extendedGCD(a, b int) (gcd, x, y int) {
	if b == 0 {
		return a, 1, 0
	}
	gcd, x1, y1 := extendedGCD(b, a%b)
	x = y1
	y = x1 - (a/b)*y1
	return
}

// Modular inverse using extended Euclidean algorithm
func modInverse(a, m int) int {
	gcd, x, _ := extendedGCD(a, m)
	if gcd != 1 {
		panic("No modular inverse exists!")
	}
	return (x%m + m) % m
}

type Congruence struct {
	remainder int
	modulus   int
}

func SolveCRT(congs []Congruence) int {
	N := 1
	for _, cong := range congs {
		N *= cong.modulus
	}
	result := 0
	for _, cong := range congs {
		ni := N / cong.modulus
		mi := modInverse(ni, cong.modulus)
		result += cong.remainder * ni * mi
	}
	return result % N
}

func ParseSpheres(lines []string) []Congruence {
	congs := make([]Congruence, len(lines))
	for i, line := range lines {
		parts := strings.Split(line, ",")
		modulus, _ := strconv.Atoi(parts[0])
		start, _ := strconv.Atoi(parts[1])
		// t ≡ -start - (i+1) mod modulus
		rem := (-start - (i + 1)) % modulus
		if rem < 0 {
			rem += modulus
		}
		congs[i] = Congruence{rem, modulus}
	}
	return congs
}

type Sphere struct {
	positionCount int
	startsAt      int
	position      int
}

func fastForward(moves int, spheres []Sphere) []Sphere {
	movedSpheres := make([]Sphere, len(spheres))
	for i, s := range spheres {
		effectiveMoves := moves + i + 1
		for s.startsAt+effectiveMoves >= s.positionCount {
			effectiveMoves -= s.positionCount
		}
		if s.startsAt+effectiveMoves == s.positionCount {
			movedSpheres[i] = Sphere{s.positionCount, s.startsAt, 0}
		} else {
			movedSpheres[i] = Sphere{s.positionCount, s.startsAt, s.startsAt + effectiveMoves}
		}
		// fmt.Printf("positions %v, starting at %v, time %v -> %v\n", movedSpheres[i].positionCount, movedSpheres[i].startsAt, moves+i, movedSpheres[i].position)
	}
	return movedSpheres
}

func Drop(startTime int, spheres []Sphere) bool {
	adjustedSpheres := fastForward(startTime, spheres)
	// fmt.Printf("\n")
	for _, sphere := range adjustedSpheres {
		if sphere.position != 0 {
			return false
		}
	}
	return true
}

func FindFirstDrop(lines []string) int {
	spheres := make([]Sphere, len(lines))
	for i, line := range lines {
		parts := strings.Split(line, ",")
		positionCount, err := strconv.Atoi(parts[0])
		if err != nil {
			panic("bad positionCount!")
		}
		startsAt, err := strconv.Atoi(parts[1])
		if err != nil {
			panic("bad startsAt!")
		}
		spheres[i] = Sphere{positionCount, startsAt, startsAt}
	}
	i := 0
	for !Drop(i, spheres) {
		i++
	}
	return i
}

func main() {
	fmt.Print("Starting day 15\n")
	inputs := adv.GetInput("15", true, "\n", true)
	// part1 := FindFirstDrop(inputs)
	// fmt.Printf("%v\n", part1)
	part2 := SolveCRT(ParseSpheres(inputs))
	fmt.Printf("%v\n", part2)
}
