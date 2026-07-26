use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    loop {
        println!("Please input your guess: ");
        // declaratie van string variable
        let mut guess = String::new();
        let random = rand::thread_rng().gen_range(1..=100);
        //println!("The secret number is: {}", random);

        //input leest de lijn en alloceert die naar guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&random) {
            Ordering::Less => println!("You guessed too small!"),
            Ordering::Greater => println!("You guessed too big"),
            Ordering::Equal => {
                println!("Congratulations! You got it!");
                break;
            },
        }
    }
}
