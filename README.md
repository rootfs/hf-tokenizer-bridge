# Hugging Face Tokenizer Bridge

A bridge between Hugging Face tokenizers (Rust) and Golang, allowing you to use HF tokenizers in Go applications.

## Quick Start

### Build

```bash
# Build everything (Rust library and Go example)
make all

# Or build just the components you need
make rust-lib   # Build only the Rust library
make go-lib     # Build the library and copy it for Go
make example    # Build the Go example app
```

### Test

```bash
# Test the Rust implementation with default model (bert-base-uncased)
make test

# Test with a specific model
make test MODEL=bert-base-cased

# Test the Go bindings
make test-go

# Test both implementations
make test-all
```

### Usage Example (Go)

```go
package main

import (
    "fmt"
    "github.com/rootfs/golang-tokenizer-bridge/tokenizer"
)

func main() {
    text := "Hello, world! This is a test."
    
    // Use a local tokenizer file
    modelPath := "/path/to/tokenizer.json"
    
    // Or use a HuggingFace model name
    // modelPath := "bert-base-uncased"
    
    result, err := tokenizer.Tokenize(text, modelPath)
    if err != nil {
        fmt.Println("Error:", err)
        return
    }
    
    fmt.Println("Tokens:", result.Tokens)
    fmt.Println("IDs:", result.Ids)
}
```

### Using Models with Authentication

For gated models like Meta-Llama:

```go
// Set HF_TOKEN environment variable
os.Setenv("HF_TOKEN", "your_huggingface_token")

// Or use the direct function
result, err := tokenizer.TokenizeWithToken(text, "meta-llama/Meta-Llama-3.1-8B-Instruct", "your_huggingface_token")
```

## Download a Model for Testing

```bash
# Download a test model
make download-model
```

## Clean Up

```bash
make clean
``` 