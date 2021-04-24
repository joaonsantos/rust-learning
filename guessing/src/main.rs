use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..26);

    loop {
        let mut guess = String::new();
        println!("\nPlease input your guess.");
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it");
                break;
            },
        };
    }
}
