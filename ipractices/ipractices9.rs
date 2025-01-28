// PRACTICE 
let mut found = false;
for (role, positions) in &role_map {
	if *role == profession {
		found = true;
		println!("\nPositions for {}:", profession);
		for (position, aps_level) in position {
			println!("{} - {}", position, aps_level);
		}
		break; //EXISTS THE LOOP ONCE WE FIND THE MATCH 
	}
}

if !found {
	println!("\nProfession not found. Please enter a valid role.");
} else {
	break; // EXITS THE MAIN LOOP IF A VALID PROFESSION IS FOUND
}