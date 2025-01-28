<<<<<<< HEAD
use std::io::fs::OpenOptions; // Imports the OpenOptions for advanced file options
use std::io::Write; //  Imports the Write trait for writing to files

fn main() {

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("write failed");
    println!("file append success")
}
=======
fn main() {
    println!("Hello, world!");
}
>>>>>>> db63d423d2df8ab5e0d9c2dbe1db2ffd734df808
