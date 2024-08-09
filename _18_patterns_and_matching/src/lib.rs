use rust_book_utilities;

const CHAPTER_NAME: &str    = "18.0 patterns and matching";
const CHAPTER_SUMMARY: &str = "\
18.1 - all the places patterns can be used;
18.2 - refutability: whether a pattern might fail to match;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _18_tests {
    use super::*;
    //-------------------------------------------------- 18.1 match expressions
    /*
    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }

    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
    */
    #[test]
    fn _1_simple_match_expr() {
        let x = Some(21);
        let y = match x {
            Some(number) => Some(number),
            None => None
        };
        assert_eq!(y, Some(21));
    }

    #[test]
    fn _1_conditional_if_let_expr() {
        let fav_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(_color) = fav_color { //fav_color is not Some
            println!("{fav_color:?}");
        } else if is_tuesday { //it is not tuesday
            println!("{is_tuesday}");
        } else if let Ok(age) = age {
            if age > 30 {  //age is iver 30
                println!("{age}"); //print age
                assert_eq!(age, 34);
            } else {
                println!("{age}");
            }
        } else {
            println!("else");
        }
    }

    #[test]
    fn _1_while_let_conditional_loop() {
        let mut stack = vec![];
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{top}");
        }
        assert_eq!(stack, vec![]);
    }

    #[test]
    fn _1_patterns_in_a_for_loop() {
        let v = vec!['a','b','c'];
        for (i, v) in v.iter().enumerate() {
            println!("{i}:{v}");
        }
        assert_eq!(v, vec!['a','b','c']);
    }

    #[test]
    fn _1_pattern_for_destructuring_tuples() {
        let (x, y, z) = (1, 2.0, '@');
        //number of params in pattern must match params in expression
        assert_eq!(x, 1);
        assert_eq!(y, 2.0);
        assert_eq!(z, '@');
    }

    #[test]
    fn _1_destructuring_a_functions_params() {
        fn print(&(x, y): &(i32, i32)) {
            assert_eq!(x, 10);
            assert_eq!(y, 10);
        }

        let tup = (10, 10);
        print(&tup);
    }

    //18.2 skip - irrefutable and refutability explained
    //-------------------------------------------------- 18.3 pattern syntax
    #[test]
    fn _3_matching_literals() {
        let x = 1;
        let y = match x {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => 0
        };
        assert_eq!(y, 1);
    }

    #[test]
    fn _3_matching_named_variables() {
        let x = Some(5);
        let _y = 10;
        let z = match x {
            Some(50) => 50,
            Some(y) => y,
            _ => 0
        };
        assert_eq!(z, 5);
    }

    #[test]
    fn _3_multi_patterning_syntax() {
        let x = 2;
        let y = match x {
            1 | 2 => 1,
            3 => 3,
            _ => 0
        };
        assert_eq!(y, 1);
    }

    #[test]
    fn _3_range_matching_syntax() {
        let x = 5;
        let y = match x {
            1..=5 => true,
            _ => false,
        };
        assert!(y);
    }

    #[test]
    fn _3_ranges_work_with_chars() {
        let x = 'x';
        let y = match x {
            'a'..='j' => true,
            _ => false,
        };
        assert!(!y);
    }

    #[test]
    fn _3_destructuring_an_entire_struct() {

        struct Point {
            x: i32,
            y: i32,
        }

        impl Point {
            fn new(x: i32, y: i32) -> Point {
                Point {
                    x,
                    y
                }
            }
        }

        let p = Point::new(10, 10);
        let Point { x, y} = p; //satisfy struct pattern with p

        assert_eq!(x, 10);
        assert_eq!(y, 10);
    }

    #[test]
    fn _3_matching_with_struct_patterns() {
        #[derive(Debug)]
        struct Point {
            _x: i32,
            _y: i32,
        }
        let p = Point { _x: 10, _y: 5 };
        match p {
            Point { _x, _y: 0} => assert_eq!(p._y, 0),
            Point { _x: 0, _y} => assert_eq!(p._x, 0),
            Point { _x, _y } => assert!(p._x != 0 && p._y != 0),
        };
        println!("{p:?}");
    }

    #[test]
    fn _3_destructuring_enums() {
        enum Message {
            // Quit,
            // Move { x: i32, y: i32 },
            // Write(String),
            ChangeColor(i32)
        }
        //let msg = Message::ChangeColor(0, 160, 255);
        let msg = Message::ChangeColor(0);
        match msg {
            // Message::Quit                                => assert!(true),
            // Message::Move { x, y }             => assert_eq!(x,x),
            // Message::Write(text)                  => assert_eq!(text,text),
            Message::ChangeColor(r) => assert_eq!(r,r),
        };
    }

    #[test]
    fn _3_destructuring_nested_enums() {
        enum Color {
            Hsv(i32, i32, i32),
        }

        enum Message {
            ChangeColor(Color)
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            //Message::ChangeColor(Color::Rgb(r, g, b)) => { println!("meow"); },
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                let s = format!("({h}, {s}, {v})");
                assert_eq!(s, "(0, 160, 255)".to_string());
            }
        };
    }

    #[test]
    fn _3_ignoring_params_with_underscores() {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => println!("can't overwrite"),
            _ => setting_value = new_setting_value,
        }

        assert_eq!(setting_value, Some(5));
    }

    #[test]
    fn _3_ignoring_params_with_match_again() {
        let numbers = (2, 4, 8, 16, 32);
        match numbers { (a, _, b, _, c) => {
            assert_eq!(format!("{a}{b}{c}"), "2832".to_string());
        } }
    }

    #[test]
    fn _3_ignore_remaining_with_dot_dot() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            _y: i32,
            _z: i32
        }
        let origin = Point {
            x: 0,
            _y: 0,
            _z: 0
        };
        match origin {
            Point{ x, .. } => assert_eq!(x, 0)
        }

        let numbers = (2, 4, 8, 16, 32);

        match numbers { (first, .., last) => { assert_eq!(first + last, 34) } }
        println!("{origin:?}");
    }

    #[test]
    fn _3_ignoring_an_unused_with_underscore() {
        let _x = 10; //<- compiler never complains this x is unused because of the _ prefix
        assert!(true);
    }

    #[test]
    fn _3_extra_conditionals_with_match_guards() {
        let num = Some(4);
        match num {
            Some(x) if x % 2 == 0 => assert_eq!(x % 2, 0), //<- even condition
            Some(x) => assert_eq!(x % 2, 1), //<- odd condition
            None => assert!(false)
        }
    }

    #[test]
    fn _3_making_more_assertions_with_match_guards() {
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => assert!(false), //<-- y is false, so match guard will not pass
            _ => assert!(true)
        }
    }

    #[test]
    fn _3_using_a_temporary_binding_with_ats() {
        enum Message {
            Hello { _id: i32 }
        }
        let msg = Message::Hello { _id: 5 };

        match msg {
            Message::Hello {
                _id: _id_variable @ 3..=7,
            } => assert!(true), //id was in the range of 3..=7 (5), assert!(true)
            Message::Hello { _id: 10..=12 } => assert!(false),
            Message::Hello { _id } => assert!(false)
        }
    }

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}