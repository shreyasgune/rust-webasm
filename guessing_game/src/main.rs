use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess your number");
    // Enter Your guess
    loop{
        // Rand Num Generator
        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("Your secret number is {}", secret_number);

        println!("Please input your number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");

        // Handle the case if the entered number is not a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You've not entered a number");
                continue;
            }
        };
        
        println!("You guessed {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }

}