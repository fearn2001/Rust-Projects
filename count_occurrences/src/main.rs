extern crate text_io;
use std::{collections::HashMap, vec};

fn count_occurences(nums:Vec<u16>) -> HashMap<u16,u16>{
    let mut occurences:HashMap<u16,u16> = HashMap::new();
    
    for num in nums{
        if occurences.contains_key(&num){
            occurences.insert(num, occurences[&num]+1);
        } else {            
            occurences.insert(num, 1);
        }
    }

    occurences
}

fn get_nums() -> Vec<u16>{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    let mut input:Vec<_> = input.trim().split_whitespace().collect();
    
    input.sort();
    
    let mut nums:Vec<u16> = vec![]; 
    
    for num in input{
        nums.push(num.parse().unwrap());
    }

    nums
}


fn main() {
    let  nums = get_nums();
    
    let mut x = HashMap::<u16,u16>::new();

    x.insert(1, 1);

    let occurences = count_occurences(nums);
    
    let mut indexes = Vec::from_iter(occurences.keys());
    indexes.sort();
    for i in indexes{
        println!("{}:{}", &i, occurences[i]);
    }
}