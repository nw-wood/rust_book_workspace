use rust_book_utilities;

const CHAPTER_NAME: &str    = "16.0 fearless concurrency";
const CHAPTER_SUMMARY: &str = "\
16.1 - using threads to run code simultaneously;
16.2 - using message passing to transfer data between threads;
16.3 - shared-state concurrency;
16.4 - extensible concurrency with the sync and send traits;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _16_tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}