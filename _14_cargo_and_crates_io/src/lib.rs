use rust_book_utilities;

const CHAPTER_NAME: &str    = "14.0 cargo and crates.io";
const CHAPTER_SUMMARY: &str = "\
14.1 - customizing builds with release profiles;
14.2 - publishing crates to crates.io;
14.3 - cargo workspaces;
14.4 - installing binaries from crates.io;
14.5 - extending cargo with custom commands;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _14_tests {
    use super::*;

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}