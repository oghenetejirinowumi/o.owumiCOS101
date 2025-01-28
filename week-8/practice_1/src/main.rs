use std::io::Write;

fn main() {
    let _announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.text").expect("create failed");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("Write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file.");
}