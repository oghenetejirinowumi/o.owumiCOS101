// The code below uses the remove_file() function to delete a file
// DELETE A FILE

use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}