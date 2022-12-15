use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut counter = 1u32;
    
    loop {
        println!("Please input your guess");
    
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue, 
            // continue,
            // why not something like { println!("msg"), continue, }
        };

        println!("You guessed: {guess}. Try number: {counter}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Exactly right! You Win :) ");
                break;
            }
        }

        counter += 1;
    }
    
    






}
