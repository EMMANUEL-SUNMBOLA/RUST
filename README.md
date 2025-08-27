# RUST

### 

```RUST
        Let Age = "6";
        println!("You are {} years old", Age);
        println!("You are {Age} years old");
```

## Data Types in RUST

- Integers can be Signed [i8, i64, ...] and Unsigned [u8, u64, ...]
- Signed integers can be either negative or positive and they range from
``` Caveman
    i8 => n = 8

    -(2^(n-1)) -> 2^(n-1) - 1
```

- Unsigned integers can only be positive 
``` caveman

    u8 => n = 8

    0 -> 2^n - 1
```

- Floats are all Signed (either f32 or f64)

- Booleans are well .. . true or false

## Compound Data Types
- Tuple
 to access Tuples you use .
 ``` RUST
        Let Tup = (1, 2,"ewa", -5, 9.4);
        
        //OR

        Let Tup2 :(164, 132, str, u64, f32) = (1, 2,"ewa", -5, 9.4);

        println!("my name is {} ,  I am {} years old , my wallet balance is ${}", tup.2, tup.1, tup.4);
 ```
- Arrays
    to index Arrays we use []
    ``` RUST
        let a:[i32, 5] = [3: 5];
        //means
        a = [3, 3, 3, 3, 3];

        println!("the first element of the array a is {}", a[0]);


        // could also be 
        let year = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"]; 

        println!("I was born in {}", year[6]);

        // another example

            let arr = [1, 2, 3, 4];

        println!("first element in the array is {}, second element is {}, third element is {}, fourth element is {}", arr[0], arr[1], arr[2], arr[3]);

        // quick playject

            let mut month = String::new();
            println!("what month were you born in numbers ");

            io::stdin().read_line(&mut month).expect("something wen't wrong");

            let month: usize = month.trim().parse().expect("input wasn't a number");

            let element = year[month-1];


            println!("{element}");
    ```

## Functions

```RUST
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

            pub fn square(x: i32, c: char, d: &str){
                // use char when expecting only 1 letter or character || use single quotes for char ''
                // use str when you're expecting more than 1 letter or character || use double quotes "" for strings 

                println!("the square of {} is {}", x, x*x);
                println!("the character of {} is {}", x, c);
                println!("the string of {} is {}", x, d);


            }


```

## Expressions, Statements, Conditionals

- `Expressions` are codelines that return a value, they must not have semi colons

```RUST
        let x = {
            let y = 6;
            y + 1

            // this is an expression because if you don'y add a semi colon it'll return a value
        }
```

- `Statements` are codelines that do not return values, they end with semi colons

```RUST
    5+3;
```

- `Conditionals` the regular `if` and `else` 

```RUST

    fn less_five(x :i32){

        if x = 0{
            println!("x is equal to 0");
        }else if x < 5{
            // the condition must lead to a boolean data type
            println!("x is less than 5");
        }else{
            println!("x is either greater than  or equal to 5");
        }

    }

```