fn main() {
    let mut wood:i32 = 35;
    bush(&mut wood); // Pass a mutable reference of 'wood' to 'bush'
    wood = wood * 2; // Modify the 'wood' after function call
    println!("The value of wood is: {}", wood) // Print the modified value of 'wood'
}

fn bush(plank: &mut i32) {
    *plank = *plank / 5; // Deference the mutable reference and divide 'plank; by 5
    let wood = *plank / 3; // Create a new varieble 'wood' with a local calculation
    println!("The value of the plank is:{}", plank); 
}