use rust_book_utilities;

const CHAPTER_NAME: &str    = "3.0 common programming concepts";
const CHAPTER_SUMMARY: &str = "\
3.1 - variables and mutability; basic functions
3.2 - data types;
3.3 - functions;
3.4 - commands;

In this chapter we go over some stuff!";

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