use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // ARRAY OF STUDENT DATA (Student name, Matric Number, Department & Level)
    let students = [
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "EC010110101", "Economics", 200),
        ("Shania Bolade", "CSC10328828", "Computer Science", 400),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    // Convert ARRAY to vector of tuples for flexibility
    let student_details: Vec<(&str, &str, &str, i32)> = students.to_vec();

    // Display the student details
    println!("PAU SMIS:");
    println!("{:<20} {:<15} {:<20} {:<10}",
        "Student Name", "Matric Number", "Department", "Level");
    
    println!("{:-<20} {:-<15} {:-<20} {:-<10}", "-", "-", "-", "-");
    for student in &student_details {
        println!( "{:<20} {:<15} {:<20} {:<10}",
            student.0, student.1, student.2, student.3);
    }

    // CREATE A NEW FILE AND WRITE STUDENT DETAILS INTO THE FILE
    let mut file = File::create("student_details.txt").expect("Failed to create the file");

    // Write headers to the file
    writeln!(file, "{:<20} {:<15} {:<20} {:<10}",
        "Student Name", "Matric Number", "Department", "Level").expect("Failed to write header to the file");
   
    writeln!(file, "{:-<20} {:-<15} {:-<20} {:-<10}",
        "-", "-", "-", "-").expect("Failed to write header separator to the file");

    // Write student details to the file
    for student in students {
        writeln!(file, "{:<20} {:<15} {:<20} {:<10}",
            student.0, student.1, student.2, student.3).expect("Failed to write student details to the file");
    
    }

    println!("Student details written to 'student_details.txt' successfully!");
    Ok(())
}