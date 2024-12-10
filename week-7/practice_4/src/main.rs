fn main() {

    //NAME VECTOR
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    // Age vector
    let age = vec![16,17,19,22,20,21,18,23];

    println!("\nAge allocation:\n");

    //loop to iterate elements in vector
    for i in 0..age.len()
    {
        // iterating through on the vector
        print!("{} is {} yearls old\n",name[i], age[i]);

    }
}
