#![allow(unused_imports)]
#![allow(unused_must_use)]

use rand::Rng;
use std::io::stdin;

pub fn number_guessing_game() {
    let number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Enter your guess!");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>(); //the line break is added into the buffer if you don't trim it from end
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("your guess is out of range");
                        } else if guess < number {
                            println!("your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct!!!");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!", e)
                    }
                }
            }
            Err(_) => continue,
        }
    }
}
