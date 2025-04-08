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

download-model:
	mkdir -p example/models
	curl -L https://huggingface.co/bert-base-uncased/resolve/main/tokenizer.json \
	     -o example/models/tokenizer.json

clean:
	cargo clean
	rm -rf $(GO_LIB_DIR)
	rm -f $(EXAMPLE_DIR)/tokenizer-app 