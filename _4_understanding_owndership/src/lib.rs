use rust_book_utilities;

const CHAPTER_NAME: &str    = "4.0 understanding ownership";
const CHAPTER_SUMMARY: &str = "\
4.1 - what is ownership?;
4.2 - references and borrowing;
4.3 - the slice type;

in this chapter we learned some stuff";

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