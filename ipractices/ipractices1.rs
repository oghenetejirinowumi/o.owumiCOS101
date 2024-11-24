fn main() {
	let empty = String::new();
	println!("Empty string: {}", empty);

	// to_string() converts a value into a string
	let number = 42;
	let stringified = number.to_string();
	println!("Stringified number: {}", stringified);
	// OUTPUT Stringified number: 42

	// replace() it replaces all occurencies of a pattern with another string
	let text = String::from("Hello, world!");
	let replaced = text.replace("world", "Rust");
	println!("{}", replaced);
	// OUTPUT: Hello, Rust!

	// as_str() converts a string into a string slince (.str)
	let string = String::from("Hello, Rust!");
	let slince = string.as_str();
	println!("Slice: {}", slice);
	// OUTPUT: Hello, Rust!

	// push() appends a single character at the end of a string
	let mut string = String::from("Rust");
	string.push("!");
	println!("{}", string);
	// OUTPUT: Rust!

	// push_str() appends another string to the end of an existing string
	let mut string = String::from("Hello");
	string.push_str(", world!");
	println!("{}", string);
	// OUTPUT: Hello, world!

	// len() Reutrns the number of BYTES of a string
	let text = String::from("Hello");
	println!("Length: {}", text.len());
	// OUTPUT: 5

	//trim() purpose: REMOVES LEADING and TRAILING whitespaces from a string
	let text = String::from("    Hello,Rust!    ");
	let trimmed = text.trim();
	println!("Trimmed: {}", trimmed);
	// OUTPUT: 'Hello, Rust!'

	/* split_whitespace() splits a string by spaces and 
	returns an ITERATOR (used to loop through each work */

    let text = String::from("Hello world from Rust");
    for word in tect.split_whitespace() {
	println!("{}", word);
}
    /* OUTPUT:
    Hello
    world
    from
    rust */

    // split() splits a string using a specific pattern (like a character)
    let text = String::from("apple-orange-banana");
    for word in text.split('_') {
    	println!("{}", word);
    }
    /* OUTPUT:
    apple
    orange
    banana */

    // chars() RETURNS and ITERATOR over each character in the string.
    let text = String::from("Rust");
    for char in text.chars() {
    	println!("{}", char);
    }
    /* OUTPUT:
    R
    u
    s
    t*/
    
    
   



  


}