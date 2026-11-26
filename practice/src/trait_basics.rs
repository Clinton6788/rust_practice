
enum OrderType {
    Market,
    Limit
}

trait Order {
    fn get_type(&self) -> &OrderType;
    fn get_quantity(&self) -> u32;
    fn get_type_as_str(&self) -> &str {
        match self.get_type() {
            OrderType::Limit => "Limit",
            OrderType::Market => "Market",
        }
    }

    fn execute(&self) -> String {
        format!(
            "Executing {} Order for: {}",
            self.get_type(), self.get_quantity()
        )
    }

    fn cancel(&mut self) -> String {
        format!(
            "Cancelling {} Order for: {}",
            self.get_type(), self.get_quantity()
        )
    }
}

trait Limit: Order {
    fn get_price(&self) -> f64;
}

struct MarketOrder {
    type_: OrderType,
    quantity: u32,
}

struct LimitOrder {
    type_: OrderType,
    quantity: u32,
    price: f64,
}

impl Order for MarketOrder {
    fn get_type(&self) -> &OrderType {
        &self.type_
    }

    fn get_quantity(&self) -> u32 {
        self.quantity
    }
}

impl Order for LimitOrder {
    fn get_type(&self) -> &OrderType {
        &self.type_
    }

    fn get_quantity(&self) -> u32 {
        self.quantity
    }
}

impl Limit for LimitOrder {
    fn get_price(&self) -> f64 {
        self.price
    }
}




