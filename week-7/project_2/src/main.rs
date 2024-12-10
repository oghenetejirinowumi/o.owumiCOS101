// RUST PROGRAM TO DETERMINE THE PERSON WITH THE HIGHEST YEAR OF EXPERIENCE
// DURING A JOB INTERVIEW
fn main() {

    // Vector of tuples: candidates and their years of experience
    let candidates = vec![
        ("OWUMI OGHENETEJIRIN", 15),
        ("TOBANI OKAFOR", 10),
        ("CHIAMAKA ARIELLE", 12),
        ("ABDUL MODIBO", 7),
        ("TOBI WONOBI", 9),
        ("BETTY BOO", 3),
    ];

    // TO FIND THE CANDIDATE WITH THE HIGHEST EXPERIENCE
    // VARIABLES TO STORE THE MOST EXPERIENCED CANDIDATE
    let mut most_experienced_candidate = "";
    let mut highest_experience = 0;

    // ITERATE THROUGH THE CANDIDATES
    for &(name, experience) in &candidates {
        if experience > highest_experience {
            most_experienced_candidate = name;
            highest_experience = experience;
        }
    }
    // print the result

    println!("The candidate with the highest experience is {} with {} years.", most_experienced_candidate, highest_experience);

}

    