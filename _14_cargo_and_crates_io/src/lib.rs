use rust_book_utilities;

const CHAPTER_NAME: &str    = "14.0 cargo and crates.io";
const CHAPTER_SUMMARY: &str = "\
14.1 - customizing builds with release profiles;
14.2 - publishing crates to crates.io;
14.3 - cargo workspaces;
14.4 - installing binaries from crates.io;
14.5 - extending cargo with custom commands;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _14_tests {
    use super::*;
    //14.1 skip with comments - this is about setting optimization levels for build profiles
    /*
        $ cargo build
            Finished dev [unoptimized + debuginfo] target(s) in 0.0s
        $ cargo build --release
            Finished release [optimized] target(s) in 0.0s

        Filename: Cargo.toml
        ---------------------------------
        |   [package]
        |   name = "guessing_game"
        |   version = "0.1.0"               <--- stuff needed to publish to crates.io
        |   edition = "2021"
        |   description = "A fun game where you guess what number the computer has chosen."
        |   license = "MIT OR Apache-2.0"
        |
        |   [profile.dev]
        |   opt-level = 0       <--- set the opt-level to 0 for the dev (the default)
        |
        |   [profile.release]
        |   opt-level = 3       <--- set to 3 for release (the default)
        |
        |   [profile.dev]
        |   opt-level = 1       <--- set to 1 for dev, non-standard, applies some optimizations
        ---------------------------------

        by default the opt level for dev is 0, and 3 for release
     */
    //14.2 skip with comments - this is about doc comments, and publishing on crates.io
    /*

        /// Adds one to the number given.
        ///
        /// # Examples
        ///
        /// ```
        /// let arg = 5;
        /// let answer = my_crate::add_one(arg);
        ///
        /// assert_eq!(6, answer);
        /// ```
        pub fn add_one(x: i32) -> i32 {
            x + 1
        }

        /// # Panics    <--- cases that result in panics as a forewarning
        /// # Errors    <--- describe error kinds in the case of a result
        /// # Safety    <--- a section that explains why a function is unsafe

        Documentation comments as tests ---
        Rust is shmart, and will know that code block in the doc comment is a test for this function

        //! syntax refers to the comments container whereas /// refers to what immediately follows

        in order to create a section for the crate itself in the main file (oft lib.rs):

        //! # My Crate
        //!
        //!'my_crate' is a collection of code that executes such and such way
        //!bla bla bla, lorem ipsum - describe the crate and uses etc

        /// Function description doc comments would follow to describe functions in lib.rs

        //need a crates.io account to publish stuff

        //$ cargo login                                 <--- will log in to crates.io
        //$ crate.io-token-sdjaksdlajksda10123          <--- api key for crates.io for account

        //$ cargo publish                               <--- publishes the current version
        //$ cargo yank --vers 1.0.1                     <--- deprecates a version

        //publishing a crate version is permanent, and substitutes
        //
     */
    //14.3 about workspaces - described in terminal

    /*
            creating a workspace ---

               $ mkdir add
               $ cd add
               $ cargo new adder
               $ cargo new add_one --lib
               $ cargo new add_two --lib        <-- book says to add for practice

               --------------------------- Filename: /add/Cargo.toml

               [workspace]

                members = [
                    "adder",
                    "add_one",
                    "add_two",
                ]
                --------------------------- Filename: /add/add_two/src/lib.rs

                pub fn add_two(x: i32) -> i32 {
                    x + 2
                }

                --------------------------- Filename: /add/add_one/src/lib.rs
                use rand;

                pub fn add_one(x: i32) -> i32 {
                    x + 1
                }

                #[cfg(test)]
                mod tests {
                    use super::*;

                    #[test]                     //<--- test added to the workspace
                    fn it_works() {
                        assert_eq!(3, add_one(2));
                    }
                }

                --------------------------- Filename: /add/adder/Cargo.toml

                [dependencies]
                add_one = { path = "../add_one" }
                add_two = { path = "../add_two" }

                --------------------------- Filename: /add/adder/src/main.rs

                use add_one;
                use add_two;

                fn main() {
                    let num = 10;
                    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
                    println!("hello, again, {num} + 2 is {}", add_two::add_two(num));
                }

                --------------------------- Filename: /add/add_one/Cargo.toml

                [dependencies]
                rand = "0.8.5"

                ---------------------------

                $ cargo test -p add_one
                   Finished test [unoptimized + debuginfo] target(s) in 0.00s
                     Running unittests src/lib.rs (target/debug/deps/add_one-b3235fea9a156f74)

                running 1 test
                test tests::it_works ... ok

                test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

                   Doc-tests add_one

                running 0 tests

                test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     */
    //14.4 skip - installing binaries from crates.io - $ cargo install ripgrep
    //14.5 skip - extending cargo -  binaries in $path with the prefix cargo- can run as cargo sub cmds
    //          - custom commands like this can be listed with cargo --list
    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}