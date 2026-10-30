# Getting Started in Rust

## Setup 

### Downloading/Installing
- Rust includes the compiler (rustc) and Cargo (the project manager/build tool) in its download. 
- Rust can be downloaded at https://rust-lang.org/tools/install/
- Confirm download `rustup --version` (Windows)
- Update where necessary `rustup update`

### Using Rustup & Cargo
- Full Docs: https://doc.rust-lang.org/cargo/index.html
- **Build tool/Package manager**
    | Desire        | Cmd               |
    | :--           | :--               |
    | Create New    | `cargo new <x>`   |
    | Build         | `cargo build`     |
    | Run           | `cargo run`       |
    | Build Docs    | `cargo doc`       |
    | Publish Lib   | `cargo publish`   |

#### General Starting Procedure
| Desc                              | CMD                                       |
| :--                               | :--                                       |
| Start new project dir             | `cargo new <project-name>`                |
| Start new project in existing dir | `cargo init .`                            |
| Install deps (Optional)           | `cargo add <dep>`                         |
| Build (Debug)                     | `cargo build`                             |
| Build (Release)                   | `cargo build --release`                   |
| Run (Debug)                       | `cargo run`                               |
| Run (Release)                     | `cargo run --release`                     |
| Check for errors (without build)  | `cargo check`                             |
| Run tests                         | `cargo test`                              |
| Format source code                | `cargo fmt`                               |
| Lint source code                  | `cargo clippy`                            |
| Update dependencies               | `cargo update`                            |
| List dependencies                 | `cargo tree`                              |
| Clean build artifacts             | `cargo clean`                             |


### Dependencies
- Find dependencies ("Crates" in rust) from https://crates.io/
- Add by either:
    - Adding to the `Cargo.toml` under `[dependencies]`
        - `ferris-says = "0.3.1"`
    - Run command `cargo add <dep>`
        - `cargo add ferris-says`
- Once added, run command `cargo build` to install the dep.

#### Using Deps
- Import crate with `use <crate_name>::<import>;`
    - Ex: `use ferris_says::say;` | `use std::io::{stdout, BufWriter}`

## File Structure

### Standard Project Layout
```text
project_name/
├── Cargo.toml          # Project metadata, dependencies, build settings
├── Cargo.lock          # Exact dependency versions
├── src/
│   ├── main.rs         # Executable entry point
│   ├── lib.rs          # Optional library entry point
│   └── modules/
│       └── example.rs  # Additional source modules
├── tests/              # Integration tests
├── examples/           # Example programs
└── target/             # Generated build artifacts
```

### Key Files
- **`Cargo.toml`** — Project configuration and dependencies.
- **`Cargo.lock`** — Locked dependency versions; normally commit this for applications.
- **`src/main.rs`** — Entry point for executable projects.
- **`src/lib.rs`** — Entry point for reusable library projects.
- **`target/`** — Automatically generated build output; do not edit manually.

### Modules
- Source files use the `.rs` extension and typically `snake_case` filenames.
- Declare a module with `mod example;` to load `src/example.rs` or `src/example/mod.rs`.
- Access public items from another module using `pub` and paths such as `crate::example::MyStruct`.
- Use `use` to bring names into scope, e.g., `use std::collections::HashMap;`.

### Common Commands
- `cargo new project_name` — Create a new project.
- `cargo build` — Compile the project.
- `cargo run` — Compile and run the executable.
- `cargo test` — Run tests.

---
---

# General Structure

Rust organizes code through **packages, crates, modules, and workspaces**. Each serves a different purpose, from organizing functions within a file to coordinating multiple applications.

## 1. Definitions

| Term | Definition | Purpose |
|---|---|---|
| **Package** | A Cargo project defined by a `Cargo.toml` file. | Groups crates, dependencies, and configuration. |
| **Crate** | A compilation unit with its own root file and module tree. | Defines a binary or library compilation target. |
| **Binary crate** | A crate that produces an executable, conventionally rooted at `main.rs`. | Starts and runs an application through `fn main()`. |
| **Library crate** | A crate that produces reusable functionality, conventionally rooted at `lib.rs`. | Exposes functions, types, and modules for other crates to use. |
| **Module** | A namespace within a crate, declared using `mod`. | Organizes code, groups related functionality, and controls visibility. |
| **Workspace** | A collection of Cargo packages managed together. | Coordinates dependencies, builds, tests, and shared development across multiple packages. |

