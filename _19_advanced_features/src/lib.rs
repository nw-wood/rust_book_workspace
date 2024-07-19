use rust_book_utilities;

const CHAPTER_NAME: &str    = "19.0 advanced features";
const CHAPTER_SUMMARY: &str = "\
19.1 - unsafe rust;
19.2 - advanced traits;
19.3 - advanced types;
19.4 - advanced functions and closures;
19.5 - macros;";

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