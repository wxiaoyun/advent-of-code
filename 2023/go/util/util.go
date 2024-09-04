package util

import (
	"os"
)

func GetInput(filename string) string {
	content, err := os.ReadFile("../../2023/questions/" + filename)
	if err != nil {
		panic(err)
	}

	return string(content)
}
