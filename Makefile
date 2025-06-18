# Project settings
PACKAGE_NAME = racket-rs
DOCS_DIR = docs
PKG_DIR = $(DOCS_DIR)/pkg

# Default target
all: build copy

# Compile to WASM using wasm-pack
build:
	wasm-pack build --target web --out-dir $(PKG_DIR)
