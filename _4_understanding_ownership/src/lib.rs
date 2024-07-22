use rust_book_utilities;

const CHAPTER_NAME: &str    = "4.0 understanding ownership";
const CHAPTER_SUMMARY: &str = "\
4.1 - what is ownership?;       sharing isn't caring in rust
4.2 - references and borrowing; references, but restrictive as heck
4.3 - the slice type;           array slice notation!

in this chapter the book dealt with lots of string ownership examples
as well as how both mutable and regular references behave -
lastly, MOFUKN' GREY GOOSH BAYBA.. array slices are great c:";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _4 {
    use super::*;
    //--------------------------------------------- 4.1 what is ownership
    #[test]
    fn _1_string_mutation() {
        let mut s = String::from("hello");
        s.push_str(" world");
        println!("{s}");
        assert_eq!(s, String::from("hello world"));
    }
    #[test]
    fn _1_borrowing_with_some_types() {
        let s1 = String::from("hello");
        let s2 = s1; //s1 moves to s2
        //println!("s1: {s1}"); since s1 moved to s2, s1 can not be borrowed by println!
        //assert_eq!(s1, s2); will not work for the same reason
        assert_eq!(s2, String::from("hello"));
    }
    #[test]
    fn _1_cloning_instead_of_moving() {
        let s1 = String::from("hi");
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }
    #[test]
    fn _1_stack_only_copying() {
        let x = 5; //integer in stack not heap like a string
        let y = x; //x does not move into y, it is copied
        assert_eq!(y, x);
    }
    #[test]
    fn _1_ownership_with_function_params() {
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
    fn _1_return_moved_value_from_function() {
        fn returns_input(s: String) -> String {
            s //immediately returns the input 's'
        }
        let mut s = String::from("hi");
        s = returns_input(s);
        assert_eq!(s, String::from("hi"));
    }
    #[test]
    fn _1_return_s_with_len_as_tup() {
        fn calc_len(s: String) -> (usize, String) {
            (s.len(), s) //in bytes not chars or graphemes
        }
        let tup = calc_len(String::from("hi"));
        assert_eq!(tup.0, 2usize);
    }
    //--------------------------------------------- 4.2 references and borrowing
    #[test]
    fn _2_example_reference_usage() {


        fn calculate_length(s: &String) -> usize { s.len() } //s: &String drops
        let s = String::from("hi");
        let len = calculate_length(&s); //takes ownership of a reference, not s
        assert_eq!(len, 2);
        //references will allow you to refer to a value without taking ownership of it
        /*let s = String::from("hi"); --- figure out derefs - why is clone needed?
        //let r = &s;
        //let r2 = &s;
        let r3 = &s;
        //fn ref_test<'a>(r: &'a String, r2: &'a String) -> (&'a String, &'a String) {
        //    (r, r2) //playing with lifetimes early - r, r2, (r, r2) return all the same?
        //}
        //let t = ref_test(r, r2);
        //println!("{}, {}", t.0, t.1);
        //assert_eq!(t.0, r);
        //let s = r.clone();
        println!("ssss: {s}");
        //println!("r3: {}", * r3);
        //let x = *r3;
        let x = r3.clone();
        assert_eq!(x, s);*/

        /*let mut s = String::from("hi");
        let mut r = &mut s;
        let mut p = &mut r; //mut ref gets moved into p from r
        let n = &mut p;
        n.push_str("ya"); //n dereferences all the way back to mut s address?
        println!("s: {s}");*/
    }
    #[test]
    fn _2_mutable_references() {
        fn change(s: &mut String) {
            s.push_str("ya"); //mutate the data the mut ref pointed to
        }
        let mut s = String::from("hi");
        change(&mut s);
        //it's important to know that if a reference to a variable exists
        //then it can not also have an existing mutable reference
        //although many refs to one thing can exist otherwise

        //interestingly so long as references are used the compiler can work out
        //whether it's okay to introduce a mutable reference afterward
        //let r1 = &s;
        //let r2 = &s;
        //println!("{}{}", r1, r2);
        //let r3 = &mut s; <----------------- this would be fine

    }
    //--------------------------------------------- 4.3 the slice type
    #[test]
    fn _3_an_enumerated_iterator() {
        fn first_word(s: &String) -> usize {
            assert_eq!(&[b'h',b'i'], String::from("hi").as_bytes());
            let bytes = s.as_bytes(); //as_bytes returns a byte array reference
            for (i, &item) in bytes.iter().enumerate() { //arrays take the .iter() method
                if item == b' ' {
                    return i;
                }
            }
            s.len()
        }
        let a = [1, 2, 3];
        let mut iterator = a.iter();
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&3)); //more messing about
        assert_eq!(iterator.next(), None);
        let mut s = String::from("hi ho");
        let index = first_word(&s);
        s.clear();
        assert_eq!(s, "");
        assert_eq!(index, 2usize);
    }
    #[test]
    fn _3_string_slices() {
        let s = String::from("hi");
        let h = &s[..1];
        let i = &s[1..];
        let hi= &s[..];
        println!("{}{}", h, i);
        assert_eq!(h, "h");
        assert_eq!(i, "i");
        assert_eq!(hi, "hi");
    }
    #[test]
    fn _3_first_word_with_slices_instead() {
        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
            &s[..] //returns string slices now
        }
        let h_i = String::from("h i");
        let x = first_word(&h_i);
        assert_eq!(x, "h");
    }
    #[test]
    fn _3_first_word_with_str_signature() {
        fn first_word(s: &str) -> &str {
            let b = s.as_bytes();
            for (i, &item) in b.iter().enumerate() {
                if item == b' ' { return &s[0..i]; }
            }
            &s[..] //you can take a slice ref from a string ref as well
        }
        let s = first_word("h i");
        assert_eq!(s, "h");
        //slices have less to do with strings and more to do with arrays
        //any types of array references can be sliced this way into another
    }
    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}