RUST_LIB_DIR := $(CURDIR)/target/release
GO_LIB_DIR := $(CURDIR)/golang-tokenizer-bridge/tokenizer/lib
EXAMPLE_DIR := $(CURDIR)/example/golang
# Default model name for testing
MODEL ?= bert-base-uncased

.PHONY: all clean rust-lib go-lib example run test test-go test-all

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

# To use with a custom model, run: make test MODEL=custom-model-name
test:
	@echo "Testing with model: $(MODEL) (Rust)"
	cargo run --bin test_tokenizer $(MODEL)

# Test the Go binding with a specified model
test-go: example
	@echo "Testing with model: $(MODEL) (Go)"
	cd $(EXAMPLE_DIR) && ./tokenizer-app $(MODEL)

# Test both implementations
test-all: test test-go
	@echo "All tests completed successfully"

download-model:
	mkdir -p example/models
	curl -L https://huggingface.co/bert-base-uncased/resolve/main/tokenizer.json \
	     -o example/models/tokenizer.json

clean:
	cargo clean
	rm -rf $(GO_LIB_DIR)
	rm -f $(EXAMPLE_DIR)/tokenizer-app 