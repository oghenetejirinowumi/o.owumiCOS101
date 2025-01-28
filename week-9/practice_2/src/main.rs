fn main() {

    let v = vec![10,20,30];
    // vector v owns the object in H E A P

    let v2 = v.clone();
    // moves ownership to v2 

    display(v2.clone());
    println!("In main {:?}", v2);
}

fn display(v: Vec<i32>){
    println!("Indide display {:?}", v);
}
