use rust_book_utilities;

const CHAPTER_NAME: &str    = "5.0 using structs to structure related data";
const CHAPTER_SUMMARY: &str = "\
5.1 - defining and instantiating structs;
5.2 - an example program using structs;
5.3 - method syntax;

learned some stuff here - am sure of it";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _5_tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}