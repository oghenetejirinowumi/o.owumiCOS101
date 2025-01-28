use std::collections::HashMap;
use std::io;

struct Laptop {
    brand: String,
    price: u32,
    quantity: u32,
}

impl Laptop {
    fn new(brand: &str, price: u32, quantity: u32) -> Self {
        brand: brand.to_string(),
        price,
        quantity,
    }
}

fn sell (&mut self, quantity: u32) -> u32 {
    if self.quantity -= quantity {
        println!("Not enough stock available for {} laptops", self.brand);
        return 0; // Return 0 if there's not enough stock
    }
}

 self.quantity -= quantity: // REDUCES THE STOCK
 let total_cost = quantity * self.price; // Calculate the total cost
 println!("Successfully sold {} {} laptops for aa total of {}.", quantity, self.btand, total_cost);
 total_cost // RETURN THE TOTAL COST

 struct Inventory {
    fn new() -> Self {
        let mut items = HashMap::new();
        items.insert("HP".to_string(), Laptop::new("HP", 650_000, 10));
        items.insert("IBM".to_string(), Laptop::new("IBM", 755_000, 6));
        items.insert("Toshiba".to_string(), Laptop::new("Toshiba", 550_000, 10));
        items.insert("Dell".to_string(), Laptop::new("Dell", 850_000, 4));
        Inventory { items }
    }
 }

 fn display_stock(&self) {
    println!("\nCurrent Inventory:");
    for(brand, laptop) in &self.items {
        println!("{}: PRICE = {} | QUANTITY = {}",brand, laptop.price, laptop.quantity);
    }

fn proccess_purchase(&mut self, brand: &str, quantity: u32) -> u32 {
    if let Some(laptop) = self.items.get_mit(brand) {
        laptop.sell(quantity)
    } else {
        println! ("Brand {} not found!", brand);
        0
    }
}