use rust_book_utilities;

const CHAPTER_NAME: &str    = "15.0 smart pointers";
const CHAPTER_SUMMARY: &str = "\
15.1 - using Box<T> to point at heap data;
15.2 - treating smart pointers like references with the deref trait;
15.3 - running code on cleanup with the drop trait;
15.4 - Rc<t>, the reference counted smart pointer;
15.5 - RefCell<T> and the interior mutability pattern;
";

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