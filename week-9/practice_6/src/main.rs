fn add_one(e: &mut i32) {
    loop {
        *e+= 1;
        println!("Current value: {}", e);
    }
    
}

fn main() {

    let mut i =3;
    add_one(&mut i);
}