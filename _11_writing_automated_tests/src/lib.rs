use rust_book_utilities;

const CHAPTER_NAME: &str    = "11.0 writing automated tests";
const CHAPTER_SUMMARY: &str = "\
11.1 - how to write tests;                  some basic test examples
11.2 - controlling how tests are run;       some example commands and ignored tests
11.3 - test organization                    setting up tests in nice ways;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _11_tests {
    use super::*;
    //----------------------------------------------------- 11.1 how to write tests
    #[test]
    fn _1_a_test() {
        fn add(left: usize, right: usize) -> usize {
            left + right
        }
        let n = add(2, 2); //4
        assert_eq!(n, 4); //wo
    }

    #[test]
    fn _2_second_test() {
        assert_eq!(1,1); //more than one! wo!
    }

    #[test]
    fn _2_using_a_regular_assert() {
        fn is_x_larger(x: usize, y: usize) -> bool {
            if x > y {
                true
            } else {
                false
            }
        }
        assert!(is_x_larger(10, 5)); //not assert_eq, assert, which just means true or false pass

        //in addition to assert_eq, assert_ne, which is asserted NOT equal, imaginatively
    }

    #[test]
    fn _1_using_a_format_result() {
        fn is_x_larger(x: usize, y: usize) -> bool {
            if x > y {
                true
            } else {
                false
            }
        }
        let r = is_x_larger(10, 20);

        assert!(is_x_larger(10, 5), "is larger: {}", r); //will show printout before fail
    }

    #[test]
    #[should_panic]
    fn _1_test_where_panic_is_expectation() {
        panic!("easy enough!"); //--- a panic is the passing condition
    }

    #[test]
    #[should_panic(expected = "easy enough!")]
    fn _1_panic_test_with_expected_result() {
        panic!("easy enough!");
        //panic!("nope!"); // will fail test - this lets the user be specific about panic a little
    }

    #[test]
    fn _1_testing_using_result() -> Result<(), String> { //similar to a box dyn error on fn main() ->
        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err("ouch".to_string())
        }
    }


    //----------------------------------------------------- 11.2 controlling tests

    //cargo test -- --test-threads=1            <--- run without multithreading
    //cargo test -- --show-output               <--- println! will show for tests
    //cargo test some_test_function_signature   <--- run a specific test
    //cargo test some                           <--- will run all tests starting with some
    #[test]
    #[ignore]
    fn _2_ignoring_a_test() {
        panic!("never fails unless this test is specifically told to run");
    }

    //----------------------------------------------------- 11.3 test organization

    //this work space is a testament to understanding these concepts

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}