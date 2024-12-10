fn main() {
	// Create an empty vector "City"
	let mut city: Vec<String> = Vec::new();
	//Print City Vector
	let mut input1 = String::new();
	println!("How many cities do you want to enter");
	std::io::stdin().read_line(&mut input1).expect("FAILED TO READ INPUT");
	let city_num:i32 = input1.trim().parse().expect("INVALID INPUT");
	for count 0..city_num {
		let mut input2 = String::new;
		println!("Enter city {}", count+1);
		std::io::stdin().read_line(&mut input2).expect("FAILED TO READ INPUT");
		let new_city:String = input2.trim().parse().expect("INVALID INPUT");
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