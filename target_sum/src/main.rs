use std::io;

fn get_nums() -> Vec<u16>{
    let mut buf = String::new();
    
    match io::stdin().read_line(&mut buf){
        Err(e) => panic!("{}", e),
        Ok(x) => x,
    };
    buf.split_whitespace().map(|x| match x.parse::<u16>(){
        Ok(x) => x,
        Err(e) => panic!("{}",e),
    }).collect()
}

fn get_target() -> u16 {
    let mut buf = String::new();
    println!("Please enter the target number");
    match io::stdin().read_line(&mut buf){
        Ok(_) => {},
        Err(e) => panic!("{}",e),
    };

    match buf.trim().parse::<u16>(){
        Ok(x) => x,
        Err(e) => panic!("{}",e)
    }
}

fn main() {
    println!("Please enter the list of numbers");
    let mut nums = get_nums();
    nums.sort(); //sort numbers so duplicates are adjacent
    nums.dedup(); //remove adjacent duplicate numbers
    
    let target = get_target();    
    let mut result:Vec<Vec<u16>> = vec![vec![]]; //create a vector of vectors. contains one empty vector which prints when printed is just a newline
    
    for i in 0..nums.len(){
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                result.push(vec![nums[i],nums[j]]);
            }
        }
    }

    print_result(result);
}

fn print_result(result:Vec<Vec<u16>>){
    for i in result{
        for j in i{
            print!("{} ",j)
        }
        println!();
    }
}