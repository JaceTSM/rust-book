use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("TEMP TEST: The secret number is {secret_number}");

    let mut counter: u16 = 0;

    loop {
        println!("Please input a guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Invalid Input: {e}");
                continue;
            },
        };
        println!("You guessed: {guess}");
        counter += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win! You took {counter} guesses.");
                break;
            },
        }
    }
}
