package main

import (
	"fmt"
	"io"
	"os"
)

func main() {
	fname := os.Args[1]
	src := readFile(fname)
	r := Runner{}
	r.run(src)
	fmt.Println()
}

func readFile(fp string) []rune {
	if _, err := os.Stat(fp); err != nil {
		fmt.Println("file doesn't exist")
		os.Exit(1)
	}

	file, err := os.Open(fp)
	if err != nil {
		panic(err)
	}
	b, err := io.ReadAll(file)
	if err != nil {
		panic(err)
	}
	s := string(b)
	src := []rune(s)
	return src
}
