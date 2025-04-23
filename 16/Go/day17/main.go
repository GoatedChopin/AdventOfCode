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
	for _, dir := range dirs {
		newV := move(v, dir)
		if inBounds(newV) {
			out = append(out, newV)
		}
	}
	return out
}

func Unlocked(runes []rune, vecs []Vec) []Vec {
	// Make MD5 hash
	sum := md5.Sum([]byte(string(runes)))
	hash := hex.EncodeToString(sum[:])
	unlockChars := map[rune]bool{
		'b': true, 'c': true, 'd': true, 'e': true, 'f': true,
	}
	dirs := []rune{'U', 'D', 'L', 'R'}
	out := make([]rune, 0)
	for i, c := range hash {
		if i < 4 && unlockChars[c] {
			out = append(out, dirs[i])
		}
	}
	unlocked := make([]Vec, 0)
	for _, r := range out {
		for _, v := range vecs {
			if v.label == r {
				unlocked = append(unlocked, v)
				break
			}
		}
	}
	fmt.Printf("%v\t%v\t%v\t%v\n", string(runes), hash, string(out), len(unlocked))
	return unlocked
}

func Pathfind(s string) string {
	queue := list.New()
	queue.PushBack(Path{[]rune(s), Vec{0, 0, ' '}})
	for queue.Len() > 0 {
		front := queue.Front()
		path := queue.Remove(front).(Path)
		position := path.currentPosition
		fmt.Printf("(%v, %v) ", position.x, position.y)
		if position.x == 3 && position.y == 3 {
			return string(path.runes)
		}
		nextPositions := GetMoves(position)
		unlockedPositions := Unlocked(path.runes, nextPositions)
		for _, p := range unlockedPositions {
			queue.PushBack(Path{append(path.runes, p.label), p})
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
