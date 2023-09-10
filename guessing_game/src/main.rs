use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    let tries = 1..=5;
    let end = *tries.end();
    let mut guess = get_user_number(get_user_input());
    for _count in tries {
        println!("You have {} tries left", end - _count);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Its higher"),
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            }
            Ordering::Greater => println!("Its lower"),
        }
        guess = get_user_number(get_user_input());
    }
}

fn get_user_input() -> String {
    let mut number = String::new();
    loop {
        println!("Please enter a number");
        match io::stdin().read_line(&mut number) {
            Result::Ok(_) => return number,
            Result::Err(_) => {
                eprintln!("Failed to read line, trye again");
                continue;
            }
        }
    }
}

fn get_user_number(mut user_string: String) -> u32 {
    let number: u32;
    loop {
        number = match user_string.trim().parse() {
            Result::Ok(value) => value,
            Result::Err(_) => {
                eprintln!("Input must be a whole number");
                user_string = get_user_input();
                continue;
            }
        };
        return number;
    }
}
