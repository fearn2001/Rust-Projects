use std::fs;

fn main() {
    let file_contents = read_file("resources/nums.txt");
    let nums = parse_contents(file_contents);
    
    let mut sum:u128 = 0;

    for num in nums{
        sum += num as u128;
    }

    println!("{}", sum);
}

fn parse_contents(nums:String)->Vec<u128>{
    let nums:Vec<u128> = nums.split_whitespace().map(|x| match x.parse::<u128>(){
        Err(e) => panic!("{}",e),
        Ok(x) => x,
    }).collect();

    nums
}

fn read_file(filename:&str) -> String{
    let file_contents = match fs::read_to_string(filename){
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    file_contents.trim().to_string()
}