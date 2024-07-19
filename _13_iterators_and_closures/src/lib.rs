use rust_book_utilities;

const CHAPTER_NAME: &str    = "13.0 iterators and closures";
const CHAPTER_SUMMARY: &str = "\
13.1 - closures: anon functions that capture their environment;
13.2 - processing series of items with iterators;
13.3 - improving the io project;
13.4 - comparing performance: loops vs iterators;";

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