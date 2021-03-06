use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the whole number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => { 
		println!("Perfect!");
		break;
	}
            Ordering::Greater => println!("Too High!"),
        }
    }
}
