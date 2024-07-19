use rust_book_utilities;

const CHAPTER_NAME: &str    = "8.0 common collections";
const CHAPTER_SUMMARY: &str = "\
8.1 - storing lists of values with vectors;
8.2 - storing utf-8 encoded text with strings;
8.3 - storing keys with values in hash maps;";

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