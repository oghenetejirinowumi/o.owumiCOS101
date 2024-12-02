use std::io;

fn main() {
    let mut name = String::new();
    println!("WELCOME, Please enter your name:");
    io::stdin().read_line(&mut name).expect("FAILED TO READ INPUT");
    let name = name.trim();

    println!("WELCOME {} to MATH 101 FORMULA PROMPT,", name);

    // DECLARING THE VARIABLES FOR FORMULA OPTIONS
    let trampezium_formula = "trampezium_formula";
    let rhombus_formula = "rhombus_formula";
    let parallelogram_formula = "parallelogram_formula";
    let cube_formula = "cube_formula";
    let cylinder_formula = "cylinder_formula";

    let mut input1 = String::new();
    println!(
        "PLEASE SELECT A FORMULA FROM THE OPTIONS: {}, {}, {}, {}, {}",
        trampezium_formula, rhombus_formula, parallelogram_formula, cube_formula, cylinder_formula
    );
    io::stdin().read_line(&mut input1).expect("FAILED TO READ INPUT");
    let input1 = input1.trim();

    if input1 == trampezium_formula {
        println!("TRAMPEZIUM FORMULA = height/2 * (base1 + base2)");
        let mut height = String::new();
        println!("Please enter the height of your trampezium:");
        io::stdin().read_line(&mut height).expect("FAILED TO READ INPUT");
        let height: f64 = height.trim().parse().expect("NOT A VALID ENTRY");

        let mut base1 = String::new();
        println!("Please enter the base 1 of your trampezium:");
        io::stdin().read_line(&mut base1).expect("FAILED TO READ INPUT");
        let base1: f64 = base1.trim().parse().expect("NOT A VALID ENTRY");

        let mut base2 = String::new();
        println!("Please enter the base 2 of your trampezium:");
        io::stdin().read_line(&mut base2).expect("FAILED TO READ INPUT");
        let base2: f64 = base2.trim().parse().expect("NOT A VALID ENTRY");

        let tramp_area = (height / 2.0) * (base1 + base2);
        println!("The area of your trampezium is: {}", tramp_area);
    } else if input1 == rhombus_formula {
        println!("RHOMBUS FORMULA = (diagonal1 * diagonal2) / 2.0");
        let mut diagonal1 = String::new();
        println!("Please enter the diagonal 1 of your rhombus:");
        io::stdin().read_line(&mut diagonal1).expect("FAILED TO READ INPUT");
        let diagonal1: f64 = diagonal1.trim().parse().expect("NOT A VALID ENTRY");

        let mut diagonal2 = String::new();
        println!("Please enter the diagonal 2 of your rhombus:");
        io::stdin().read_line(&mut diagonal2).expect("FAILED TO READ INPUT");
        let diagonal2: f64 = diagonal2.trim().parse().expect("NOT A VALID ENTRY");

        let rhomb_area = (diagonal1 * diagonal2) / 2.0;
        println!("The area of your rhombus is: {}", rhomb_area);
    } else if input1 == parallelogram_formula {
        println!("PARALLELOGRAM FORMULA = base * altitude");
        let mut basep = String::new();
        println!("Please enter the base of your parallelogram:");
        io::stdin().read_line(&mut basep).expect("FAILED TO READ INPUT");
        let basep: f64 = basep.trim().parse().expect("NOT A VALID ENTRY");

        let mut altitude = String::new();
        println!("Please enter the altitude of your parallelogram:");
        io::stdin().read_line(&mut altitude).expect("FAILED TO READ INPUT");
        let altitude: f64 = altitude.trim().parse().expect("NOT A VALID ENTRY");

        let paral_area = basep * altitude;
        println!("The area of your parallelogram is: {}", paral_area);
    } else if input1 == cube_formula {
        println!("CUBE FORMULA = 6.0 * (length of the side * length of the side)");
        let mut clength = String::new();
        println!("Please enter the length of one side of the cube:");
        io::stdin().read_line(&mut clength).expect("FAILED TO READ INPUT");
        let clength: f64 = clength.trim().parse().expect("NOT A VALID ENTRY");

        let cube_area = 6.0 * (clength * clength);
        println!("The surface area of the cube is: {}", cube_area);
    } else if input1 == cylinder_formula {
        println!("CYLINDER FORMULA = PI * (radius * radius) * height");
        let mut radiusc = String::new();
        println!("Please enter the radius of your cylinder:");
        io::stdin().read_line(&mut radiusc).expect("FAILED TO READ INPUT");
        let radiusc: f64 = radiusc.trim().parse().expect("NOT A VALID ENTRY");

        let mut heightc = String::new();
        println!("Please enter the height of your cylinder:");
        io::stdin().read_line(&mut heightc).expect("FAILED TO READ INPUT");
        let heightc: f64 = heightc.trim().parse().expect("NOT A VALID ENTRY");

        let cylinder_volume = std::f64::consts::PI * (radiusc * radiusc) * heightc;
        println!("The volume of the cylinder is: {}", cylinder_volume);
    } else {
        println!("Invalid formula selection.");
    }

    println!("THANK YOU FOR USING THE MATH 101 FORMULA PROMPT\nHAVE A NICE DAY");
}


