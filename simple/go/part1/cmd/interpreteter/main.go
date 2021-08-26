package main

import (
	"bufio"
	"fmt"
	"interpreteter/internal/expr"
	"interpreteter/internal/token"
	"io"
	"os"
)

func main() {
	tokenStream := token.NewReader(bufio.NewReader(os.Stdin))

	for {
		fmt.Print("calc> ")
		result, err := expr.Calc(tokenStream)
		if err != nil {
			if err == io.EOF {
				fmt.Println("bye")
				return
			}
			fmt.Fprintln(os.Stderr, err)
			return
		}

		fmt.Println(result)
	}
}
