use std::{cmp::Ordering, io};

use rand::Rng;
fn main() {

    let secret = rand::thread_rng().gen_range(1..=100);
    
    println!("The secret number is : {secret}");
    // println!("Input your guess");
    let mut guess = String::new();

    loop{

        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    
        let mut guess: u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("your guess is {guess}, so you guessed ");
    
        match guess.cmp(&secret){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("correctly 😀");
                break; 
            },
        }
        
    }
}
