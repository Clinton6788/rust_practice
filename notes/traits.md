# Rust Traits and Generic Programming

## 1. What Are Traits?

A **trait** defines a set of behaviors that a type promises to implement.

Think of traits as a combination of:
- Interfaces / abstract base classes.
- Protocols.
- Generic constraints.
- Shared behavioral contracts.

Traits allow different types to expose common functionality without requiring them to share the same underlying structure.

### Example: Defining a Trait

```rust
struct Stock {
    symbol: String,
    price: f64,
}

struct Crypto {
    symbol: String,
    price: f64,
}

struct Forex {
    pair: String,
    price: f64,
}
```

Define a common trait:

```rust
trait MarketInstrument {
    fn get_price(&self) -> f64;
}
```

This establishes a requirement: any type implementing `MarketInstrument` must provide a `get_price()` method.

Implement the trait:

```rust
impl MarketInstrument for Stock {
    fn get_price(&self) -> f64 {
        self.price
    }
}

impl MarketInstrument for Crypto {
    fn get_price(&self) -> f64 {
        self.price
    }
}

impl MarketInstrument for Forex {
    fn get_price(&self) -> f64 {
        self.price
    }
}
```

**Syntax distinction:**

```rust
impl MarketInstrument for Stock {}
```

Implements a trait for a type.

```rust
impl Stock {}
```

Defines inherent methods belonging directly to the type.

---

## 2. Trait-Bounded Generics

Traits become particularly useful when writing functions.

Without traits, you might need separate functions for each type:

```rust
fn process_stock(stock: &Stock) {}
fn process_crypto(crypto: &Crypto) {}
fn process_forex(forex: &Forex) {}
```

With traits, you can write one generic function:

```rust
fn process_instrument<T: MarketInstrument>(instrument: &T) {
    let price = instrument.get_price();
    println!("Price: {price}");
}
```

Usage:

```rust
process_instrument(&stock);
process_instrument(&crypto);
process_instrument(&forex);
```

`T: MarketInstrument` means:

- `T` represents a generic type.
- The type can be anything.
- However, it MUST implement `MarketInstrument`.

This is called a **trait bound**.

The compiler ensures that the supplied type satisfies the required behavior.

You do not need to convert different types into a common struct.

---

## 3. Static Dispatch vs. Dynamic Dispatch

Rust provides two primary ways to use traits when accepting different types.

### A. Static Dispatch (Generics)

```rust
fn process<T: MarketInstrument>(instrument: &T) {
    println!("{}", instrument.get_price());
}
```

Rust generates specialized versions of the function for the concrete types used.

Conceptually:

```text
process<Stock>()
process<Crypto>()
process<Forex>()
```

This process is called **monomorphization**.

Advantages:
- No runtime type lookup.
- Excellent performance potential.
- Compiler can optimize each specialized implementation.
- Strong compile-time type checking.

Disadvantages:
- Can increase compiled binary size.
- Each concrete type generally gets its own compiled version.

### B. Dynamic Dispatch (`dyn Trait`)

```rust
fn process(instrument: &dyn MarketInstrument) {
    println!("{}", instrument.get_price());
}
```

Usage:

```rust
process(&stock);
process(&crypto);
process(&forex);
```

Instead of generating a specialized function for every type, Rust uses a **vtable (virtual method table)** to determine which implementation to execute at runtime.

Advantages:
- One function can work with different concrete types at runtime.
- Useful for heterogeneous collections.
- Can reduce code duplication in compiled output.

Disadvantages:
- Runtime dispatch overhead.
- Some compiler optimizations become harder.
- Trait objects have restrictions involving dyn compatibility.

### When to Use Each

| Situation | Preference |
|---|---|
| Performance-sensitive calculations | Generics |
| Known types at compile time | Generics |
| Collection of different implementing types | `dyn Trait` |
| Runtime-selected implementations | `dyn Trait` |
| Plugin-style architecture | Often `dyn Trait` |

Neither approach is universally superior.

---

## 4. Default Trait Implementations

Traits can provide default implementations.

You don't need to implement every method individually.

```rust
trait MarketInstrument {
    fn get_price(&self) -> f64;

    fn get_value(&self, quantity: f64) -> f64 {
        self.get_price() * quantity
    }
}
```

Here:
- `get_price()` is required.
- `get_value()` has a default implementation.

An implementing type automatically receives `get_value()` unless it overrides it.

```rust
impl MarketInstrument for Stock {
    fn get_price(&self) -> f64 {
        self.price
    }
}
```

Usage:

```rust
let value = stock.get_value(100.0);
```

This is useful when multiple types share common behavior.

---

## 5. Supertraits (Trait Dependencies)

Traits can require other traits.

Rust calls this a **supertrait** relationship.

```rust
trait MarketInstrument {
    fn get_price(&self) -> f64;
}

trait Tradable: MarketInstrument {
    fn execute_order(&self, quantity: f64);
}
```

The colon establishes that any type implementing `Tradable` must also implement `MarketInstrument`.

Example:

```rust
impl MarketInstrument for Stock {
    fn get_price(&self) -> f64 {
        self.price
    }
}

impl Tradable for Stock {
    fn execute_order(&self, quantity: f64) {
        println!("Executing {quantity} units");
    }
}
```

You can also require multiple traits:

```rust
fn execute<T: MarketInstrument + Tradable>(instrument: &T) {
    // ...
}
```

Or use a `where` clause:

