use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();
        
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is: {secret_number}");


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess : u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number){
            Ordering::Greater => println!("too great"),
            Ordering::Less => println!("too Small"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }

}
