use std::{cmp::Ordering, io};
use rand::Rng;

fn main(){

    // num_game();
    // find_month()
}

pub fn less_five(x :i32){

    if x > 5{
        println!("your number is greater than 5");
    }else{
        println!("your number is either less than or equal to 5");
    }


}

pub fn find_month(){

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

    print!("it's all over noob");

}

pub fn square(x: i32){

    println!("the square of {} is {}", x, x*x);

}

pub fn find_monthx(x :i32) -> &str{

    let year = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];


    // println!("your month is {}", year[(x - 1) as usize]);

    year[(x-1) as usize]

}

pub fn about(d: i32, m: i32, c: &str){

    let mth = find_monthx(m);

    println!("you were born on the {} of {} and your fav color is {}", d, mth, c);

}