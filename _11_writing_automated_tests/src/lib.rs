use rust_book_utilities;

const CHAPTER_NAME: &str    = "11.0 writing automated tests";
const CHAPTER_SUMMARY: &str = "\
11.1 - how to write tests;
11.2 - controlling how tests are run;
11.3 - test organization;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _11_tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}