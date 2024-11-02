use std::io;

fn main() {
    let array = [1, 2, 3, 4, 5, 6];

    println!("\nHere you will access the value of an array by providing its index number!\n");
    println!("> Your input:");

    // get input from user
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the line");

    // parse the number from the input
    let index: usize = match user_input
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!!");
                return;
            }
        };

    // check if its a valid index
    if index >= array.len() {
        println!("Give a correct index within {}, sir :(", array.len() - 1);
        std::process::exit(0);
    }

    println!("The value for the index {} is {}.", index, array[index]);
}


