use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let lower_bound: u32 = 1;
    let upper_bound: u32 = 100;
    let secret_number: u32 = rand::thread_rng().gen_range(lower_bound..(upper_bound));

    loop {
        println!("Please, input your guess (between 1 and 100):");

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
