fn main() {
    // AN ARRAY OF NUMBERS
    let numbers = [1, 2, 3, 4, 5];
    println!("Original array = {:?}", numbers);

    // CREATES A SLICE OF 2ND AND 3RD ELEMENT
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced ={:?}", slice1);

    // OMIT THE START INDEX
    let slice2 = &numbers[..3];
    // this means the slice starts from index 0 and goes up to index 3 (exclusive)
    println!("index 0 to 3 sliced = {:?}", slice2);

    // OMIT THE END INDEX
    let slice3 = &numbers[2..];
    // This means the slice stars from index 2 and goes up to index 5 (exclusive)
    println!("index 2 to 5 sliced = {:?}", slice3);

    // omit the start index and the end index
    //reference the whole array
    let slice4 = & numbers[..];
    // This means the slice starts from index 0 and goes up to index 5 (exclusive);
    println!("index 0 to index 5 sliced = {:?}", slice4);
}