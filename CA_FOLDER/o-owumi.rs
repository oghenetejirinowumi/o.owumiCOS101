// QUESTION 1 FOR CA

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

    for x in lower_bound..=upper_bound {
        if x > 50 {
            println!("ONLY 100 ENTRIES ALLOWED.");
            break;
        }
    // checks if the candidate is a current class rep
    let mut class_rep = String::new();
    println!("Are you a current class rep? (YES/NO):");
    io::stdin().read_line(&mut class_rep).expect("NOT A VALID ENTRY");
    let class_rep = class_rep.trim().to_uppercase() == "YES";

    // checks if the candidate is not in 100 level
    let mut lvl = String::new();
    println!("What level are you in? (100, 200, 300, 400, 500):");
    io::stdin().read_line(&mut lvl).expect("NOT A VALID ENTRY");
    let lvl: u32 = lvl.trim().parse().expect("INVALID ENTRY");

    // Ensure the level is not 100 (candidate must not be in 100 level)
    if lvl == 100 {
        println!("Sorry, you are not eligible to vote because you are in 100 level.");
        return; // This if statement ensures that the candidate is not in 100 level
    }

    // checks if the candidate has a CGPA above 4.0
    let mut cgpa = String::new();
    println!("Please enter your CGPA:");
    io::stdin().read_line(&mut cgpa).expect("NOT A VALID ENTRY");
    let cgpa: f64 = cgpa.trim().parse().expect("INVALID ENTRY FOR CGPA");

    // Collects candidate details if eligible
    if class_rep && lvl != 100 && cgpa > 4.0 {
        // Get the candidate's details
        let mut name = String::new();
        println!("Please enter your Name:");
        io::stdin().read_line(&mut name).expect("NOT A VALID ENTRY");
        let name = name.trim(); 
        
        let mut email = String::new();
        println!("Please enter your Email:");
        io::stdin().read_line(&mut email).expect("NOT A VALID ENTRY");
        let email = email.trim(); 

        let mut department = String::new();
        println!("Please enter your Department:");
        io::stdin().read_line(&mut department).expect("NOT A VALID ENTRY");
        let department = department.trim(); 

        let mut state_of_origin = String::new();
        println!("Please enter your State of Origin:");
        io::stdin().read_line(&mut state_of_origin).expect("NOT A VALID ENTRY");
        let state_of_origin = state_of_origin.trim(); 

        // DISPLAY THE FOLLOWING DETAILS OF CANDIDATES
        println!("\nCandidate Details:");
        println!("Name: {}", name);
        println!("Email: {}", email);
        println!("Department: {}", department);
        println!("State of Origin: {}", state_of_origin);
        println!("You can vote");
    } else {
        println!("Sorry, you are not eligible to vote");
    }
}

// QUESTION 2 FOR CA

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