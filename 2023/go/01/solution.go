package day01

import (
	"adventofcode/util"
	"fmt"
	"strconv"
	"strings"
)

func PartOne() {
	input := util.GetInput("01.txt")
	lines := strings.Split(input, "\n")
	result := 0

	for _, line := range lines {
		chars := strings.Split(line, "")
		nums := []int{}
		for _, char := range chars {
			if char == "" {
				continue
			}

			num, err := strconv.Atoi(char)
			if err != nil {
				continue
			}
			nums = append(nums, num)
		}
		result += nums[0]*10 + nums[len(nums)-1]
	}

	fmt.Println(result)
}
