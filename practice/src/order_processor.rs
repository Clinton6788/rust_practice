
enum OrderStatus {
    Pending,
    PartiallyFilled(u32),
    Filled(u32),
    Cancelled,
}


pub fn pattern_and_destructing() {
    println!("\n-----Pattern and Destructing-----");

    let mut status = OrderStatus::Pending;
    describe_order(&status);

    status = OrderStatus::PartiallyFilled(5);
    println!("Order Filled: {}", is_filled(&status));
    println!("Large Fill: {}", is_large_fill(&status));
    println!("{}",describe_order(&status));

    status = OrderStatus::Filled(50);
    println!("Order Filled: {}", is_filled(&status));
    println!("Large Fill: {}", is_large_fill(&status));
    println!("{}",describe_order(&status));

    status = OrderStatus::Filled(150);
    println!("Order Filled: {}", is_filled(&status));
    println!("Large Fill: {}", is_large_fill(&status));
    println!("{}",describe_order(&status));

    status = OrderStatus::Cancelled;
    println!("{}",describe_order(&status));

}

fn describe_order(status: &OrderStatus) -> String{
    match status {
        OrderStatus::Pending => String::from("Awaiting Execution"),
        OrderStatus::PartiallyFilled(n) => format!("Partially Filled: {n}"),
        OrderStatus::Filled(n) => format!("Fully Filled: {n}"),
        OrderStatus::Cancelled => String::from("Order Cancelled"),
    }
}

fn is_filled(status: &OrderStatus) -> bool {
    matches!(
        status,
        OrderStatus::PartiallyFilled(_) | OrderStatus::Filled(_)
    )
}

fn is_large_fill(status: &OrderStatus) -> bool {
    matches!(
        status,
        OrderStatus::Filled(n) if *n >= 100
    )
}