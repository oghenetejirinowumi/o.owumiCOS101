use std::fs::File; // Imports the File struct.
use std::io::{self, Read}; // Import the Read trait for reading files.

fn main() -> std::io::Result<()> {
	let mut file = File::open("example.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	println!("File Contents: {}", contents);
	Ok(())

}