use rust_book_utilities;

const CHAPTER_NAME: &str    = "19.0 advanced features";
const CHAPTER_SUMMARY: &str = "\
19.1 - unsafe rust;
19.2 - advanced traits;
19.3 - advanced types;
19.4 - advanced functions and closures;
19.5 - macros;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _19_tests {
    use std::fmt::{Display, Formatter};

    use super::*;

    //---------------------------------------------------------------------- 19.1 unsafe rust
    #[test]
    fn _1_declaring_a_raw_pointer() {
        let mut num = 5;
        let r1 = &num as *const i32; //immutable
        let r2 = &mut num as *mut i32;
        unsafe {
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        }
    }

    #[test]
    fn _1_dangerous_function_ex() {
        unsafe fn dangerous() { assert!(true); } //<- spooky unsafe function!
        unsafe {
            dangerous();
        }
    }

    #[test]
    fn _1_safe_abstraction_over_unsafe_code() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = r.split_at_mut(3); //<- does unsafe things potentially
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);

        /*fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = values.len();
            let ptr = values.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }*/
    }

    /*use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) }; //undefined behavior*/

/*    #[test]
    fn using_the_extern_keyword() {
        extern "C" {
            fn abs(input: i32) -> i32;
        }
        unsafe {
        println!("{}", abs(-3));
        }
    }*/

/*    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }*/

    #[test]
    fn _1_hello_static() {
        static HELLO_WORLD: &str = "hello world"; //global variable declarations are 'statics'
        assert_eq!(HELLO_WORLD, "hello world");
    }

    #[test]
    fn _1_mutable_static_example() {

        static mut COUNTER: u32 = 0;

        fn add_to_counter(number: u32) {
            unsafe {
                COUNTER += number; //required an unsafe block to add to the COUNTER
            }
        }

        add_to_counter(3);

        unsafe {
            assert_eq!(COUNTER, 3); //required again to assert equal to 3
        }
    }

    //---------------------------------------------------------------------- 19.2 traits extreme
    #[test]
    fn _2_fully_qualified_syntax() {
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly (&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                assert!(true);
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                assert!(true);
            }
        }

        impl Human {
            fn fly(&self) {
                assert!(true);
            }
        }

        let person = Human;
        Pilot::fly(&person); //<- Pilot's impl when provided a reference to a Human
        Wizard::fly(&person); //<- Wizard's impl
        person.fly(); //<- method impl
    }

    #[test]
    fn _2_disambiguating_by_trait() {

        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                "Spot".to_string()
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                "Puppy".to_string()
            }
        }

        assert_eq!(Dog::baby_name(), "Spot".to_string());
        //assert_eq!(Animal::baby_name(), "Spot".to_string());
        assert_eq!(<Dog as Animal>::baby_name(), "Puppy".to_string());

        //fully qualified syntax:
        
        //      <Type as Trait>::function(receiver_if_method, next_arg, ...);
    }

/*    #[test]
    fn _2_using_a_super_trait() {
        trait OutlinePrint: Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {output} *");
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
                assert!(true);
            }
        }
    }*/

    #[test]
    fn _2_trait_constraint_example_with_super() {
        use std::fmt;
        trait Out: Display { //fmt::Display trait required for Out trait's as well
            fn out_print(&self) { }
        }
        struct Point { x: i32, y: i32 }
        impl Display for Point {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
        impl Out for Point {
            fn out_print(&self) {
                println!("out_print");
            }
        }
        let p = Point { x: 10, y: 10 };
        p.out_print();
        println!("{p}");
        assert!(true)//yeah!
    }

    #[test]
    fn _2_using_tuple_structs_to_wrap_for_traits() {
        use std::fmt;

        struct Wrapper(Vec<String>); //<- tuple struct 'Wrapper' containing a string vec

        impl Display for Wrapper { //<- trait applied to wrapper, so now the vec essentially has a trait
            // in this way external traits can be applied to external types
            // in the book it's called the 'Newtype Pattern'
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let x = Wrapper(vec!["hi".to_string()]);
        println!("{x}");
        assert!(true);//yeah!
    }

    //---------------------------------------------------------------------- 19.3 advanced types
    #[test]
    fn _3_using_a_type_alias() {
        type Km = u8;
        let x: Km = 10;
        assert_eq!(x, 10u8);
        //let y = 10km; //will not work

        //more useful ex:
        //  type Frunk = Box<dyn Fn() + Send + 'static>; //<- type of anon function as a 'Frunk'
    }

    //fn bar -> ! { } //<- function that returns 'never'

    //---------------------------------------------------------------------- 19.4 adv Fn's

    #[test]
    fn _4_providing_a_function_to_another() {
        fn a(x: usize) -> usize {
            x + 1
        }
        fn b(f: fn(usize) -> usize, ar: usize) -> usize {
            f(ar) + f(ar)
        }
        let an = b(a, 5);
        assert_eq!(an, 12);
    }

    #[test]
    fn _4_iterating_a_vector_into_another() {
        let list = vec![1 /*, 2, 3, 4, 5*/];
        let list: Vec<String> = list.iter().map(|i| i.to_string()).collect();
        assert_eq!(list[0], "1".to_string());
    }

    #[test]
    fn _4_alternative_func_for_map() {
        let list = vec![1];
        let list: Vec<String> = list.iter().map(ToString::to_string).collect();
        assert_eq!(list[0], "1".to_string());
        //to_string got called on each element by map
    }

    #[test]
    fn _4_lesser_wizard_summoning() {

        #[derive(Debug)]
        enum Data {
            Value(usize)
        }

        let list: Vec<Data> = (0usize..100).map(Data::Value).collect();

/*        match list {
            Data::Value(100) => assert!(true),
            _ => ()
        } //needs to be in a loop or collected out to another type*/

        for i in list { match i { Data::Value(100) => assert!(true), _ => () } }

        //0-100 Data enums in a Vec with Value set in order
        //this can be expressed without an explicit for loop
    }

    #[test]
    fn _4_closure_return() {
        fn returns_closure() -> Box<dyn Fn(usize) -> usize> {
            Box::new(|x| x + 1usize) //<-function stuck in a box and tossed on the heap
        }
        let _x  = returns_closure();
        assert!(true);
    }

    //19.5 procedural macros I'll come back for another time - the basics are understood

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}