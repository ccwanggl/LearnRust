use rand::Rng;
use std::cmp::Ordering;
use std::io;        //prelude

fn main() {

    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input a number:");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        // Use match to deal the error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guess is: {}", guess);
        
        match guess.cmp(&secret_number){
            Ordering::Less =>println!("Too small!"),
            Ordering::Greater =>println!("Too big!"),
            Ordering::Equal =>
            {
                println!("You win!");
                break;
            }
        }
    }

}
