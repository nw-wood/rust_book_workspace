use rust_book_utilities;

const CHAPTER_NAME: &str    = "21.0 appendix";
const CHAPTER_SUMMARY: &str = "\
21.1 - a - keywords;
21.2 - b - operators and symbols;
21.3 - c - derivable traits;
21.4 - d - useful development tools;
21.4 - e - editions;
21.5 - f - translations of the book;
21.6 - g - how rust is made and 'nightly rust';";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _21_tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}