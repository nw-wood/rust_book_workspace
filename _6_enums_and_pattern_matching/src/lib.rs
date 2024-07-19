use rust_book_utilities;

const CHAPTER_NAME: &str    = "6.0 enums and pattern matching";
const CHAPTER_SUMMARY: &str = "\
6.1 - defining and enum;
6.2 - the match control flow construct;
6.3 - concise control flow with if let;";

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