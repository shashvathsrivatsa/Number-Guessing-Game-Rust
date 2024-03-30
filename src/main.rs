use std::io;
use rand::Rng;

struct GameState {
    target: u8,
    num_guesses: u8,
}

impl GameState {
    fn new() -> Self {
        let target = rand::thread_rng().gen_range(0..=100);
        GameState { target, num_guesses: 0 }
    }

    fn check_guess(&mut self, guess: u8) -> bool {
        self.num_guesses += 1;

        if guess == self.target {
            println!("You won in {} guesses", self.num_guesses);
            return true;
        } else if guess > self.target {
            println!("Lower");
        } else {
            println!("Higher");
        }

        false
    }
}

fn main() {
    let mut game = GameState::new();

    loop {
        let guess = take_input();
        if game.check_guess(guess) {
            break;
        }
    }
}

fn take_input() -> u8 {
    loop {
        let mut input = String::new();
        println!("\nEnter a number from 0 to 100: ");
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

