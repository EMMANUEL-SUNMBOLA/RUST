// use std::{cmp::Ordering, io};
// use std::io::{self, Read};
use std::io;
// use bs58;

// use rand::Rng;

// mod functions;
// mod loops;
mod functions;
fn main() {

    struct User {
        active: bool,
        username: String,
        email: String,
        pwd: String,
        sign_in_count: u64,
    }

    let caveman = User {
        active: true,
        username: String::from("caveman"),
        email: String::from("adedayoforgit@gmail.com"),
        pwd: String::from("1234"),
        sign_in_count: 4,
    };
    
    println!("\n my name is {}, I am always active -> {}, hit me up on {} let's work together", caveman.username, caveman.active, caveman.email);


    // script to fetch username and parse to String
    println!("input your username  ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("something went wrong");
    let username: String = username.trim().parse().expect("input a string");

    // script to fetch email and parse to String
    println!("input your email  ");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("something went wrong");
    let email: String = email.trim().parse().expect("input a string");

    // script to fetch pwd and parse to String
    println!("input your pwd  ");
    let mut pwd = String::new();
    io::stdin().read_line(&mut pwd).expect("something went wrong");
    let pwd: String = pwd.trim().parse().expect("input a string");

    // script to fetch age and parse to unsigned integer 64 bits
    println!("input your age  ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("something went wrong");
    let age: u64 = age.trim().parse().expect("input a u64 integer");

    let caveman = functions::build_user1(username, email, pwd, age);

    println!("\n \n hello {}, you've been registered as a {} year old, and we'll contact you at {}", caveman.username, caveman.age, caveman.email);

    // let s = String::from("hello");
    // let s = "hello";

    // let s1 = s.as_bytes();

    // print!("{s1}");

    // let gm = String::from("good morning");
    // let gw = "good morning";

    // let s1 = &gm[..4];
    // // [0..4] = [..4];
    // let s2 = &gw[0..4];


    // println!("{s1} \n\n {s2}");


    // let wallet = "GWn3ousvJY9mos6uW5a8VWDyVMoaH2CNX6XovwjpW9DE";

    // let decoded = bs58::decode(wallet).into_vec().unwrap();
    // println!("wallet in 32 bytes is \n {:?} and Length in bytes: {}",decoded, decoded.len());

    // let wallet = String::from("GWn3ousvJY9mos6uW5a8VWDyVMoaH2CNX6XovwjpW9DE");



    // for (i, ch) in wallet.chars().enumerate(){



    //     println!("{} - {}", i+1, ch);

    // }

        // let mut h = String::from("Opueh");

        // h = String::from("knew");

        // println!("he {} her", h);

    // let mut s1 = String::from("hello");

    // s1.push_str(", caveman");

    // let s2 = s1;

    // println!("{s1}");
   
   
    // println!("input your number in celcius");

    // let mut x = String::new();

    // io::stdin().read_line(&mut x).expect("something went wrong horribly");

    // let x :f64= x.trim().parse().expect("input the right type of data");

    // let y = functions::to_fahre(x);

    // println!("{x} deg celcius is {y} deg Fahrenheit");


    // // to celcius

    // println!("insert your temp in fahrenheit . . .");

    // let mut p: String = String::new();

    // io::stdin().read_line(&mut p).expect("something went wrong walai at input");

    // let p: f64 = p.trim().parse().expect("something went wrong walai at parse");

    // let q = functions::to_celci(p);

    // println!("your temp {p} deg F in C is {q}");


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
