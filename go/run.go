package main

import (
	"bufio"
	"fmt"
	"os"
)

type Runner struct {
	i      int
	loopI  []int
	cursor int
	// state  [256]byte
	state [12]byte
	src   []rune
	out   []rune
	step  bool
}

func (r *Runner) run(src []rune) {
	r.src = src

	for r.i < len(r.src) {
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
		case '.':
			if !r.step {
				fmt.Print(string(r.state[r.cursor]))
			} else {
				r.out = append(r.out, rune(r.state[r.cursor]))
			}
		case ',':
			//user input
		case '[':
			r.loopI = append(r.loopI, r.i)
		case ']':
			if r.state[r.cursor] > 0 {
				r.i = r.loopI[len(r.loopI)-1]
				r.loopI = r.loopI[:len(r.loopI)]
			}
		}

		if r.step {
			fmt.Println(r.state)
			fmt.Printf("%v", string(r.out))
			buf := bufio.NewReader(os.Stdin)
			buf.ReadLine()
			fmt.Print("\033c")
		}
		r.i++
	}
}
