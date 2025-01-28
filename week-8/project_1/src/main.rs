use std::fs::File; // Import the File struct to handle file operations
use std::io::{self, Write}; // Import I/O traits for writing to files and handling errors

fn main() -> io::Result<()> {
    // DEFINING THE CATEGORIES OF DRINKS AS VECTORS OF STRINGS
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let nonalcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // Create and open a file for writing (this will overwrite if the file already exists)
    let mut file = File::create("nigerian_brewery_categories.txt")
        .expect("Failed to create or open the file for writing!");

    // WRITE THE DATA TO THE FILE
    writeln!(file, "NIGERIAN BREWERY LIMITED HIGH QUALITY CATEGORIES:\n")
        .expect("Failed to write the header to the file!");
    
    writeln!(file, "Lager:").expect("Failed to write Lager section to the file!");
    for beer in &lager {
        writeln!(file, " - {}", beer).expect("Failed to write a lager item to the file!");
    }

    writeln!(file, "\nStout:").expect("Failed to write Stout section to the file!");
    for beer in &stout {
        writeln!(file, " - {}", beer).expect("Failed to write a stout item to the file!");
    }

    writeln!(file, "\nNon-Alcoholic:").expect("Failed to write Non-Alcoholic section to the file!");
    for drink in &nonalcoholic {
        writeln!(file, " - {}", drink).expect("Failed to write a non-alcoholic item to the file!");
    }

    // Inform the user that the data has been successfully saved
    println!("Categories saved to 'nigerian_brewery_categories.txt'.");

    Ok(()) // Return Ok to indicate successful execution
}