use rust_book_utilities;

const CHAPTER_NAME: &str    = "3.0 common programming concepts";
const CHAPTER_SUMMARY: &str = "\
3.1 - variables and mutability; 'let this = that'
3.2 - data types;               'i32, f64, arrays, etc'
3.3 - functions;                'different types of signatures'
3.4 - comments;                 '//'
3.5 - control flow;             'if for while loop else'

In this chapter we go over some stuff!";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _3 {
    use super::*;


    //--------------------------------------------- 3.1 variables and mutability
    #[test]
    fn _1_mutable_x() {
        let mut x = 5;
        println!("x: {x}");
        x = 6;
        println!("x: {x}");
        assert_eq!(x, 6);
    }


    #[test]
    fn _1_shadow_x() {
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            println!("x: {x}");
        }
        assert_eq!(x, 6);
    }


    #[test]
    fn _1_shadow_length_test() {
        let spaces = "    ";
        let spaces = spaces.len();
        let mut spaces = spaces;
        spaces += 1;
        //you must use let
        // spaces = "   "; rizzes the piler
        println!("spaces: {spaces}");
        assert_eq!(spaces, 5);
    }


    //--------------------------------------------- 3.2 data types
    #[test]
    fn _2_some_floating_point_types() {
        let x = 3.0;
        let y: f32 = 3.0;
        assert_eq!(x, y);
    }


    #[test]
    fn _2_some_basic_operations() {
        let sum = 5 + 10;
        let difference = 96.5 - sum as f64;
        let product = difference * 2.0;
        let quotient = product / 32.2;
        let truncated = quotient as i32 + 5 / 3;
        let remainder = truncated + 45 % 5;
        println!("remainder: {remainder}");
        assert_eq!(remainder, 6);
    }


    #[test]
    fn _2_booly_fooly() {
        let t = true;
        let f: bool = false;
        assert_eq!(f, false);
        assert_eq!(t, true);
    }


    #[test]
    fn _2_the_character_type() {
        let c = 'z';
        let mut d = String::new();
        d.push(c);
        assert_eq!(d, "z");
    }


    #[test]
    fn _2_the_tuple_type() {
        let tup: (i32, f64, u8, char) = (500, 6.4, 1, '1');
        let (x, y, z, w) = tup; //destructured tup
        assert_eq!(x, 500);
        assert_eq!(y, 6.4);
        assert_eq!(z, 1);
        assert_eq!(tup.3, w); //(tup dot number notation)
    }


    #[test]
    fn _2_the_array_type() {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        for e in a { println!("{e}"); }
        let a = [ 3; 5]; //produces [3, 3, 3, 3, 3]! (hmm)
        for e in a { assert_eq!(e, 3); }
        assert_eq!(a[1], 3); //array bracketed index notation
        //attempting to access a[10] panics with 'index out of bounds: len is 5, index is 10'
    }


    //--------------------------------------------- 3.3 functions
    #[test]
    fn _3_functional_fun() {
        fn plus_one(x: i32) -> i32 {
            x + 1
        }
        fn beep(x: i32, y: String) -> i32 {
            println!("{y}");
            assert_eq!(x, 3);
            x + 10
        }
        let x = beep(3, String::from("hello, test"));
        assert_eq!(x, 13);
        //let y = (let x = 3); does not make sense
        let y = { //scope statement to evaluate and return expression
            let x = 3;
            x + 10
        };
        let y = plus_one(y);
        assert_eq!(plus_one(x), y);
    }


    //--------------------------------------------- 3.4 comments
    /*#[test]
    fn this_is_a_comment() {
        println!("this won't ever happen because it's in a comment block");
    }
    this code does not exist
     */


    //--------------------------------------------- 3.5 control flow
    #[test]
    fn _5_if_expressions() {
        let number = 3;
        if number == 3 {
            assert_eq!(number, 3);
        } else {
            assert_eq!(number, number);
        }
    }


    #[test]
    fn _5_if_in_let_expressions() {
        let condition = true;
        let number = if condition { 5 } else { 6 };
        assert_eq!(number, 5);
    }


    #[test]
    fn _5_loop_and_break() {
        let mut count = 0;
        loop {
            count += 1;
            if count == 10 { break; }
        }
        assert_eq!(count, 10);
    }


    #[test]
    fn _5_loop_labels() {
        //these allow for embedded loops where you can break out of a loop explicitly
        let mut count = 0;
        'outer_loop: loop {
            let mut inside_count = 0;
            count += 1;
            'inner_loop: loop {
                inside_count += 1;
                if inside_count + count == 20 { break 'outer_loop; }
                else if inside_count == 10 { break 'inner_loop; }
            }
        }
        assert_eq!(count, 10);
    }


    #[test]
    fn _5_while_conditional_loop() {
        let mut n = 3;
        while n != 0 {
            n-=1;
        }
        assert_eq!(n, 0);
    }


    #[test]
    fn _5_basic_for_loops() {
        let a = [10; 10];
        for e in a {
            assert_eq!(e, 10);
        }
        //for e in (1..4).rev() { etc }
        //slice tests later
    }


    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1);
    }
}