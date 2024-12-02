use std::io;

fn add(a: i32, b: i32) {
    let sum = a + b;

    println!("Sum of A and B = {}" ,sum);
}

fn main() {
    let mut input1 = String::new();
    println!("Enter input for parameter A;");
    io::stdin().read_line(&mut input1).expect("FAILED TO READ INPUT");
    let a:i32 = input1.trim().parse().expect("INVALID INPUT");

    let mut input2 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input2).expect("FAILED TO READ INPUT");
    let b:i32 = input2.trim().parse().expect("INVALID INPUT");

    // call add function with arguements
    add(a,b);
}
