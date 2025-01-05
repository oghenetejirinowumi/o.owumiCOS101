use std::fs; // Imports the filesystem (fs) module

fn main() -> std::io::Result<()> {
	fs::remove_file("old_file.txt")?;
	println!("File deleted successfully.");

	Ok(()) // Indicate success.
}