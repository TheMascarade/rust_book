use rand::Rng;
use std::cmp::Ordering;
use std::io;

enum MessageType {
    Warning,
    Info,
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    let tries = 1..=5;
    let end = *tries.end();
    for _count in tries {
        let guess = get_user_number(get_user_input());
        let left = end - _count;
        pretify_message(
            format!("You have {left} tries left").as_str(),
            MessageType::Info,
        );
        match guess.cmp(&secret_number) {
            Ordering::Less => pretify_message("Its higher", MessageType::Warning),
            Ordering::Equal => {
                pretify_message("You guessed right!", MessageType::Info);
                break;
            }
            Ordering::Greater => pretify_message("Its lower", MessageType::Warning),
        }
    }
    pretify_message("You loose!", MessageType::Warning)
}

fn pretify_message(msg: &str, msg_type: MessageType) {
    match msg_type {
        MessageType::Warning => {
            println!("^^^^^^^^^^^^^^^^^^^^^");
            println!("{}", msg);
            println!("^^^^^^^^^^^^^^^^^^^^^");
        }
        MessageType::Info => {
            println!("---------------------");
            println!("{}", msg);
            println!("---------------------");
        }
    }
}

fn get_user_input() -> String {
    let mut number = String::new();
    loop {
        pretify_message("Please enter a number", MessageType::Info);
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
