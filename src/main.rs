use std::io;
use rand::{thread_rng,Rng};
use std::cmp::Ordering;

fn main() {
    println!("guess the number");


    let secret_number = thread_rng().gen_range(0..=100);
    println!("secret_number is {}", secret_number);
    loop {
        println!("please enter your number");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read_line");
        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("try again");
                continue;
            },
        };

        println!("guess the number is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You are good");
                break;
            },
        }
    }

}
