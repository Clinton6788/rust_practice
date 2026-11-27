//! # Rust Commenting and Documentation Demonstration
//!
//! This file demonstrates the various ways to document and comment
//! Rust code.
//!
//! Documentation comments support **Markdown**.
//!
//! ## Contents
//! - Ordinary comments
//! - Doc comments
//! - Struct and field documentation
//! - Enum documentation
//! - Constants
//! - Functions
//! - Markdown features

// ============================================================
// 1. BLOCK COMMENTS
// ============================================================

/*
This is a block comment.

    /*
    This is a nested block comment.
    Rust supports nested block comments.
    */
*/

// ============================================================
// 2. INLINE COMMENTS
// ============================================================

fn main() {
    // This is an inline comment.

    let x = 10; // This is an inline comment beside code.

    // Variables themselves don't have a dedicated rustdoc
    // hover-documentation mechanism within a function.
    let y = 20;

    let result = add(x, y);

    println!("Result: {result}");

    let result = do_addition(x, y);

    println!("Result: {result}");

    let config = Config::new(100, true);

    println!("Max workers: {}", config.max_workers);

    let mode = EngineMode::Active;

    println!("Mode: {mode:?}");
}

// ============================================================
// 3. FUNCTION DOCUMENTATION
// ============================================================

/// Adds two numbers together.
///
/// # Arguments
///
/// * `x` - The first number.
/// * `y` - The second number.
///
/// # Returns
///
/// The sum of `x` and `y`.
///
/// # Example
///
/// ```
/// let result = 5 + 10;
/// assert_eq!(result, 15);
/// ```
///
/// # Notes
///
/// This is a basic addition function.
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// ============================================================
// 4. MARKDOWN DOCUMENTATION
// ============================================================

/// Does addition things and explores Rust's documentation.
///
/// # Obeys Markdown
///
/// - Fully?
/// - *Italicized*
/// - **Bold**
/// - `Code`
/// - ~~Strikethrough~~
///
/// ## Title 2
///
/// ### Title 3
///
/// #### Title 4
///
/// # Table Example
///
/// | Basic | Description |
/// | :-- | :-- |
/// | `data` | *Description* |
/// | `x` | **Bold description** |
///
/// # Code Block
///
/// ```rust
/// let x = 10;
/// let y = 20;
/// let result = x + y;
/// assert_eq!(result, 30);
/// ```
///
/// # Links
///
/// [Rust Documentation](https://doc.rust-lang.org/)
///
/// # Important
///
/// Documentation comments are converted into documentation
/// attributes and can be viewed through IDE tooling.
///
/// # Arguments
///
/// * `x` - First number.
/// * `y` - Second number.
///
/// # Returns
///
/// The result of adding `x` and `y`.
fn do_addition(x: i32, y: i32) -> i32 {
    // This is an ordinary comment inside the function.

    // Attempt to doc a local variable:
    // /// This is not useful as variable hover documentation.
    // let z = &x;

    // Instead, use ordinary comments for local variables.
    let z = &x;

    let a: i32 = add(*z, y);

    a
}

// ============================================================
// 5. STRUCT DOCUMENTATION
// ============================================================

/// Configuration settings for an engine.
///
/// This struct contains settings that determine
/// the behavior of an engine instance.
struct Config {
    /// Maximum number of workers the engine can utilize.
    max_workers: usize,

    /// Whether the engine should automatically restart workers.
    auto_restart: bool,
}

impl Config {
    /// Creates a new configuration.
    ///
    /// # Arguments
    ///
    /// * `max_workers` - Maximum worker count.
    /// * `auto_restart` - Whether to restart failed workers.
    ///
    /// # Returns
    ///
    /// A new `Config` instance.
    fn new(max_workers: usize, auto_restart: bool) -> Self {
        Self {
            max_workers,
            auto_restart,
        }
    }
}

// ============================================================
// 6. ENUM DOCUMENTATION
// ============================================================

/// Represents the operational mode of an engine.
#[derive(Debug)]
enum EngineMode {
    /// The engine is actively processing tasks.
    Active,

    /// The engine is idle.
    Idle,

    /// The engine is shutting down.
    Shutdown,
}

// ============================================================
// 7. CONSTANT DOCUMENTATION
// ============================================================

/// Default number of workers used when no configuration is provided.
const DEFAULT_WORKERS: usize = 4;

// ============================================================
// 8. MODULE DOCUMENTATION EXAMPLE
// ============================================================

// If this were a separate module/file, you could use:
//
// //! # Engine Module
// //!
// //! Responsible for managing engine lifecycle,
// //! workers, scheduling and execution.
//
// ============================================================
// 9. DOCUMENTATION CONVENTIONS
// ============================================================
//
// /// Documents the item below it (function, struct, enum, etc.)
//
// //! Documents the containing module
//
// // Ordinary comment
//
// /* Block comment */
//
// /* Nested /* block */ comment */
//
// ============================================================