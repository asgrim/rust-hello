use std::io;
use rand::Rng;
use std::cmp::Ordering;

/// Generate a random number between `low_inclusive` and `high_inclusive`. Because that makes more
/// sense than `gen_range` which is `low_inclusive` and `high_exclusive` ¯\_(ツ)_/¯
///
/// # Example
///
/// ```
/// let secret_number = make_secret_number();
/// println("Number is {}", secret_number);
/// ```
fn make_secret_number_in_range(low_inclusive: u32, high_inclusive: u32) -> u32 {
    return rand::thread_rng().gen_range(low_inclusive, high_inclusive + 1);
}

fn main() {
    println!("Guess the number!");

    let secret_number = make_secret_number_in_range(1, 100);

    loop {
        println!("Input guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} was not a number.", guess.trim());
                continue
            },
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
