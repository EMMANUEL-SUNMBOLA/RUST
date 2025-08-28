// use std::{cmp::Ordering, io};
use std::io;

// use rand::Rng;

// mod functions;
mod loops;
fn main() {

    println!("what word would you like to print infinitely _ _ _");

    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("something went wrong");

    let x = x.trim();

    println!("How many times would you love to loop the word _ _ _");

    let mut c = String::new();

    io::stdin().read_line(&mut c).expect("something went wrong");

    let c :i32= c.trim().parse().expect("something went wrong");


    loops::print_inf(x, c);

    // functions::less_five(300);

    // let x :i32= if 7 > 5 {7} else {5};
    // functions::about(29, 7, "black");
    // functions::square(50);
    // functions::find_month();
    // functions::num_game();

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
