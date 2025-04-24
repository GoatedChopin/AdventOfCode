package main

import (
	"container/list"
	"crypto/md5"
	"encoding/hex"
	"fmt"
)

type Vec struct {
	x     int
	y     int
	label rune
}

type Path struct {
	runes           []rune
	currentPosition Vec
}

func move(v Vec, d Vec) Vec {
	return Vec{v.x + d.x, v.y + d.y, d.label}
}

func inBounds(v Vec) bool {
	if 0 <= v.x && v.x < 4 && 0 <= v.y && v.y < 4 {
		return true
	}
	return false
}

func GetMoves(v Vec) []Vec {
	out := make([]Vec, 0)
	dirs := []Vec{{1, 0, 'D'}, {-1, 0, 'U'}, {0, 1, 'R'}, {0, -1, 'L'}}
	// dirs := []Vec{{0, -1, 'U'}, {0, 1, 'D'}, {-1, 0, 'L'}, {1, 0, 'R'}}
	for _, dir := range dirs {
		newV := move(v, dir)
		if inBounds(newV) {
			out = append(out, newV)
		}
	}
	return out
}

func Unlocked(passcode []rune, position Vec) []Vec {
	sum := md5.Sum([]byte(string(passcode)))
	hash := hex.EncodeToString(sum[:])
	fmt.Printf("%v ", hash)
	unlockChars := map[rune]bool{
		'b': true, 'c': true, 'd': true, 'e': true, 'f': true,
	}
	dirs := []Vec{
		{-1, 0, 'U'},
		{1, 0, 'D'},
		{0, -1, 'L'},
		{0, 1, 'R'},
	}
	out := make([]Vec, 0)
	for i := range 4 {
		if unlockChars[rune(hash[i])] {
			newV := move(position, dirs[i])
			if inBounds(newV) {
				out = append(out, dirs[i])
			}
		}
	}
	return out
}

func Pathfind(s string) string {
	queue := list.New()
	queue.PushBack(Path{[]rune(s), Vec{0, 0, ' '}})
	for queue.Len() > 0 {
		front := queue.Front()
		path := queue.Remove(front).(Path)
		position := path.currentPosition
		if position.x == 3 && position.y == 3 {
			return string(path.runes[len(s):])
		}
		// nextPositions := GetMoves(position)
		unlockedPositions := Unlocked(path.runes, position)
		fmt.Printf("(%v, %v) %v\t", position.x, position.y, string(path.runes))
		for _, pos := range unlockedPositions {
			fmt.Printf("(%v, %v, %c) ", pos.x, pos.y, pos.label)
		}
		fmt.Printf("\n")
		for _, p := range unlockedPositions {
			queue.PushBack(Path{append(path.runes, p.label), move(position, p)})
		}
	}
	return ""
}

func main() {
	fmt.Print("Starting day 15\n")
	input := "udskfozm"
	part1 := Pathfind(input)
	fmt.Printf("Part 1: %v", part1)
}
