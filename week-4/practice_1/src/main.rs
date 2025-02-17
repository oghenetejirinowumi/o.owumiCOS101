// Rust program to output name and age

use std::io;

fn main() {
    println!("\nStudent Information Management System");

    // input name
    println!("\nOWUMI OGHENETEJIRIN GREGORY");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Your name is: {}", name.trim());

    // input age
    println!("\n18");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is {}", age);
}    