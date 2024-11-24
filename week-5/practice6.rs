fn main() {
	let n1 = "Electrical".to_string();
	let n2 = " Electronic".to_string();
	let n3 = " Engineering".to_string();
	let n4 = n1 + &n2 + &n3; // n2 & n3 reference is passed

	// About Electrical/Electronic
	println!("\nThe {} is informed by the application to 
    train electrical/electronic egineering professionals
	in the area of design, building and maintenance of
	electrical control systems,", n4);

	let w1 = "Computer".to_string();
	let w2 = " Science".to_string();
	let w3 = w1 + &w2; // w2 reference is passed
	println!();
	println!("{} is aimed at developing component, creative,
		innovatice, entrepreneural and ethically-minded persons,
		capable of creating VALUE in the diverse fields of 
		Computer Science",w3);


}