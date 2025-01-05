use std::fs::OpenOptions; // Import OpenOptions for advanced file options.
use std::io::Write; // Imports Write trait for writing to files.

fn main() -> std::io::Result<()> {
	let mut file = OpenOptions::new().append(true).open("log.txt")?;
	writeln!(file, "New log entry.")?;
	println!("Text appended successfully");
	Ok(())

}