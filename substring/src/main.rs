use std::io;

extern crate text_io;

fn get_user_input() -> String{
    let mut input= String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Please enter the string to be searched");
    let string_a = get_user_input();
    println!("Please enter the substring");
    let string_b = get_user_input();

    match string_a.contains(&string_b) {
        true => println!("The substring was found in the first string"),
        false => println!("The substring was not found in the first string")
    }
}