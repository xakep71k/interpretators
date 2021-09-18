package main

import (
	"bufio"
	"bytes"
	"fmt"
	"interpreteter/internal/expr"
	"io"
	"os"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	for {
		fmt.Print("calc> ")
		line, err := reader.ReadSlice('\n')
		if err != nil {
			if err == io.EOF {
				fmt.Println("bye")
				return
			}
			fmt.Fprintln(os.Stderr, err)
			return
		}

		input := bytes.NewBuffer(line)
		result, err := expr.Calc(input)
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			fmt.Println()
			continue
		}
		fmt.Println(result)
	}
}
