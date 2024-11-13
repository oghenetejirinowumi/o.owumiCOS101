use std::io;


fn main() {
    let mut input1 = String::new();
    println!("Enter your Age:");
    io::stdin().read_line(&mut input1).expect("NOT A VALID ENTRY");
    let a: f64 = input1.trim().parse().expect("NOT A VALID NUMBER FOR 'a'");

    // Determinations of the INCENTIVES
    if a > 40.0 {
        let incentive1 = 1_560_000.00;
        println!("Your annual incentive is: {}", incentive1);
    }
    else if a > 30.0 && a < 40.0 {
        let incentive2 = 1_480_000.00;
        println!("Your annual incentive is: {}", incentive2);
    }
    else if a < 28.0 {
        let incentive3 = 100_000.00;
        println!("Your annual incentive is: {}", incentive3);
    }

}
    

