fn main(){

}

pub fn print_inf(x :&str, c : i32){

    let mut count = 0;
    loop{

        if count < c {

            count += 1;

            if count % 2 == 1{

                println!("{count} - {x}");
            }else {
                println!("*caveman rocks*");
            }
    

        }else {
            break ;
        }


    }

}