use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("\nWelcome to the Guessing game!!\n");
    println!("> Please enter your input between 1-10.");

    loop {
        // take the input
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the line.");

        // parse it to number
        let guess: u32 = match user_input
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Enter a number you dumb XD");
                    continue;
                }
            };

        // check it
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small XD"),
            Ordering::Greater => println!("Too big ;)"),
            Ordering::Equal => {
                println!("You win :)");
                break;
            },
        }
    }
}
