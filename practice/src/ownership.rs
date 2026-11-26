
// ! # Ownership Model
//!
//! This crate primarily focuses on the ownership model,
//! borrowing, and mutability. Additional Rust concepts are
//! also practiced here as this is the first official practice crate.
//!
//! ## Ownership and Borrowing
//!
//! Rust distinguishes between ownership and borrowing:
//!
//! - Ownership: A variable owns its allocated value.
//! - Immutable Borrowing (`&T`): Allows reading without taking ownership.
//! - Mutable Borrowing (`&mut T`): Allows modifying without taking ownership.
//!
//! ## Mutability vs. Ownership
//!
//! `mut` and `&mut` are NOT interchangeable. They describe
//! two different properties.
//!
//! ### 1. Binding Mutability: `mut`
//!
//! `mut` is placed BEFORE the variable/argument name.
//! It determines whether the local variable binding can be modified
//! or reassigned.
//!
//!     let mut x: i32 = 10;
//!
//! Function argument examples:
//!
//!     fn process(x: Vec<i32>) { }
//!     fn process(mut x: Vec<i32>) { }
//!
//! - `x: Vec<i32>`: Takes ownership, immutable local binding.
//! - `mut x: Vec<i32>`: Takes ownership, mutable local binding.
//!
//! IMPORTANT:
//! Taking ownership does NOT require the variable to be mutable.
//! A value can be moved into a function regardless of whether
//! the caller's original binding was declared `mut`.
//!
//! ### 2. Reference Mutability: `&mut`
//!
//! `&mut` is part of the TYPE and is placed AFTER the colon.
//! It determines whether the reference allows modification
//! of the underlying value.
//!
//! Function argument examples:
//!
//!     fn process(x: &Vec<i32>) { }
//!     fn process(x: &mut Vec<i32>) { }
//!
//! - `x: &Vec<i32>`: Immutable reference; read-only access.
//! - `x: &mut Vec<i32>`: Mutable reference; allows modifying
//!   the original vector.
//!
//! IMPORTANT:
//! You do NOT need `mut` before the argument name to modify
//! the referenced value. The `&mut` type already provides
//! that permission.
//!
//! ### 3. Combining Both
//!
//!     fn process(mut x: &mut Vec<i32>) { }
//!
//! This combines both properties:
//!
//! - `&mut Vec<i32>`: Allows modification of the referenced vector.
//! - `mut x`: Allows reassignment of the local reference binding.
//!
//! You generally only need the additional `mut` when you intend
//! to reassign the reference itself.
//!
//! ### Quick Reference
//!
//! | Declaration       | Owns Value? | Can Modify Value? | Can Reassign Binding? |
//! |-------------------|-------------|-------------------|-----------------------|
//! | `x: Vec<i32>`     | Yes         | No                | No                    |
//! | `mut x: Vec<i32>` | Yes         | Yes               | Yes                   |
//! | `x: &Vec<i32>`    | No          | No                | No                    |
//! | `x: &mut Vec<i32>`| No          | Yes               | No                    |
//! | `mut x: &mut Vec<i32>` | No     | Yes               | Yes                   |
//!
//! Note: Interior mutability (Cell, RefCell, Mutex, etc.) provides
//! additional ways to modify values through otherwise immutable
//! references. These will be covered separately.


//----------Ownership Pipeline----------

pub fn ownership_pipeline() {
    println!("Hello, world!");

    //----------Immutable Borrowing----------
    let immutable_x: i32 = 5;
    let immutable_y: i32 = 6;

    // Explicit reference passing
    let ref_i_x: &i32 = &immutable_x;
    let ref_i_y: &i32 = &immutable_y;

    borrowing_immutable(ref_i_x, ref_i_y);

    // Direct reference passing
    borrowing_immutable(&immutable_x, &immutable_y);

    println!("\n-----Can Still Access Originals-----");
    // Borrowing does not transfer ownership.
    println!("Original X = {immutable_x}");
    println!("Original Y = {immutable_y}");


    //----------Mutable Borrowing----------
    let mut mutable_x: i32 = 5;
    let mut mutable_y: i32 = 6;

    println!("\n-----Mutable Variables Borrowing-----");
    println!("Original X = {mutable_x}");
    println!("Original Y = {mutable_y}");

    borrowing_mutable(&mut mutable_x, &mut mutable_y, false);
    println!("Values in Main: X={mutable_x}, Y={mutable_y}");

    borrowing_mutable(&mut mutable_x, &mut mutable_y, true);
    println!("Values in Main: X={mutable_x}, Y={mutable_y}");


    //----------Ownership vs. Borrowing----------
    println!("\n-----Ownership vs. Borrowing-----");

    // The caller does NOT need to declare this mutable.
    // Ownership is transferred into the function.
    // Mutability is specified by the receiving function.
    let owned_values = vec![1, 2, 3];

    let returned_values = take_ownership_and_modify(owned_values);

    println!("Returned Values: {:?}", returned_values);


    // The caller MUST have a mutable binding to create
    // a mutable reference to its original vector.
    let mut borrowed_values = vec![1, 2, 3];

    modify_through_borrow(&mut borrowed_values);

    println!("Original Values: {:?}", borrowed_values);
}


//----------Immutable Borrowing----------

fn borrowing_immutable(x: &i32, y: &i32) {
    println!("\n-----Immutable Borrow-----");

    // This works because Rust implements multiplication
    // for references to primitive numeric types.
    //
    // This is operator trait support, not a universal
    // rule that immutable references never need dereferencing.
    let r: i32 = x * y;

    println!("Result = {r}");
}


//----------Mutable Borrowing----------

fn borrowing_mutable(x: &mut i32, y: &mut i32, inplace: bool) {

    // {:p} prints the address of the referenced value.
    println!("\nMemory address X: {:p}", x);
    println!("Memory address Y: {:p}", y);

    if inplace {
        // Explicit dereferencing accesses the underlying values.
        *x += 10;
        *y += 10;

        println!("Modifying Inplace. X={x}, Y={y}");
    } else {
        // &mut i32 does not support this particular addition
        // operation directly, so explicitly dereference.
        //
        // These calculations do NOT modify the originals.
        let temp_x = *x + 10;
        let temp_y = *y + 10;

        println!("Did Not Mutate. X={temp_x}, Y={temp_y}");
    }

    // The if expression is being used as a statement here.
    // No semicolon is necessary after its closing brace.
}


//----------Ownership Transfer / Mutable Binding----------

fn take_ownership_and_modify(mut values: Vec<i32>) -> Vec<i32> {

    // `values` owns the vector.
    // `mut` makes its local binding mutable.
    // The caller does not need to declare its original binding mutable.

    values.push(4);
    values.push(5);

    // Ownership is transferred back to the caller.
    values
}


//----------Mutable Borrowing / Mutable Reference----------

fn modify_through_borrow(values: &mut Vec<i32>) {

    // `values` does not own the vector.
    // &mut provides permission to modify the original.
    // No `mut` before the argument name is necessary.

    values.push(4);
    values.push(5);
}
