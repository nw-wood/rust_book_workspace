use rust_book_utilities;

const CHAPTER_NAME: &str    = "7.0 packages and crate modules";
const CHAPTER_SUMMARY: &str = "\
7.1 - packages and crates;
7.2 - defining modules to control scope and privacy;
7.3 - paths of referring to items on module tree;
7.4 - bringing paths into scope with the use keyword;
7.5 - separating modules into different files;";

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