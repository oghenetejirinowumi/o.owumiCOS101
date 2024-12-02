use std::io;

fn checker() {
    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("FAILED TO READ INPUT");
    let check:char = input.trim().parse().expect("INVALID INPUT");

    if check >= '0' && check <= '9'
    {
        println!("Charactrer '{}' is a digit",check);
    }
    else
    {
        println!("Character: '{}' is not a digit",check);
    }
}

fn main() {
    //calling the function
    println!("Welcome! This program checks whether a character variable
        contains a digit or not");
    checker()
}