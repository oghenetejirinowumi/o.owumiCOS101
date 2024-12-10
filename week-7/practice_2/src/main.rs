fn main() {

    let v = vec!['C','O','M','P','U','T','E','R'];

    let mut input1 = String::new();

    println!("Enter an index value btw (0 - 7)");
    std::io::stdin().read_line(&mut input1).expect("FAILED TO READ INPUT");
    // index is the NON NEGATIVE value which is smaller than the sie of the vector
    let index:usize = input1.trim().parse().expect("INVALID INPUT");

    //GETTING VALUE AT GIVEN INDEX VALUE
    let ch: char = v[index];
    print!("{} is the character for the index [{}]\n",ch,index);
}