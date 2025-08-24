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
    Let Tup = (1, 2,"ewa", -5, 9.4)
    
    //OR

    Let Tup2 :(164, 132, str, u64, f32) = (1, 2,"ewa", -5, 9.4)

    println!("my name is {} ,  I am {} years old , my wallet balance is ${}", tup.2, tup.1, tup.4)
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
    ```