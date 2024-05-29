use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;


fn main() {
    println!("Input guess:");
    
    
    let secret_number=rand::thread_rng().gen_range(1,101);

    loop {    
        let mut guess=String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        println!("The secret number is: {}",secret_number);
        

        let guess:u32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}","That's not a number! Try again.".magenta()); 
                continue;
            },
        };

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number){
            Ordering::Equal =>{
                println!("{}","Correct!".green());
                break;
            },
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Less => println!("{}","Too small!".red()),
        }
        println!("Try again?")
    }
}
