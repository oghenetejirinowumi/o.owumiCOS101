use std::fs::File; // Imports the file struct to work with files
use std::io::{self, Read}; // Imports the read trait for reading content.

fn main() -> io::Result<()> {
	// Attempts to open an existing file called "example.txt"
	let mut file = File::open("example.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("ERROR")?;
	println!("File Contents: {}", contents);

	Ok(()) // Indicate the function succeeded

}