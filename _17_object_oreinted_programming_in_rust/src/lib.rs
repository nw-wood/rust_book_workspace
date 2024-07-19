use rust_book_utilities;

const CHAPTER_NAME: &str    = "17.0 object oriented functionality in rust";
const CHAPTER_SUMMARY: &str = "\
17.1 - characteristics of object oriented languages;
17.2 - using trait objects that allow for values of different types;
17.3 - implementing an object-oriented design pattern;";

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