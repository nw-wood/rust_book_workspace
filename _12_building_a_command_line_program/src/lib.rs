use rust_book_utilities;

const CHAPTER_NAME: &str    = "12.0 building a command line program";
const CHAPTER_SUMMARY: &str = "\
12.1 - accepting command line arguments;
12.2 - reading a file;
12.3 - refactoring to improve modularity and error handling;
12.4 - test driven development;
12.5 - working with environment variables;
12.6 - writing error messages to standard error;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _12_tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}