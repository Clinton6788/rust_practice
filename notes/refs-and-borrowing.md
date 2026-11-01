# Borrowing and References in Rust

## Summary
- Arguments without `&` are passed **by value**.
    - **IF AND ONLY IF** the type implements the `Copy` trait, the value is copied and the original remains usable.
    - `i32`, `f64`, `bool`, etc. implement `Copy`.
    - Types such as `String` and `Vec<T>` do not implement `Copy`; passing them transfers ownership (MOVE).
- Borrowing allows a function to access a value without taking ownership.
    - `&T` = immutable reference (read access).
    - `&mut T` = mutable reference (read/write access).
    - References are pointer-like and allow access to the original value without copying its contents.
- **Important distinction:** Moving ownership does NOT necessarily mean passing a pointer/reference. It means transferring responsibility for the value to another owner. The original variable can no longer be used.

#### TL;DR
When the final owner of an object dies, that object and its reference die alongside it - hence borrowing/copying.


### Example: Ownership Transfer (MOVE)
```rust
let x = String::from("Hello");

func2(x);

// println!("{x}"); // ERROR: ownership was transferred

fn func2(value: String) {
    println!("{value}");
}
```

### Example: Copy
```rust
let x: i32 = 5;

func2(x);

println!("{x}"); // Works!

fn func2(value: i32) {
    println!("{value}");
}
```

### Example: Borrowing
```rust
let x = String::from("Hello");

func2(&x);

println!("{x}"); // Works!

fn func2(value: &String) {
    println!("{value}");
}
```