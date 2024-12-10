fn main() {

    // USING Vec::new(); METHOD
    let v:Vec<i64> = Vec::new();

    // printing the size of vector
    println!("\n the length of the Vec::new(); is: {}", v.len());

    // USING MACRO
    let v = vec!["Grace", "Effiong", "Brasil", "Kareem", "Susan"];

    // printing the size of the vector
    println!("\nThe length of the vec macro is: {}", v.len());
}