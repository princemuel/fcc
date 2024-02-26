# Learn Rust

```sh
RUSTFLAGS="-Z threads=8" cargo +nightly build --release
```

## Ownership Rules

1. Each value in Rust has a variable that's called its' owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped
4. `&str` implements the `Copy` trait while `String` implements the `Drop` trait

## Reference Rules

1. At any given time, you can have either one mutable reference, or any number of immutable references
2. References must always be valid
