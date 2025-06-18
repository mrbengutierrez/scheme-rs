# racket-rs

A Rust-based Racket interpreter. Just a fun little project.

## 🛠 Building

To build the native CLI:
make build-bin

To build the browser demo (WebAssembly):
make build-web

To clean both:
make clean

## 🌐 Live Demo

Try the playground in your browser:  
https://mrbengutierrez.github.io/racket-rs/

## ✅ Currently Supported

### Literals
- Numbers
- Booleans
- Strings

### Special Forms
- `define`
- `lambda`
- `begin`
- `if`
- `let`

### Built-in Functions
- Arithmetic: `+`, `-`, `*`, `/`
- Comparison: `=`, `<`, `>`
- Logic: `and`, `or`, `not`
- Lists: `list`, `car`, `cdr`, `cons`

### Function Application
- Built-in and user-defined functions (via `lambda`)
- Lexical scoping with environment chaining

### Evaluation
- REPL-style expression evaluation
- Simple error handling (e.g., arity mismatch, type error, undefined symbol)

## ❌ Not Supported (yet)
- Macros
- Tail-call optimization
- Full Racket standard library
- Advanced types and I/O