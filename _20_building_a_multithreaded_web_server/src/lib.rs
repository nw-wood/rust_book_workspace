use rust_book_utilities;

const CHAPTER_NAME: &str    = "20.0 building a multithreaded web server";
const CHAPTER_SUMMARY: &str = "\
20.1 - building a single-threaded server;
20.2 - turning our single-threaded server into a multithreaded one;
20.3 - graceful shutdown and cleanup;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _20_tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}