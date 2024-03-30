use std::io;
use rand::Rng;

static mut TARGET: u8 = 0;
static mut NUM_GUESSES: u8 = 0;

fn main() {
    generate_target();

    loop {
        let guess = take_input();
        if check_guess(guess) {
            break;
        }
    }
}

fn generate_target() {
    unsafe {
        let mut rng = rand::thread_rng();
        TARGET = rng.gen_range(0..=100);
    }
}

fn take_input() -> u8 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");
        match input.trim().parse() {
            Ok(num) => {
                if num <= 100 {
                    return num;
                } else {
                    println!("Invalid input");
                }
            }

            Err(_) => {
                println!("Invalid input")
            }
        }
    }
}

fn check_guess(guess: u8) -> bool {
    unsafe {
        NUM_GUESSES += 1;

        if guess == TARGET {
            println!("You won in {} guesses", NUM_GUESSES);
            return true;
        } else if guess > TARGET {
            println!("Lower");
        } else {
            println!("Higher");
        }
    }

    false
}

