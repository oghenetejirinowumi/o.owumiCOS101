// WEEK 7 PROJECT 2 ALTERNATIVE WAYS

// ALTERNATIVE WAY 1: USING ITERATORS AND CLOSURES

fn main() {
	// Vector of tuples: candidates and their years of experience
	let candidate = vec![

	("OWUMI OGHENETEJIRIN", 15),
	("TOBANI OKAFOR", 10),
	("CHIAMAKA ARIELLE", 12),
	("ABDUL MODIBO", 7),
	("TOBI WONOBI", 9),
	("BETTY BOO", 3),

	];

	println!("--- JOB INTERVIEW CANDIDATES ---");
	for (index, &(name, experience)) in candidate.iter().enumetrate() {
		println!("{}. {} - {} years of experience", index + 1, name, experience);
	}

	// Using `max_by_key` to find the candidate with the highest experience
	let most_experienced = candidates.iter().max_by_key()


}