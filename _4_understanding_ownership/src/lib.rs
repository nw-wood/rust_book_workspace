use rust_book_utilities;

const CHAPTER_NAME: &str    = "4.0 understanding ownership";
const CHAPTER_SUMMARY: &str = "\
4.1 - what is ownership?;
4.2 - references and borrowing;
4.3 - the slice type;

in this chapter we learned some stuff";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _4_tests {
    use super::*;
    //--------------------------------------------- 4.1 what is ownership
    #[test]
    fn string_mutation() {
        let mut s = String::from("hello");
        s.push_str(" world");
        println!("{s}");
        assert_eq!(s, String::from("hello world"));
    }
    #[test]
    fn borrowing_with_some_types() {
        let s1 = String::from("hello");
        let s2 = s1; //s1 moves to s2
        //println!("s1: {s1}"); since s1 moved to s2, s1 can not be borrowed by println!
        //assert_eq!(s1, s2); will not work for the same reason
        assert_eq!(s2, String::from("hello"));
    }
    #[test]
    fn cloning_instead_of_moving() {
        let s1 = String::from("hi");
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }
    #[test]
    fn stack_only_copying() {
        let x = 5; //integer in stack not heap like a string
        let y = x; //x does not move into y, it is copied
        assert_eq!(y, x);
    }
    #[test]
    fn ownership_with_function_params() {
        fn takes_ownership(s: String) {
            println!("{s}");
        }
        fn does_not_take(i: i32) {
            println!("{i}");
        }
        let s = String::from("hi");
        let i = 22;
        takes_ownership(s);
        does_not_take(i);
        //assert_eq!(s, String::from("hi")); string moved to function
        assert_eq!(i, 22); //integer wasn't moved, it was copied
    }
    #[test]
    fn return_moved_value_from_function() {
        fn returns_input(s: String) -> String {
            s //immediately returns the input 's'
        }
        let mut s = String::from("hi");
        s = returns_input(s);
        assert_eq!(s, String::from("hi"));
    }
    #[test]
    fn return_s_with_len_as_tup() {
        fn calc_len(s: String) -> (usize, String) {
            (s.len(), s) //in bytes not chars or graphemes
        }
        let tup = calc_len(String::from("hi"));
        assert_eq!(tup.0, 2usize);
    }
    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}