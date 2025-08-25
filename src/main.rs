use std::{cmp::Ordering, io};

use rand::Rng;

mod functions;
fn main() {


    functions::square(50);
    functions::find_month();
    functions::num_game();

    // let secret = rand::thread_rng().gen_range(1..=100);
    
    // println!("The secret number is : {secret}");
    
    // loop{
    //     let mut guess = String::new();
        
    //     println!("Input your guess");

    //     io::stdin()
    //     .read_line(&mut guess)
    //     .expect("failed to read line");
    
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    
    //     println!("your guess is {guess}, so you guessed ");
    
    //     match guess.cmp(&secret){
    //         Ordering::Less => println!("too small"),
    //         Ordering::Greater => println!("too big"),
    //         Ordering::Equal => {
    //             println!("correctly 😀");
    //             break; 
    //         },
    //     }
        
    // }
}
