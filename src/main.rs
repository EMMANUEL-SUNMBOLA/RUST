// use std::{cmp::Ordering, io};
use std::io::{self, Read};

// use rand::Rng;

// mod functions;
// mod loops;
mod functions;
fn main() {


    println!("input your number in celcius");

    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("something went wrong horribly");

    let x :i64= x.trim().parse().expect("input the right type of data");

    functions::toFahre(x);

    // for num in (1..50).rev(){
    //     println!("{num}");
    // }

    // print!("Lift off");

    // let year = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];

    // // let mut index = 0;
    // print!("_");

    // for month in year{

    //     print!("_ {month} _");

    // }

    // print!("_");


    // while index < 12 {

    //     println!("The no {} month is {}", (index + 1), year[index]);
    //     index += 1;

    // }

    // println!("what word would you like to print infinitely _ _ _");

    // let mut x = String::new();

    // io::stdin().read_line(&mut x).expect("something went wrong");

    // let x = x.trim();

    // println!("How many times would you love to loop the word _ _ _");

    // let mut c = String::new();

    // io::stdin().read_line(&mut c).expect("something went wrong");

    // let c :i32= c.trim().parse().expect("something went wrong");


    // loops::print_inf(x, c);

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
