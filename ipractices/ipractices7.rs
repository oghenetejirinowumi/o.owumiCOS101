// ipractice7 for week 7 COS101 PROJECT 1
use std::io;

fn main() {
	    // VECTORS FOR JOB AND POSITIONS
    let roles = ["Office Administrator", "Academic", "Lawyer", "Teacher"];
    let aps_levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL 8-10", "EL2 10-13", "SES"];

    // JOB POSITIONS AND THEIR LEVELS IN THE TABLES
    let office_admin_positions = vec![
        ("Intern", aps_levels[0]),
        ("Administrator", aps_levels[1]),
        ("Senior Administrator", aps_levels[2]),
        ("Office Manager", aps_levels[3]),
        ("Director", aps_levels[4]),
        ("CEO", aps_levels[5]),
    ];

    let academic_positions = vec![
        ("-", aps_levels[0]),
        ("Research Assistant", aps_levels[1]),
        ("PhD Candidate", aps_levels[2]),
        ("Post-Doc Researcher", aps_levels[3]),
        ("Senior Lecturer", aps_levels[4]),
        ("Dean", aps_levels[5]),
    ];

    let lawyer_positions = vec![
        ("Paralegal", aps_levels[0]),
        ("Junior Associate", aps_levels[1]),
        ("Associate", aps_levels[2]),
        ("Senior Associate 1-2", aps_levels[3]),
        ("Senior Associate 3-4", aps_levels[4]),
        ("Partner", aps_levels[5]),
    ];

    let teacher_positions = vec![
        ("Placement", aps_levels[0]),
        ("Classroom Teacher", aps_levels[1]),
        ("Senior Teacher", aps_levels[2]),
        ("Leading Teacher", aps_levels[3]),
        ("Deputy Principal", aps_levels[4]),
        ("Principal", aps_levels[5]),
    ];

    // VECTOR OF TUPLES TO MAP ROLES TO THEIR POSITIONS
    // (roles, positions)
    let role_map = vec![
    ("Office Administrator", office_admin_positions),
    ("Academic", academic_positions),
    ("Lawyer", lawyer_positions),
    ("Teacher", teacher_positions),

    ];

    loop {
    	let mut profession = String::new();
    	println!("Enter a profession (Office Administrator, Academic, Lawyer, Teacher):");
    	io:stdin().read_line(&mut profession).expect("FAILED TO READ INPUT");
    	let profession = profession.trim();

    	let mut found = false;
    	for (roles, positions) in &role_map {
    		if *roles == profession {
    			found = true;
    			println!("\nPositions for {}:", profession);
    			for (position, aps_level) in positions {
    				println!("{} - {}", positions, aps_level);
    			}
    			break; // EXITS THE LOOP ONCE WE FIND THE MATCH
    		}
    	}

    	if !found {
    		println!("\nProfession not found. Please enter a valid role.");
    	} else {
    		break; // EXITS THE MAIN LOOP IF A VALID PROFESSION IS FOUND
    	}
    }
}