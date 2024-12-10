fn main() {

    // nutable array 
    let mut colors = ["red", "green", "yellow", "white"];

    println!("\nOriginal array = {:?}", colors);

    // mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("Fist slice = {:?}", sliced_colors);

    // change the value of the original slice t the first index
    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}", sliced_colors)
}