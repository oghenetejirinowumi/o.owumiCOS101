use std::io;

fn main() {
    println!("Enter lower bound:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("FAILED TO READ ENTRY");
    let lower_bound: u32 = input1.trim().parse().expect("FAILED TO READ ENTRY");

    println!("Enter upper bound:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("FAILED TO READ ENTRY");
    let upper_bound: u32 = input2.trim().parse().expect("FAILED TO READ ENTRY");

    /
    for x in lower_bound..=upper_bound {
        if x > 100 {
            println!("ONLY 100 ENTRIES ALLOWED.");
            break;
        }

        // Staff member's name
        let mut name = String::new();
        println!("Please enter your name:");
        io::stdin().read_line(&mut name).expect("NOT A VALID ENTRY");
        let name = name.trim();

        // Get the number of papers published
        let mut input1 = String::new();
        println!("Please enter the number of papers you have published:");
        io::stdin().read_line(&mut input1).expect("NOT A VALID ENTRY");
        let p: u64 = input1.trim().parse().expect("NOT A VALID NUMBER FOR 'p'");

        // Determine the incentive based on the number of papers published
        let incentive = if p >= 3 && p <= 5 {
            500_000
        } else if p > 5 && p < 10 {
            800_000
        } else if p >= 10 {
            1_000_000
        } else {
            100_000 // Default case for p < 3
        };

        println!("\nStaff Member: {}", name);
        println!("You have published {} papers.", p);
        println!("Your incentive is: N{}", incentive);
    }
}