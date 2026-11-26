use std::ops::{Range,RangeInclusive};


pub fn data_structures_pipeline(){
    arrays();
    ranges();
}

fn arrays(){
    println!("-----Arrays-----");
    // Arrays are fixed-len lists with syntax `[type; len]`
    let nums: [i32; 5] = [1,2,3,4,5];

    // nums.push(5); Fails compilation - cannot modify len
    iter_and_print("Nums", nums);

    // We can init every element with the same value
    let ones: [i32; 5] = [1; 5];
    iter_and_print("Same Elements", ones);

    // ----- OR create a series -----
    let series: [i32; 5] = std::array::from_fn(|i| (i + 1) as i32);
    iter_and_print("Series from_fn", series);
}

fn ranges(){
    // Ranges are their own TYPE, and store the parameters rather than the actual values
    // For example, `1..100` would store the start, end and step, then call the func
    // when the values are iterated or otherwise used. It does NOT store 99ints in menory.

    let range: Range<i32> = 0..10;
    let step_range = (0..10).step_by(2);
    let inc_range: RangeInclusive<i32> = 1..=5;


    iter_and_print("Range", range);
    iter_and_print("Stepped Range", step_range);
    iter_and_print("Inclusive Range", inc_range);   
}

fn iter_and_print<T>(header: &str, iterable: T)
    where
        T: IntoIterator,
        T::Item: std::fmt::Display,
    {
        println!("-----{header}-----");

        for i in iterable {
            println!("{i}");
        }
    }