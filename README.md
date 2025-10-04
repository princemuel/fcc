# FCC, The Faux-C Compiler

This is the Rust reference implementation for the book [Writing a C Compiler][writing-c-compiler], a hands-on guide to writing your own compiler for a substantial subset of the C programming language.

This implementation is a work in progress, so please fork with caution!

[writing-c-compiler]: https://nostarch.com/writing-c-compiler

## Goals by Chapter

- [ ] Ch 1–3: Lexer, parser, expression handling

- [ ] Ch 4–5: Statement parsing, functions

- [ ] Ch 6–7: Locals, stack management

- [ ] Ch 8–10: Control flow, conditionals, loops

- [ ] Ch 11–12: Pointers, arrays, structs

- [ ] Ch 13+: Codegen optimizations, calling conventions, ABI compliance

## 🦀 Prerequisites

You’ll need:

- Rust ≥ 1.79.0 (latest stable recommended)
- Cargo, installed with Rustup
- A C toolchain (GCC or Clang) to assemble and link the output programs
- (Optional) llvm-tools-preview if you plan to emit LLVM IR or link through llc

If you’re new to Rust, install the toolchain using [rustup](https://rustup.rs):

```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Confirm installation:

```console
rustc --version
cargo --version
```

## Building FCC

```console
git clone https://github.com/princemuel/fcc.git # check out the repo
cd fcc
cargo build --release
```

This puts the executable at `target/release/fcc`.

For a faster iteration:

```sh
cargo run -- path/to/source.c
```

## Running the Tests

FCC uses Rust’s built-in test framework for unit and integration tests. There are a handful of unit tests, can be run with this command:

```sh
cargo test
```

For compiler verification against the official [Writing a C Compiler Test Suite][test-suite]:

```console
git clone https://github.com/nlsandler/writing-a-c-compiler-tests.git
cd writing-a-c-compiler-tests
./test_compiler --check-setup # make sure you meet all the system requirements
```

```console
./test_compiler ~/path/to/fcc/target/release -- --chapter 4
```

[test-suite]: https://github.com/nlsandler/writing-a-c-compiler-tests

### Usage Example

Assume we have this source file at ~/hello.c:

```c
int puts(char *c);

int main(void) {
    puts("Hello, world!");
}
```

To compile and run it:

```console
$ target/release/fcc ~/hello.c -o ~/hello
$ ~/hello
Hello, world!
```

### Common Tasks

```console
cargo run -- examples/return_42.c
```

If you use the LLVM backend:

```sh
cargo run --release --backend=llvm examples/hello.c
```

To emit LLVM IR (if enabled) instead of machine code:

```sh
cargo run -- --emit-llvm examples/fib.c
```

## License

MIT License © 2025
Based on the book _Writing a C Compiler_ by Nora Sandler.
Ported and adapted to Rust for educational purposes.
