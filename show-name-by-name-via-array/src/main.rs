fn main() {
    let name_array = ["Adarsh", "Gupta"];
    let mut first_loop_iter: usize = 0;

    // here 'first_loop is a label for this loop
    'first_loop: loop {
        if first_loop_iter >= name_array.len() {
            // used the 'first_loop label to break the loop
            break 'first_loop;
        } else {
            // add space between the two names, only if it has been iter more than once
            if first_loop_iter != 0 { println!("\t") }
        }

        // get the array value
        let value = name_array[first_loop_iter];
        let mut second_loop_iter: usize = 0;

        // here 'second_loop is a label for this loop
        'second_loop: loop {
            if second_loop_iter >= value.len() {
                // used the 'second_loop label to break the loop
                break 'second_loop;
            }

            // convert the array value's char (respective to the iter value), convert it to char and make it uppercase
            let value_char = value.chars().nth(second_loop_iter).unwrap().to_uppercase();
            println!("{}", value_char);

            second_loop_iter += 1;
        }

        first_loop_iter += 1;
    }
}
