// declare a structure 
struct Employee {
    ceo:String,
    company:String,
    age:u32
}

fn main() {
    // INITIALIZE THE STRUCTURE
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee {
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };

    // pass emp1 and emp2 to display
    display(emp1);
    display(emp2);
}
// fetch values to specific structure fields using the
// operator and print it to the consol

fn display(emp: Employee) {
    println!("Name is:{} Company is: {} Age is: {}", emp.ceo,emp.company,emp.age)
}