# Learn Rust

```sh
RUSTFLAGS="-Z threads=8" cargo +nightly build --release
```

## Roadmap

### Learn Language Fundamentals (Beginner)

#### Rust

- `TRPL` with Let's Get Rusty (quik high level overview)
- `TRPL` Online Ebook with Rustlings (in-depth)
- `TRPL` Online Ebook (Brown) with `Rust by Practice` and `Rust by Example` (revise)

#### CS

- `TRPL` with Let's Get Rusty (quik high level overview)
- `TRPL` Online Ebook with Rustlings (in-depth)
- `TRPL` Online Ebook (Brown) with `Rust by Practice` and `Rust by Example` (revise)

### ?? (Intermediate)

-

## Resources

1. The Rust Programming Language (Book and Online Ebook)
2. The Rust Programming Language by Brown Uni (Online Ebook)
3. Rustlings (Interactive Code Katas)
4. Rust by Example
5. Rust by Practice (Interactive Code Katas)
6.

## Other Resources

- Ultralearning Scott H. Young
- Learn You a Haskell for Great Good by Maria Lipovaca
- Real World Haskell O'Reilly

## Ownership Rules

1. Each value in Rust has a variable that's called its' owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped
4. `&str` implements the `Copy` trait while `String` implements the `Drop` trait

## Reference Rules

1. At any given time, you can have either one mutable reference, or any number of immutable references
2. References must always be valid
