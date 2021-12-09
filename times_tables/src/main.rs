extern crate text_io;
fn main() {

    println!("Please enter a number between 1 and 15");

    let num:u8 = text_io::read!();

    print!("\x1B[2J\x1B[1;1H"); //clear the screen
    
    if !(num > 0 && num < 16){
        main();
    } else {
        for i in 1..num+1{
            for j in 1..num+1{
                print!("{: ^4}",i*j);
            }
            println!();
        }
        
    }

}
