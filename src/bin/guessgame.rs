use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Guess the number");
        println!("Enter plxxx:");

        // create allocation for a new guess input
        let mut guess = String::new();
        // get and store the new input into guess
        io::stdin().read_line(&mut guess).expect("Failed to read");

        if guess.trim() == "quit" {
            return;
        };

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let random = thread_rng().gen_range(1..=99);

        println!("Random: {random}");
        println!("You guessed: {guess}");

        match guess.cmp(&random) {
            Ordering::Less => println!("Greater"),
            Ordering::Equal => println!("Ting Ting Ting!!"),
            Ordering::Greater => println!("Smaller"),
        }
    }
}
