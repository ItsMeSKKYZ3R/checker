# Checker

## By SKKYZUS#5453

---------------------

### All about this crate

This crate is a checking crate that make the type checking easier in Rust.

---------------------

### Usage

```rust
use checking::Checker;

fn main() {
    let result: bool = Checker::type_of::<i32>(&"test".to_string());

    match result {
        true => println!("Correct type"),
        false => println!("Incorrect type"),
    }

    // Expected result: Correct type
}
```

### See the doc on https://docs.rs/checking