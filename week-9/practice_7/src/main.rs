struct Employee {
    name:String,
    company:String,
    age:u32
}

fn main() {
    let emp1 = Employee {
        company:String::from("Enrst & Young"),
        name:String::from("Ebibiong Jessica"),
        age:25
    };
    println!("Nmae = {} \n", emp1.name);
    println!("Company = {} \n", emp1.company);
    println!("Age = {}", emp1.age);
}