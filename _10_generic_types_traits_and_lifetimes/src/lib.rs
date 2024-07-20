use rust_book_utilities;

const CHAPTER_NAME: &str    = "10.0 generic types, traits, and lifetimes";
const CHAPTER_SUMMARY: &str = "\
10.1 - generic data types;
10.2 - traits: defined shared behavior;
10.3 - validating references with lifetimes;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _10_tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}