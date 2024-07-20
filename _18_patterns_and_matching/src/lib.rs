use rust_book_utilities;

const CHAPTER_NAME: &str    = "18.0 patterns and matching";
const CHAPTER_SUMMARY: &str = "\
18.1 - all the places patterns can be used;
18.2 - refutability: whether a pattern might fail to match;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _18_tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}