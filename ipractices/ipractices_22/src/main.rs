fn main() {

    let data = ["Ade","Ola","Joyce","Adam","Yemi","Sam","Tola"];
    pass_me(&data[4..]);

}

fn pass_me(use_data:&[&str]) {

    println!("The length of the use_data is: {:?}", use_data.len());
    println!("{:?}", use_data);
}