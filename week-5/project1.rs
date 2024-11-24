use std::io;

fn main() {

    let price_p = 3200;
    let price_f = 3000;
    let price_a= 2500;
    let price_e = 2000;
    let price_w = 2500;

    println!("P = Poundo Yam/Edikaiko Soup  - N 3,000");
    println!("F = Fried Ricw & Chicken  - N 3,000");
    println!("A = Amala & Ewedu Soup  - N 2,500");
    println!("E = Eba & Egusi  - N 2,000");
    println!("W = White Rice & Stew  - N 2,500");

    //  TO ENTER YOUR FOOD ORDER
    let mut food_order = String::new();
    println!("Hello, PLEASE ENTER YOUR ORDER (P, F, A, E, W):");
    io::stdin().read_line(&mut food_order).expect("SORRY, NOT A VALID ENTRY \nPLEASE TRY AGAIN");
    let food_order = food_order.trim().to_uppercase();

    // TO ENTER YOUR FOOD ORDER QUANTITY
    let mut order_quantity = String::new();
    println!("PLEASE ENTER YOUR ORDER QUANTITY");
    io::stdin().read_line(&mut order_quantity).expect("Failed to read entry");
    let order_quantity: i32 = order_quantity.trim().parse().expect("Please enter a valid number");
    
    // Declare a variable to store the total cost
    let mut total_cost = 0;

    // CALCULATE THE COST OF food_order
    if food_order == "P" {
        total_cost = price_p * order_quantity;
    } else if food_order == "F" {
        total_cost = price_f * order_quantity;
    } else if food_order == "A" {
        total_cost = price_a * order_quantity;
    } else if food_order == "E" {
        total_cost = price_e * order_quantity;
    } else if food_order == "W" {
        total_cost = price_w * order_quantity;
    } else {
        println!("Invalid food choice.");
        return; // Exits the program if the food choise is invalid
    }

    // Check if the discount should be applied
    if total_cost > 10_000 {
        total_cost = (total_cost as f64 * 0.95) as i32; // Applying the 5% discount
        println!("A discount has been applied to your order");
    }

    // PRINT THE TOTAL COST
    println!("The total cost of your oder is: N{}", total_cost);
    println!("Thank you for ordering! Please come back again.");


}