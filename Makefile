# Project settings
PACKAGE_NAME = scheme-rs
DOCS_DIR = docs
PKG_DIR = $(DOCS_DIR)/pkg

# Build native CLI binary
build-bin:
	cargo build --release

# Build WebAssembly for the browser
build-web:
	wasm-pack build --target web --out-dir $(PKG_DIR)

# Build everything
build: build-bin build-web

# Clean both binary and WASM output
clean:
	cargo clean
	rm -rf $(PKG_DIR)
	rm -f $(DOCS_DIR)/scheme_rs*