**Important distinctions:**

- A package can contain both a binary crate and a library crate.
- A crate can contain many nested modules.
- A workspace can contain many packages, including multiple independent executables and reusable libraries.
- A library crate is not a separate running process. It executes inside the application that uses it.
- A workspace does not need its own `main.rs` or `lib.rs`.

## 2. Basic package structure

A simple package can contain both a binary and a library.

```text
file_structure/                 <- Package
├── Cargo.toml                  <- Package configuration
├── Cargo.lock                  <- Locked dependency versions
└── src/
    ├── main.rs                 <- Binary crate root
    ├── lib.rs                  <- Library crate root
    ├── stats.rs                <- Library module
    └── calculator/
        ├── mod.rs              <- Calculator module
        └── operations.rs       <- Nested module
```

### `Cargo.toml`

Defines the package name, version, Rust edition, dependencies, and optional target configuration.

### `src/main.rs`

The default root of the binary crate. Contains `fn main()`, which serves as the executable's entry point.

### `src/lib.rs`

The default root of the library crate. Declares its modules and determines which functionality is publicly exposed.

### Modules

`stats.rs`, `calculator/mod.rs`, and `calculator/operations.rs` organize functionality inside the library crate. They are not independent crates simply because they have separate files.

A package can have both `main.rs` and `lib.rs`. In that case, the binary can use the library through the package's crate name.

## 3. Nested modules and namespaces

Modules can be nested to organize code into logical hierarchies.

For example:

```text
src/
├── main.rs
└── calculator/
    ├── mod.rs
    └── simple/
        ├── mod.rs
        └── addition.rs
```

### Declaring the module hierarchy

Inside `calculator/mod.rs`:

```rust
pub mod simple;
```

Inside `calculator/simple/mod.rs`:

```rust
pub mod addition;
```

Inside `calculator/simple/addition.rs`:

```rust
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

The declaration is `pub mod addition;`, not `pub mod addition.rs;`. Rust resolves the module declaration to the corresponding source file.

### Accessing the function

If `calculator` is declared as a module in the same binary crate, `main.rs` can contain:

```rust
mod calculator;

use calculator::simple::addition::add;

fn main() {
    let result = add(10, 20);
    println!("{result}");
}
```

Alternatively, call the function through its full path:

```rust
mod calculator;

fn main() {
    let result = calculator::simple::addition::add(10, 20);
    println!("{result}");
}
```

### What the syntax means

| Syntax | Meaning |
|---|---|
| `mod calculator;` | Declares a module and includes its source file in the crate's module tree. |
| `pub mod simple;` | Declares a publicly accessible child module. |
| `use calculator::simple::addition::add;` | Brings `add` into the current scope. |
| `calculator::simple::addition::add(...)` | Calls `add` through its fully qualified path relative to the current scope. |
| `pub use addition::add;` | Re-exports `add` through the current module's public interface. |

Rust defaults to private visibility. A public function inside a private module is not automatically accessible from outside that module's visibility boundary. Every enclosing module along an external access path must be accessible.

**Note:** `use calculator;` is not required merely to use a module declared in the current crate. `mod calculator;` declares the module; `use` imports names into scope.

## 4. Organizing multiple applications and libraries

For a larger application, a single package can become cumbersome. A Cargo workspace allows the project to be divided into multiple packages while keeping them managed together.

### Recommended structure

```text
market_analyzer/                    <- Workspace root
├── Cargo.toml                      <- Workspace configuration
├── Cargo.lock                      <- Shared workspace lockfile
│
├── apps/
│   ├── node_control/               <- Binary package
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs             <- Starts Node Control
│   │
│   ├── engine/                     <- Binary package
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs             <- Starts Engine
│   │
│   ├── feeder/                     <- Binary package
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs             <- Starts Feeder
│   │
│   └── account/                    <- Binary package
│       ├── Cargo.toml
│       └── src/
│           └── main.rs             <- Starts Account/Broker
│
└── crates/
    ├── types/                      <- Library package
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    │
    ├── nats/                       <- Library package
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    │
    └── engine_core/                <- Library package
        ├── Cargo.toml
        └── src/
            ├── lib.rs
            ├── indicators.rs
            ├── conditions.rs
            ├── positions.rs
            └── metrics.rs
