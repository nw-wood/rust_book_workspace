use rust_book_utilities;
use std::error::Error;
use std::fs::File;
use std::io::Read;

const CHAPTER_NAME: &str    = "12.0 building a command line program";
const CHAPTER_SUMMARY: &str = "\
12.1 - accepting command line arguments;                        skipped
12.2 - reading a file;                                          skipped
12.3 - refactoring to improve modularity and error handling;    using std::process::exit()
12.4 - test driven development;                                 skipped
12.5 - working with environment variables;                      skipped
12.6 - writing error messages to standard error;                using eprintln!()";

/*  --- the book expects this to be setup, but that's not going to happen
    --- instead it'll be chopped up, and tests out based on example
    --- std::in for multithreaded tests doesn't make sense
    --- INSTEAD: I'll be testing stuff I learn since this is supposed to just be review and not new

    An I/O Project: Building a Command Line Program

    ---------------------------------------------------------------------------

    grep project will combine a number of concepts learned so far:

        Organizing code (using what you learned about modules in Chapter 7)
        Using vectors and strings (collections, Chapter 8)
        Handling errors (Chapter 9)
        Using traits and lifetimes where appropriate (Chapter 10)
        Writing tests (Chapter 11)

    ---------------------------------------------------------------------------

 */

pub fn run_summary() {
    rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
    fn garbage() -> Result<String, Box<dyn Error>> {
        let mut garbo = File::open("garbitchslkda;lskda;salads")?;
        let mut s = String::new();
        garbo.read_to_string(&mut s)?;
        println!("garbo?: {}", s);
        Ok(s)
    }
    let g = garbage();
    drop(g); //try to read string from file, and drop the error, or result, doesn't matter
}

#[cfg(test)]
mod _12_tests {
    use super::*;
    //12.1 - nothing new
    //12.2 - nothing new
    //--------------------------------------------------------------- 12.3 refactoring
    #[test]
    #[ignore] //not actually returning a result here

    fn _3_using_process_exit_to_kill_main() {
        use std::process;
        process::exit(1); //rip
    }

    #[test]

    fn _3_returning_errors_from_a_function() -> Result<(), Box<dyn Error>> {
        Ok(())
        //let error = fs::read_to_string(""); //no file specified error probably
    }

    #[test]
    #[ignore]
    fn _3_handling_a_returned_error() {
        use std::process;
        use std::fs;
        fn returns_err() -> Result<(), Box<dyn Error>> {
            fs::read_to_string("asdbajsda")?; //propagate arbitrary error
            Ok(())
        }
        if let Err(e) = returns_err() {
            println!("application error: {e}");
            process::exit(1);
        }
    }
    //12.4 nothing new
    //12.5 nothing new
    //--------------------------------------------------------------- 12.6 writing to standard error
    #[test]
    fn _6_arbitrary_write_to_standard_error() {
        eprintln!("standard error");
    }
    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}