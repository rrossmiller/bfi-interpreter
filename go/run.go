package main

import (
	// "bufio"
	"fmt"
	// "os"
)

type Runner struct {
	i      int
	cursor int
	// state  [256]uint8
	state [12]uint8
	src   []rune
}

func (r *Runner) run(src []rune) {
	r.src = src

	for r.i < len(r.src) {
		r.op()
		r.i++
	}
}

func (r *Runner) op() {
	c := r.src[r.i]
	switch c {
	case '+':
		r.state[r.cursor] += 1
	case '-':
		r.state[r.cursor] -= 1
	case '>':
		r.cursor++
	case '<':
		r.cursor--
	case '[':
		loopI := r.i
		for r.state[r.cursor] > 0 {
			// if the current char is the end of the loop, go to the beginning
			if r.src[r.i] == ']' {
				r.i = loopI
			}
			r.i++ // step into the loop
			r.op()

		}
	case '.':
		fmt.Print(string(r.state[r.cursor]))
	case ',':
		//user input
	}
	fmt.Println(r.state)
	// buf := bufio.NewReader(os.Stdin)
	// buf.ReadLine()
	// fmt.Print("\033c")
}
