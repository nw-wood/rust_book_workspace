use rust_book_utilities;

const CHAPTER_NAME: &str    = "6.0 enums and pattern matching";
const CHAPTER_SUMMARY: &str = "\
6.1 - defining an enum;                     enum Case { a, b }
6.2 - the match control flow construct;     match c { a => f(), b => g() };
6.3 - concise control flow with if let;     if let Some(n) = o_n { a() };

this chapter got into the nutty grunt of some of the various usages of
both regular enums as well as the Option<T> type of enum
additionally, the control flow match statement, and it's if let comparison
";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _6_tests {
    use super::*;


    //--------------------------------------------- 6.1 defining an enum
    #[test]
    fn _1_using_an_enum_and_methods() {
        enum IpAddr {
            V4(String),
            V6(String)
        }
        impl IpAddr {
            fn dump_ip(&self) -> String {
                match self {
                    IpAddr::V4(s) => s.to_string(),
                    IpAddr::V6(s) => s.to_string()
                }
            }
        }
        let home = IpAddr::V4(String::from("127.0.0.1")).dump_ip();
        let loopback = IpAddr::V6(String::from("::1")).dump_ip();
        println!("home: {home}, loopback: {loopback}");
        assert_eq!(home, "127.0.0.1");
        assert_eq!(loopback, "::1");
    }

    #[test]
    fn _1_enums_with_varied_types_defined() {
        enum Message { //various things an enum can be
            Quit, //no associated data
            Move { x: i32, y: i32 }, //fields, struct-like
            Write(String), //a string like v4 and v6
            ChangeColor(i32, i32, i32) //a tuple
        }
        impl Message {
            fn process_type(&self) -> usize {
                match self {
                    Message::Quit => 1,
                    Message::Move{x, y} => {
                        println!("x: {x}, y:{y}");
                        2
                    },
                    Message::Write(s) => {
                        println!("Write(s): {s}");
                        3
                    }
                    Message::ChangeColor(r, g, b) => {
                        println!("r: {r}, g: {g}, b:{b}");
                        4
                    }
                }
            }
        }
        let a = Message::Quit;
        let b = Message::Move { x: 10, y: 10 };
        let c = Message::Write(String::from("hi"));
        let d = Message::ChangeColor(10, 10, 10);
        assert_eq!(a.process_type(), 1);
        assert_eq!(b.process_type(), 2);
        assert_eq!(c.process_type(), 3);
        assert_eq!(d.process_type(), 4);
    }


    #[test]
    fn _1_the_option_enum() {
        let some_number = Some(5);
        let some_char = Some(' ');
        let absent_number: Option<i32> = None;
        /*let x = match some_number { //verbose handling of Option<T>
            Some(n) => n,
            None => panic!("code for none result here")
        };*/
        //let n = some_number.unwrap_or_else(|| panic!("informative")); //more valid panic
        let n = some_number.unwrap(); //don't care? simple
        assert_eq!(n, 5);
        let c = some_char.unwrap_or_else(|| 'a'); //return a comparable value
        assert_eq!(c, ' ');
        let a = absent_number.unwrap_or_else(|| 0); //None would panic, so set to 0
        assert_eq!(a, 0);
    }


    //--------------------------------------------- 6.2 the match control flow construct
    #[test]
    fn _2_control_flow_with_match() { //I've used match in here for dissecting enums before ;)
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState) //enum contains enum
        }
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska
        }
        fn value_in_cents(c: Coin) -> u8 {
            match c {
                Coin::Penny => {
                    println!("pennies happen to be lucky");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(s) => {
                    println!("state: {s:?}");
                    25
                }
            }
        }
        let c = Coin::Quarter(UsState::Alabama);
        let pocket = [Coin::Nickel, Coin::Quarter(UsState::Alaska), Coin::Penny, Coin::Dime, c];
        let mut t = 0;
        let mut tc = 0;
        for e in pocket { t += value_in_cents(e); tc += 1 }
        assert_eq!(t, 66); //61u8
        assert_eq!(tc, 5);
    }


    #[test]
    fn _2_matching_with_option_t() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1)
            }
        }
        let n = Some(5);
        let n = plus_one(n);
        let none = plus_one(None);
        assert_eq!(n, Some(6));
        assert_eq!(none, None);
    }


    #[test]
    fn _2_catch_alls_and_placeholders() {
        fn run_a_function() {
            println!("okay, 3");
        }
        let roll = 20;
        match roll {
            20 => println!("neat, 20"),
            3 => run_a_function(),
            other => println!("okay, {other}")
        }
    }


    //--------------------------------------------- 6.3 if let control flow
    #[test]
    fn _3_concise_control_flow() {
        let o_n = Some(3u8);
        match o_n {
            Some(n) => println!("{n}"),
            _ => ()
        }
        assert_eq!(o_n, Some(3));
    }

    #[test]
    fn _3_less_concise_shorthand() {
        let o_n = Some(3u8);
        if let        Some(n) = o_n { println!("n: {n}"); }            //assumes _ => ()
        match   o_n { Some(n) =>    { println!("n: {n}")  } _ => () }; //match doesn't

        if let Some(n) = o_n { println!("n: {n}") };
        if /*Option::*/Some(3u8) == o_n { println!("o_n(?), ?: {}", 3u8) }; //even more specific
        assert_eq!(o_n, Some(3u8));
    }


    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}