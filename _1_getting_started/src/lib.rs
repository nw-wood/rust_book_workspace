use std::cmp::Ordering;
use rand::Rng;

//use std::process::Command;

const CHAPTER_NAME: &str    = "1.0 getting started";
const CHAPTER_SUMMARY: &str = "\
1.1 - installation;  how to install rust
1.2 - hello, world!; how to use rustc (easy mode)
1.3 - hello, cargo!; how to use the package manager

In this chapter we learned how to install rust, and
get some of the basics working with the compiler
and the package manager. Very little in terms of
code and much more in the way of setup.";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

fn guessing_game(range: (u32, u32)) {
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
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1);
    }

    #[test]
    fn run_guessing_game() {
        guessing_game((0, 1000));
    }
    /*#[test]
    fn chapter_1_test_idea() { //figure error handling better
        let output = Command::new("ls").output().expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }*/
}

//------------------------------------------------------------------------------------1.1
//$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
//$ xcode-select --install
//$ rustc --version
//$ echo $PATH
//$ rustup update
//$ rustup self uninstall

//------------------------------------------------------------------------------------1.2
/*
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
$ cargo new project     -------------- 1.3
$ cargo check           -------------- 1.3
$ cargo build           -------------- 1.3
$ cargo run             -------------- 1.3
*/

/*
$ cd project/src
$ rustc main.rs
$ ./main
$ Hello, world!
 */