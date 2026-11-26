
//! # Iterators
//!
//! This module demonstrates the three primary iterator types in Rust.
//!
//! ## Iterator Rules
//!
//! | Method          | Behavior                  | Original Collection |
//! |-----------------|---------------------------|---------------------|
//! | `.iter()`       | Iterates immutable refs   | Preserved           |
//! | `.iter_mut()`   | Iterates mutable refs     | Preserved           |
//! | `.into_iter()`  | Iterates owned elements   | Depends on input    |
//!
//! IMPORTANT:
//! `.into_iter()` does not always consume the underlying collection.
//! Its behavior depends on the type it receives:
//!
//! - Vec<T>       -> Yields owned T values; consumes the Vec.
//! - &Vec<T>      -> Yields &T references; preserves the Vec.
//! - &mut Vec<T>  -> Yields &mut T references; preserves the Vec.
//!
//! `for item in collection` implicitly calls IntoIterator::into_iter().
//! The collection's type determines which implementation is used.

pub fn iter_pipeline() {
    let i_nums = vec![1, 2, 3];
    let mut m_nums = vec![1, 2, 3];

    // Immutable borrowing
    immutable_iteration(&i_nums);

    // Mutable borrowing
    mutable_iteration(&mut m_nums);

    // Ownership transfer
    // m_nums is moved into the function and is no longer accessible here.
    let new_nums: Vec<i32> = ownership_change_iter(m_nums);

    println!("\n-----Returned Collection-----");
    println!("{:?}", new_nums);
}


//----------Immutable Iteration----------

fn immutable_iteration(nums: &Vec<i32>) {
    println!("\n-----Immutable Iteration-----");

    // Implicit iteration:
    // Because nums is &Vec<i32>, this yields &i32.
    // The underlying collection remains accessible.
    for n in nums {
        println!("{n}");
    }

    // Explicit iteration:
    // .iter() also yields &i32.
    for n in nums.iter() {
        println!("{n}");
    }
}


//----------Mutable Iteration----------

fn mutable_iteration(nums: &mut Vec<i32>) {
    println!("\n-----Mutable Iteration-----");

    // Explicit mutable iteration:
    // .iter_mut() yields &mut i32.
    // The original collection is preserved.
    for n in nums.iter_mut() {
        println!("{n}");
    }

    // Mutate original values in place.
    for n in nums.iter_mut() {
        *n += 2;
    }

    // Immutable iteration after mutation.
    // The previous mutable borrows have ended.
    println!("Mutated Values:");

    for n in nums.iter() {
        println!("{n}");
    }
}


//----------Ownership Transfer Iteration----------

fn ownership_change_iter(nums: Vec<i32>) -> Vec<i32> {
    println!("\n-----Into Iter-----");

    // nums is an owned Vec<i32>, not a reference.
    // Calling into_iter() consumes the Vec and yields owned i32 values.
    //
    // No cloning is necessary to transfer qualifying values
    // into another collection.

    let mut out_list: Vec<i32> = Vec::new();

    for n in nums.into_iter() {
        // n is an owned i32, so no dereferencing is necessary.
        if n % 2 != 0 {
            out_list.push(n);
        }
    }

    // Return the new collection.
    // The original collection has been consumed.
    out_list
}
