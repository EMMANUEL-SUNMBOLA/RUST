use std::{cmp::Ordering, io};
use rand::Rng;

fn main(){

    num_game();
    find_month()
}

fn find_month(){

    let year = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];

    println!("input your month in number");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("some shit happened");

    let month :usize = input.trim().parse().expect("not an integer");

    println!("your month is {}", year[month - 1]);

}

pub fn num_game(){

    let secret = rand::thread_rng().gen_range(1..=100);

    println!("secret number is {}", secret);

    loop{
        
        let mut guess = String::new();
    
        println!("Enter your guess noob . . .");
        
        io::stdin().read_line(&mut guess).expect("trouble getting your guess");
    
        let guess:u32 = guess.trim().parse().expect("not a signed number");

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