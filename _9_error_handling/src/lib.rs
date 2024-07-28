use rust_book_utilities;

const CHAPTER_NAME: &str    = "9.0 error handling";
const CHAPTER_SUMMARY: &str = "\
9.1 - unrecoverable errors with panic;  unwinding a stack trace when panics happen
9.2 - recoverable errors with result;   handling errors all over the place
9.3 - to panic! or not to panic!;       why you panic is up to you, and the panic method";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _9_tests {
    use std::io::{Error, Read};
    use super::*;
    /*--------------------------------------------- 9.1 panic!("panic message"); no section required

                                                    set env var RUST_BACKTRACE to 1 for stack trace

    ----------------------------------------------- 9.2 recovering with results */

    /*enum Result<T, E> {
        Ok(T),
        Err(E)                  <-------- more important if not as important as Option<T> in Rust
    }
    */

    #[test]
    fn _2_failing_ungracefully() {
        use std::fs::File;
        //let r: Result<File, Err> = File::open("file");
        match File::open("file") {
            Ok(_f) => assert_eq!(1,1), // _f because not returning f in match
            Err(e) => panic!("e: {e:?}") //<--- will panic if it doesn't
        };
    }

    #[test]
    fn _2_catching_an_error() {
        use std::fs::File;
        use std::io::ErrorKind;

        //the compiler just tried to flex on me for not using 1 anonymous function
        //in a case where clearly 2 could be used

        /*let r = File::open("file");
        let f = match r {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match File::create("file") {
                    Ok(f) => f,
                    Err(e) => panic!("could not create: {e:?}")
                },
                e => {
                    panic!("unexpected error: {e}")
                }
            }
        };*/

        //it's like writing bad LSL on purpose I love this language
        //if you are reading this please don't write code this way - please

        let b = File::open("file").unwrap_or_else(|e| match e.kind() {
            ErrorKind::NotFound => File::create("file").unwrap_or_else(|e| panic!("could not create file: {e:?}")),
            _ => panic!("could not find or create file: {e:?}")
        }).read_to_string(&mut String::new()).unwrap_or_else(|e| panic!("couldn't read f: {e:?}"));
        assert_eq!(b, 0); //file should be empty :)

    }

    #[test]
    fn _2_using_expect() {
        use std::fs::File;
        use std::io::ErrorKind;

        //using expects actually helped me format this block into something more legible

        let b = File::open("file")
            .unwrap_or_else(|e| match e.kind() {
                ErrorKind::NotFound => File::create("file").expect("could not create file"),
                _ => panic!("could not find or create file: {e:?}")
            })
            .read_to_string(&mut String::new())
            .expect("could not read to string");

        assert_eq!(b, 0); //file should be empty :)
    }

    #[test]
    fn _2_error_propagation() {
        use std::fs::File;
        use std::io::{Read};
        fn read() -> Result<String, Error> {
            let mut f = File::open("file")?; // <- ?
            let mut s = String::new();
            f.read_to_string(&mut s)?; // <- ?
            Ok(s)
        }
        let s = read().expect("should have returned a string, but got err");
        assert_eq!(s, "".to_string());
    }

    #[test]
    fn _2_combined_knowledge_of_errors() {
        use std::fs::File;
        use std::io::{Read, ErrorKind};

        fn bytes_in_file(f: &str) -> Result<usize, Error> { //<-- explicit result type
            let b = File::open(f) //<-- runs a function that returns a result
                .unwrap_or_else(|e| match e.kind() { //<-- fn closure(e: Error) { ... }
                    ErrorKind::NotFound => File::create("file").expect("error creating"), //<-- panic via expect
                    _ => panic!("error opening and creating") //explicitly panicking
                })
                .read_to_string(&mut String::new())?; //<-- propagates error
            Ok(b) //<-- returns result
        }

        let bytes = bytes_in_file("file").unwrap(); //<-- unwrapped result
        assert_eq!(bytes, 0); //tada~
    }

    //----------------------------------------------- 9.3 to panic or not
    #[test]
    fn _3_panicking_method_call() {
        pub struct St {
            v: i32
        }

        impl St {
            fn new(v: i32) -> St {
                if v < 1 || v > 100 {
                    panic!("out of range 0..100");
                }
                St { v }
            }

            fn get_v(self) -> i32 {
                self.v
            }
        }

        let a = St::new(10);
        let a = a.get_v();
        assert_eq!(a, 10);

    }

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}