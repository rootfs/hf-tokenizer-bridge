package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/rootfs/golang-tokenizer-bridge/tokenizer"
)

func main() {
	text := "Hello, world! This is a test."

	// Check if command line argument is provided for tokenizer file path
	var modelPath string
	if len(os.Args) > 1 {
		modelPath = os.Args[1]
	} else {
		// Check if we can find a cache model for bert-base-uncased
		homeDir, err := os.UserHomeDir()
		if err == nil {
			cachePath := filepath.Join(homeDir, ".cache", "huggingface", "hub", "models--bert-base-uncased", "snapshots")
			if _, err := os.Stat(cachePath); err == nil {
				// Cache directory exists, find the latest snapshot
				entries, err := os.ReadDir(cachePath)
				if err == nil && len(entries) > 0 {
					// Take the first directory (which is usually the only one)
					tokenizerPath := filepath.Join(cachePath, entries[0].Name(), "tokenizer.json")
					if _, err := os.Stat(tokenizerPath); err == nil {
						modelPath = tokenizerPath
					}
				}
			}
		}

		// Fallback to a known local model
		if modelPath == "" {
			modelPath = "../models/tokenizer.json"
		}
	}

	fmt.Printf("Using tokenizer file: %s\n", modelPath)

	// Check if this is a model that requires authentication (like Meta-Llama)
	var result *tokenizer.TokenizerResult
	var err error

	// For Meta-Llama models, try to use HF_TOKEN environment variable
	if modelPath == "meta-llama/Meta-Llama-3.1-8B-Instruct" ||
		modelPath == "meta-llama/Llama-2-7b-hf" ||
		strings.Contains(modelPath, "meta-llama") {

		// Get HF_TOKEN from environment
		token := os.Getenv("HF_TOKEN")
		if token != "" {
			fmt.Println("Using Hugging Face API token for authentication")
			result, err = tokenizer.TokenizeWithToken(text, modelPath, token)
		} else {
			fmt.Println("Warning: Meta-Llama models require authentication.")
			fmt.Println("Set the HF_TOKEN environment variable with your Hugging Face API token.")
			fmt.Println("Attempting tokenization without authentication (will likely fail)...")
			result, err = tokenizer.Tokenize(text, modelPath)
		}
	} else {
		// For other models, use standard tokenization
		result, err = tokenizer.Tokenize(text, modelPath)
	}

	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	fmt.Println("Tokens:", result.Tokens)
	fmt.Println("IDs:", result.Ids)
}
