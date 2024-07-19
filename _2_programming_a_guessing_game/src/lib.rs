use rust_book_utilities;

const CHAPTER_NAME: &str    = "2.0 programming a guessing game";
const CHAPTER_SUMMARY: &str = "\
in this chapter we crash coursed a guessing game";

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

/* ------------------------------------ chapter template
use rust_book_utilities;

const CHAPTER_NAME: &str    = "1.0 Chapter Name";
const CHAPTER_SUMMARY: &str = "\
filler";

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
 */