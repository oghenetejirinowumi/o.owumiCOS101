fn main() {
    // Array with data type (explicit integer datatype eid)
    let arr1:[i32;4] = [10,20,30,40];
    println!("\nArray with data type");
    println!("array is {:?}", arr1);
    println!("array size is: {}",arr1.len());

    // Array without data type (IFD - implicit float datatype)
    let arr2 = [10.4,20.7,30.4,40.9,52.3,72.2];
    println!("\nArray without data type");
    println!("array is {:?}", arr2);
    println!("array sie is:{}",arr2.len());

    // Array with default values that creates and
    // initializes all its elements with a default of -1.
    let arr3:[i32;8] = [-1;8];
    println!("\nArray with default values");
    println!("array is {:?}", arr3);
    println!("array sie is: {}",arr3.len());
}