use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MIN_NUMBER_INCLUSIVE: u32 = 1;
const MAX_NUMBER_INCLUSIVE: u32 = 10;

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

pub fn run() {
    println!("Guess the number, between {} and {}!", MIN_NUMBER_INCLUSIVE, MAX_NUMBER_INCLUSIVE);

    let secret_number = make_secret_number_in_range(MIN_NUMBER_INCLUSIVE, MAX_NUMBER_INCLUSIVE);

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

#[cfg(test)]
mod tests {
    use crate::game::make_secret_number_in_range;

    #[test]
    fn check_upper_lower_bounds() {
        let min = 1;
        let max = 2;

        for _ in 0..10 // Testing randomness is fun...
        {
            let n = make_secret_number_in_range(min, max);
            assert!(n >= min, "{} was below the min bound {}", n, min);
            assert!(n <= max, "{} was above the max bound {}", n, max);
        }
    }
}
