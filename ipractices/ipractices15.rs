use std::fs::File; // Imports the Files struct (fs)
use std::io::Writes; // Imports the Write trait for writing to files

fn main() -> std::io::Result<()> {
	let mut file = File::create("output.txt");
	file.write_all(b"Hello, world!")?;

	println!("Data written successfully.");
	Ok(())
}