use std::io;

fn main() {
    println!{"Welcome to the Roll-A-Dice!\nGuess a value between 1-6 and win the game ;)\n"}

    'game_loop: loop {
        let user_input = get_user_input();

        match user_input {
            -1 => continue,
            5 => {
                println!("Congratulations!! You won the game.");
                break 'game_loop;
            }
            _ => {
                if user_input as u64 > 6 {
                    println!("Guess a value between 1-6.");
                    continue;
                }
                println!("You guessed wrong!!");
            },
        }
    }
}

fn get_user_input() -> i64 {
    // get the user's input in integer
    let mut user_input: String = String::new();
    println!("\n> Enter your guess between 1-6:");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the number. Try again.");

    if user_input.len() == 0 {
        return -1;
    }

    // convert it to number
    let user_guess: u64 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("You haven't entered a valid number! Try again.");
            return -1;
        }
    };

    user_guess as i64
}
