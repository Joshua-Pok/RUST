use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    /*
       rand::thread_rng() gives the random number generator
       we then call gen_range on the number generator and pass a range to get a random number
    */

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //ok is what parse returns if it can parse correctly.
            //ok contains a number
            Err(_) => continue, /*
                                _ in this case is a match all pattern
                                where we want to catch all err values, no
                                matter what is inside them
                                */
        };
        /*
         * We trim the whitespace, parse the string into a number and call expect because it can
         * throw an error
         *
         * Parse method on strings converts string to another type, we tell rust what type to
         * convert to bty using :u32 to declare the type of 32bit unsigned integer
         * */

        println!("You guesed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
