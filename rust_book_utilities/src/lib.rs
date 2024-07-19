//every chapter gets this chunk of consts------------------

const CHAPTER_NAME: &str    = "The Rust Book Notes";
const CHAPTER_SUMMARY: &str = "\
Do what you don't understand until it bores you.
Embrace captivation, but abhor captivity.
-mr wood";

//---------------------------------------------------------

pub fn run_summary() { chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

const LENGTH_LIMIT: i32 = 50;
const HBAR_CHAR: char = '_';

pub fn chapter_summary(title: &str, summary: &str) {
    let mut s = String::from("______ ") + title + " ";
    let tail: i32 = LENGTH_LIMIT - s.len() as i32;
    if tail <= 0 {
        s = s[..LENGTH_LIMIT as usize].parse().unwrap();
    } else {
        for _ in 0..tail {
            s.push(HBAR_CHAR);
        }
    }
    println!("{s}\n");
    println!("{summary}\n");
    hbar();
}

pub fn hbar() {
    let mut s = String::from("");
    for _ in 0..LENGTH_LIMIT {
        s.push(HBAR_CHAR);
    }
    println!("{s}\n");
}

#[cfg(test)]
mod rust_book_utilities_tests {
    use super::*;

    #[test]
    fn show_summary() {
        chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1, 1); //later chapter_summary will return a Result<String, Err>? - brush up on error handling
    }
}

/* useful terminal commands -
    cargo test -p rust_book_utilities --lib
 */