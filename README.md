# Hugging Face Tokenizer Bridge

This project provides a C bridge between the Hugging Face tokenizers library (Rust) and other programming languages. Currently, it supports Golang integration.

## Project Structure

```
hf-tokenizer-bridge/
├── src/                # Rust source code for the bridge
├── golang-tokenizer-bridge/    # Golang bindings
│   ├── tokenizer/     # Tokenizer package
│   │   ├── lib/       # Shared library files
│   │   ├── tokenizer.go   # Go bindings
│   │   └── hf_tokenizer_bridge.h   # C header
├── example/           # Example applications
│   ├── golang/        # Golang example
│   └── models/        # Tokenizer models
```

## Building the Rust Library

### Prerequisites

- Rust and Cargo (1.50+)
- C compiler (gcc/clang)

### Build Steps

1. Clone the repository:
   ```
   git clone https://github.com/rootfs/hf-tokenizer-bridge.git
   cd hf-tokenizer-bridge
   ```

2. Build the Rust library:
   ```
   cargo build --release
   ```

3. Copy the shared library to the Golang bindings directory:
   ```
   mkdir -p golang-tokenizer-bridge/tokenizer/lib
   cp target/release/libhf_tokenizer_bridge.so golang-tokenizer-bridge/tokenizer/lib/
   ```

## Using the Golang Bindings

### Prerequisites

- Go 1.23+

### Setup

1. Import the tokenizer package in your Go code:
   ```go
   import "github.com/rootfs/golang-tokenizer-bridge/tokenizer"
   ```

2. In your go.mod file, add a replace directive to use the local version:
   ```
   module your-module-name

   go 1.23

   require github.com/rootfs/golang-tokenizer-bridge v0.0.0-unpublished

   replace github.com/rootfs/golang-tokenizer-bridge => /path/to/hf-tokenizer-bridge/golang-tokenizer-bridge
   ```

### Example Usage

```go
package main

import (
    "fmt"
    "github.com/rootfs/golang-tokenizer-bridge/tokenizer"
)

func main() {
    text := "Hello, world! This is a test."
    modelPath := "/path/to/tokenizer.json"
    
    result, err := tokenizer.Tokenize(text, modelPath)
    if err != nil {
        fmt.Println("Error:", err)
        return
    }
    
    fmt.Println("Tokens:", result.Tokens)
    fmt.Println("IDs:", result.Ids)
}
```

### Running Your Go Application

There are several ways to ensure your Go application can find the shared library:

1. Set the LD_LIBRARY_PATH environment variable:
   ```
   LD_LIBRARY_PATH=/path/to/hf-tokenizer-bridge/golang-tokenizer-bridge/tokenizer/lib go run main.go
   ```

2. Use rpath when building:
   ```
   CGO_LDFLAGS="-Wl,-rpath,/path/to/hf-tokenizer-bridge/golang-tokenizer-bridge/tokenizer/lib" go build -o myapp
   ```

3. Install the library system-wide:
   ```
   sudo cp /path/to/hf-tokenizer-bridge/golang-tokenizer-bridge/tokenizer/lib/libhf_tokenizer_bridge.so /usr/local/lib/
   sudo ldconfig
   ```

## Example Makefile

You can use this Makefile to simplify the build process:

```makefile
RUST_LIB_DIR := $(CURDIR)/target/release
GO_LIB_DIR := $(CURDIR)/golang-tokenizer-bridge/tokenizer/lib
EXAMPLE_DIR := $(CURDIR)/example/golang

.PHONY: all clean rust-lib go-lib example run

all: rust-lib go-lib example

rust-lib:
	cargo build --release

go-lib: rust-lib
	mkdir -p $(GO_LIB_DIR)
	cp $(RUST_LIB_DIR)/libhf_tokenizer_bridge.so $(GO_LIB_DIR)/

example: go-lib
	cd $(EXAMPLE_DIR) && CGO_LDFLAGS="-Wl,-rpath,$(GO_LIB_DIR)" go build -o tokenizer-app

run: example
	cd $(EXAMPLE_DIR) && ./tokenizer-app

clean:
	cargo clean
	rm -rf $(GO_LIB_DIR)
	rm -f $(EXAMPLE_DIR)/tokenizer-app
```

## Downloading a Tokenizer Model

You can download a pre-trained tokenizer model from Hugging Face:

```bash
mkdir -p example/models
curl -L https://huggingface.co/bert-base-uncased/resolve/main/tokenizer.json \
     -o example/models/tokenizer.json
```

## Troubleshooting

1. **Shared library not found**: Ensure the `libhf_tokenizer_bridge.so` file is correctly built and accessible through one of the methods described above.

2. **Tokenizer model not found**: Make sure the path to your tokenizer.json file is correct.

3. **Segmentation fault**: This might occur if the tokenizer model file is missing or invalid, or if there's an issue with the library initialization.

4. **Go build errors**: If you get CGO-related errors, make sure you have a C compiler installed and that the header file (`hf_tokenizer_bridge.h`) is accessible.

## License

[MIT License](LICENSE) 