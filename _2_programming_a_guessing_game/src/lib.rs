use std::cmp::Ordering;
use rust_book_utilities;

use rand::Rng;

const CHAPTER_NAME: &str    = "2.0 programming a guessing game";
const CHAPTER_SUMMARY: &str = "\
in this chapter we crash coursed a guessing game";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }

    #[test]
    fn run_guessing_game() {
        let range = (0, 1000000);
        let secret_number = rand::thread_rng().gen_range(range.0..=range.1);
        println!("secret_number: {secret_number}");
        let mut remaining = (range.0, range.1);
        loop {
            let guess = rand::thread_rng().gen_range(remaining.0..=remaining.1);
            println!("guess: {guess}");
            /*io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {guess}");*/

            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("guess < secret");
                    remaining.0 = guess;
                },
                Ordering::Greater => {
                    println!("guess > secret");
                    remaining.1 = guess;
                },
                Ordering::Equal => {
                    println!("guess == secret");
                    assert_eq!(guess, secret_number);
                    break; //meow
                }
            }
        }
    }
}

/* ------------------------------------ chapter template
use rust_book_utilities;

const CHAPTER_NAME: &str    = "1.0 Chapter Name";
const CHAPTER_SUMMARY: &str = "\
filler";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}
 */