package main

import (
	"fmt"

	"github.com/rootfs/golang-tokenizer-bridge/tokenizer"
)

func main() {
	text := "Hello, world! This is a test."
	modelName := "bert-base-uncased"

	result, err := tokenizer.Tokenize(text, modelName)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	fmt.Println("Tokens:", result.Tokens)
	fmt.Println("IDs:", result.Ids)
}
