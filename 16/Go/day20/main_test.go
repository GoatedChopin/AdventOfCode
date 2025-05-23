package main

import (
	"testing"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func Test_dayFourteen00(t *testing.T) {
	// t.Run("LowestIp", func(t *testing.T) {
	// 	if got := LowestAllowedIp([]string{"5-8", "0-2", "4-7"}); got != 3 {
	// 		t.Errorf("%v = %v, want %v", "5", got, 3)
	// 	}
	// })
	// t.Run("LowestIp", func(t *testing.T) {
	// 	if got := LowestAllowedIp(adv.GetInput("20", true, "\n", true)); got != 3 {
	// 		t.Errorf("%v = %v, want %v", "5", got, 3)
	// 	}
	// })
	t.Run("All IPs", func(t *testing.T) {
		if got := AllAllowedIps(adv.GetInput("20", true, "\n", true)); got != 3 {
			t.Errorf("%v = %v, want %v", "5", got, 3)
		}
	})
}