```rust
fn execute<T>(instrument: &T)
where
    T: MarketInstrument + Tradable,
{
    // ...
}
```

Both approaches express the same constraints.

---

## 6. Associated Types

An associated type allows a trait to specify that implementing types must provide a particular type.

```rust
trait DataSource {
    type Output;

    fn fetch(&self) -> Self::Output;
}
```

Different implementations can return different types:

```rust
struct MarketFeed;
struct AccountFeed;

impl DataSource for MarketFeed {
    type Output = Vec<f64>;

    fn fetch(&self) -> Self::Output {
        vec![100.0, 101.0, 102.0]
    }
}

impl DataSource for AccountFeed {
    type Output = String;

    fn fetch(&self) -> Self::Output {
        String::from("Account data")
    }
}
```

The trait defines the required behavior, but each implementation determines its output type.

This is useful when different implementations have different data structures but share the same conceptual interface.

---

## 7. Blanket Implementations

A **blanket implementation** implements a trait automatically for every type satisfying certain requirements.

Example:

```rust
trait Loggable {
    fn log(&self);
}

impl<T: std::fmt::Debug> Loggable for T {
    fn log(&self) {
        println!("{:?}", self);
    }
}
```

Every type implementing `Debug` automatically implements `Loggable`.

You don't need to implement `Loggable` individually for each type.

This is useful for building reusable functionality across large groups of types.

### Important Restriction: Coherence / Orphan Rules

Rust prevents conflicting trait implementations.

Generally, you cannot implement a foreign trait for a foreign type.

For example, you cannot freely implement an external library's trait for another external library's type.

This prevents conflicting implementations across dependencies.

---

## 8. Standard Library Traits

Rust already uses traits extensively.

| Trait | Purpose |
|---|---|
| `Copy` | Allows implicit copying of values |
| `Clone` | Explicitly duplicates values |
| `Debug` | Supports debug formatting (`{:?}`) |
| `Display` | Supports user-facing formatting (`{}`) |
| `Default` | Provides a default value |
| `PartialEq` | Supports equality comparisons |
| `Ord` | Supports ordering |
| `Send` | Allows ownership transfer between threads |
| `Sync` | Allows references to be shared between threads |
| `From` / `Into` | Defines conversions between types |
| `AsRef` | Provides a reference to another representation |

### Example: Deriving Traits

```rust
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
```

`derive` automatically implements these traits when the type's fields support them.

Usage:

```rust
let p1 = Point { x: 1.0, y: 2.0 };
let p2 = p1;

println!("{:?}", p1);
```

Because `Point` implements `Copy`, assigning `p1` to `p2` copies it rather than moving ownership.

---

## 9. Callable Traits: Fn, FnMut, FnOnce

Rust has traits specifically for callable objects, including closures.

| Trait | Meaning |
|---|---|
| `Fn` | Can be called repeatedly without mutating captured state |
| `FnMut` | Can be called repeatedly and may mutate captured state |
| `FnOnce` | Can be called by consuming captured state; not necessarily callable again |

### Example

```rust
fn execute<F: Fn(i32) -> i32>(operation: F) -> i32 {
    operation(5)
}

fn main() {
    let result = execute(|x| x * 2);

    println!("{result}"); // 10
}
```

Here:
- `F` represents a callable type.
- `Fn(i32) -> i32` establishes the required callable interface.
- `|x| x * 2` is a closure implementing `Fn`.

### Why Are These Useful?

They allow functions to accept other functions or closures as arguments.

Common applications:
- Callbacks.
- Event handlers.
- Iterators.
- Custom sorting.
- Asynchronous tasks.
- Worker execution.

---

## 10. Trait Architecture Overview

| Concept | Purpose |
|---|---|
| Trait definition | Defines required behavior |
| `impl Trait for Type` | Connects a concrete type to a trait |
| Trait bounds | Restricts which types a generic function accepts |
| Static dispatch | Resolves implementations at compile time |
| Dynamic dispatch | Resolves implementations at runtime |
| Default implementation | Provides reusable behavior |
| Supertrait | Establishes trait dependencies |
| Associated type | Allows implementations to specify related types |
| Blanket implementation | Automatically implements traits for qualifying types |
| `Fn`, `FnMut`, `FnOnce` | Define callable behavior |

### Mental Model

Traits separate **what a component must be able to do** from **how that component actually does it**.

A type can have its own internal structure and implementation while still satisfying a shared behavioral contract.

This enables:
- Interchangeable implementations.
- Reusable generic functions.
- Abstraction without requiring a common parent struct.
- Compile-time behavioral validation.
- Runtime polymorphism when needed.

---

## 11. Practice Exercises

Use the market-instrument example to experiment with traits.

1. Define `Stock`, `Crypto`, and `Forex` structs.
2. Define a `MarketInstrument` trait with `get_price()`.
3. Implement the trait for all three structs.
4. Write a generic function accepting `T: MarketInstrument`.
5. Change that function to accept `&dyn MarketInstrument`.
6. Add a default method to the trait.
7. Add a second trait called `Tradable` that requires `MarketInstrument`.
8. Create a `Vec<Box<dyn MarketInstrument>>` containing all three instrument types.

The final exercise introduces **trait objects in collections**: a single collection containing different concrete types, all accessible through a common interface.

### Design Consideration

Traits are useful when you genuinely need:
- Shared behavior.
- Interchangeable implementations.
- Generic constraints.
- Runtime polymorphism.

Not every struct needs a trait. Avoid introducing abstractions without a practical reason.