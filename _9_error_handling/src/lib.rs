use rust_book_utilities;

const CHAPTER_NAME: &str    = "9.0 error handling";
const CHAPTER_SUMMARY: &str = "\
9.1 - unrecoverable errors with panic;
9.2 - recoverable errors with result;
9.3 - to panic! or not to panic!;";

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