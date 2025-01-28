// UPDATING A VECTOR
fn main() {
	// CREATE AND EMPTY VECTOR "CITY"
	let mut city: Vec<String> = Vec::new();
	// PRINT CITY VECTOR
	println!("The City vector has an element {}",city.len());
	// PUSH NEW ELEMENTS INTO
	let mut input1 = Sting::new();
	println!("How many cities do you want to enter?");
	std::io::stdin().read_line(&mut input1).expect("Failed to read input");
	let city_num:i32 = input1.trim().parse().expect("INVALID INPUT");
	for count in 0..city_num {
		let mut input 2 = String::new();
		println!("Enter City {}", count+1);
		std::io::stdin.read_line(&mut input2).expect("Failed to read input");
		let new_city:String = input2.trim().parse().expect("INVAlid INPUT");
		city.push(new_city);	
	}
	print!("Your preferred cities are:\n");
	let mut count=1;
	// loop to iterate elements in vector
	for i in city_num
	{
		// iterating through i on the vector
		println!("{} {}", count, i);
		count+=1;
	}
}