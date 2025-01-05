use std::fs::File; // Import the file struct.

// fn main() -> std::io::Result<()>
/* fn main() -> std::io::Result<()>
fn main() -> std::io::Result<()>
fn main() -> std::io::Result<()) */

fn main() -> std::io::Result<()> {
	let _file = File::create("new_file.txt")?;
	println!("File created successfully.");

	Ok(()) // Indicate success
}