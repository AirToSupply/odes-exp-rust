use colored::*;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("<<Welcome to guess game!>>");

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);
    println!("Secret number is: {}", secret_number);

    loop {
        println!("Please input a number: ");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Input Error!");
        println!("Your number is: {}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => 
                println!("{}", "Too less!".red()),
            Ordering::Greater => 
                println!("{}", "Too greater!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }

}
