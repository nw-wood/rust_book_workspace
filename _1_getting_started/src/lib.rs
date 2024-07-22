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

#[cfg(test)]
mod _1 {
    use super::*;

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1);
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