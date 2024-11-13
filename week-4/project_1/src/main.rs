use std::io;


fn main() {
    let mut input1 = String::new();
    println!("Enter a value for a:");
    io::stdin().read_line(&mut input1).expect("INVALID INPUT"); 
    let a: f64 = input1.trim().parse().expect("Not a valid number for 'a'");
    input1.clear();

    let mut input2 = String::new();
    println!("Enter a value for b:");
    io::stdin().read_line(&mut input2).expect("INVALID INPUT"); 
    let b: f64 = input2.trim().parse().expect("Not a valid number for 'a'");
    input2.clear();

    let mut input3 = String::new();
    println!("Enter a value for c:");
    io::stdin().read_line(&mut input3).expect("INVALID INPUT"); 
    let c: f64 = input3.trim().parse().expect("Not a valid number for 'a'");
    input3.clear();

    // calculation of the discriminant b^2 - 4ac
    let discriminant = b * b - 4.0 * a * c;

    // to determine the nature of the roots using if, else, if.. else and else.. if satements
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("THE TWO DISTINCT REAL ROOTS: root1 = {}, root2 = {}", root1, root2);
    } else if discriminant == 0.0 {
        // There is only 1 real root
        let root = -b / (2.0 * a);
        println!("There is one real root: root = {}", root);
    } else {
        // No real roots
        println!("There are no real roots.");
    }
}
