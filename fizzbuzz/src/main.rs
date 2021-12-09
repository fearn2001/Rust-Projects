fn main() {
    for i in 1..101{
        let mut output = "".to_string();

        if i % 3 == 0{
            output.push_str("Fizz")
        }

        if i % 5 == 0{
            output.push_str("Buzz")
        }

        if output == ""{
            println!("{}",i)
        }else {
            println!("{}",output);
        }
    }
}
