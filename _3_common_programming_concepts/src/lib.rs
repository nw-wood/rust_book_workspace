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
    fn mutable_x() {
        let mut x = 5;
        println!("x: {x}");
        x = 6;
        println!("x: {x}");
        assert_eq!(x, 6);
    }

    #[test]
    fn shadow_x() {
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            println!("x: {x}");
        }
        assert_eq!(x, 6);
    }

    #[test]
    fn shadow_length_test() {
        let spaces = "    ";
        let spaces = spaces.len();
        let mut spaces = spaces;
        //you must use let
        // spaces = "   "; rizzes the piler
        println!("spaces: {spaces}");
        assert_eq!(spaces, 4);
    }

    //that's it for ch 3

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}