```

The `apps/` and `crates/` directory names are organizational conventions, not special Rust directories. The actual package boundaries are established by the individual `Cargo.toml` files and the workspace configuration.

### Workspace configuration

The root `Cargo.toml` might contain:

```toml
[workspace]
members = [
    "apps/node_control",
    "apps/engine",
    "apps/feeder",
    "apps/account",
    "crates/types",
    "crates/nats",
    "crates/engine_core",
]
resolver = "3"
```

The workspace root does not need `src/`, `main.rs`, or `lib.rs` when it is only a workspace and not itself a package.

Each member package has its own `Cargo.toml`. For example, the Engine package can declare local library dependencies:

```toml
[package]
name = "engine"
version = "0.1.0"
edition = "2024"

[dependencies]
types = { path = "../../crates/types" }
nats = { path = "../../crates/nats" }
engine_core = { path = "../../crates/engine_core" }
```

The Engine can then import the public APIs exposed by those libraries:

```rust
use engine_core::evaluate_indicators;
use types::MarketData;

fn main() {
    // Initialize and run the engine.
}
```

The names above are illustrative; the corresponding library crates must define and expose those APIs.

### Building and running applications

From the workspace root:

```cmd
cargo build --workspace
cargo test --workspace
cargo run -p node_control
cargo run -p engine
```

Each `cargo run -p ...` command launches the specified binary package. The applications can also be launched independently by the operating system or a service manager.

## 5. Alternative: libraries nested inside an application

A library can be located beneath an application's directory and still be a separate package, provided it has its own `Cargo.toml` and is configured as a workspace member or dependency.

For example:

```text
market_analyzer/
├── Cargo.toml
├── apps/
│   └── engine/
│       ├── Cargo.toml
│       ├── src/
│       │   └── main.rs
│       └── engine_core/
│           ├── Cargo.toml
│           └── src/
│               └── lib.rs
└── crates/
    ├── types/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── nats/
        ├── Cargo.toml
        └── src/lib.rs
```

This is valid, but it makes the Engine's private implementation package visually distinct from shared libraries only by location and naming. Its dependency paths and workspace membership still need to be configured.

Use this organization when the library is closely tied to one application. Use a top-level `crates/` directory when the library is shared or expected to be reused.

For small internal components, you may not need another package at all. Modules inside `engine/src/` are often sufficient.

## 6. Choosing the right level of organization

| Need | Recommended structure |
|---|---|
| Organize related functions | Modules within one crate |
| Reuse functionality across crates | Library crate |
| Create an independently launchable service | Binary package |
| Manage multiple services and libraries together | Cargo workspace |
| Keep one application's private implementation separate | Internal modules or a nested library package |
| Share types and message contracts between services | Dedicated library crate |

A crate boundary provides an independent compilation and API boundary, but it does not create a separate operating-system process. Multiple crates can be compiled into and run within the same executable.

## 7. Recommended architecture for Market Analyzer

Use a workspace containing:

- **Binary packages:** Node Control, Engine, Feeder, Account/Broker, and any other component requiring its own process lifecycle.
- **Shared library packages:** Common types, messaging interfaces, and genuinely reusable functionality.
- **Internal modules:** Indicator calculations, condition evaluation, position management, and metrics within `engine_core`, at least initially.

Extract a module into its own library crate only when independent reuse, testing, API boundaries, or dependency management justify the added package.

This approach preserves the distinction between process ownership and code organization: Node Control can govern component lifecycles through messaging, while each service owns its runtime and library crates provide the code each service needs.
