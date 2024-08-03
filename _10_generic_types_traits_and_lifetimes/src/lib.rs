use rust_book_utilities;

const CHAPTER_NAME: &str    = "10.0 generic types, traits, and lifetimes";
const CHAPTER_SUMMARY: &str = "\
10.1 - generic data types;
10.2 - traits: defined shared behavior;
10.3 - validating references with lifetimes;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _10_tests {

    use super::*;
    //----------------------------------------------- 10.0 setup for 10.1
    //10.0 starts this way
    /*
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");
    //------------- continues into function return
    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {result}");
     */
    //came this far to explain that the 'T' fn largest<T>(list: &[T]) -> &T { pertains to any type
    //documentation provides some code
    /*
    fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {   <--- it then explains about the PartialOrder trait
                largest = item;    <--- and the reader doesn't understand yet at all
            }                       <--- really not a big fan of how generics are introduced
        }
        largest
    }
     */

    //----------------------------------------------- 10.1 generic data types
    #[test]
    fn _1_generics_in_struct_definitions() {
        struct PointXY<T> {
            x: T,
            y: T //<--- generic type T assumed for x, and y on creation of struct
        }
        let i = PointXY { x: 5, y: 10 };    //<--- i32 provided
        let f = PointXY { x: 1.0, y: 4.0 }; //<--- f64 provided

        assert_eq!(i.x, 5);
        assert_eq!(f.y, 4.0);
    }

    #[test]
    fn _1_multiple_generics() { //<- I think T kinda goes for Type, but I prefer A, B, C
        struct PointXY<A, B> {
            x: A,
            y: B
        }
        let two_ints = PointXY { x: 10, y: 10 };
        assert_eq!(two_ints.x, 10);
        let two_float = PointXY { x: 1.1, y: 1.1 };
        assert_eq!(two_float.x, 1.1);
        let int_and_f = PointXY { x: 0.1, y: 3 }; //<-- I can choose any two types to satisfy a PointXY
        assert_eq!(int_and_f.x, 0.1);
        assert_eq!(int_and_f.y, 3);
    }

    #[test]
    fn _1_generics_in_method_definitions() {
        struct Point<A> {
            x: A,
            y: A
        }
        impl<A> Point<A> {
            fn x(&self) -> &A {
                &self.x
            }
        }
        let p = Point { x: 10, y: 10 };
        assert_eq!(*p.x(), 10);
        assert_eq!(p.y, 10);
    }

    #[test]
    fn _1_constraining_generics_with_impls() {
        struct Point<T> {
            x: T,
            y: T
        }
        impl Point<f32> { //<--- we specify Point where f32s have replaced the generic
            fn x(&self) -> f32 {
                self.x
            }
        }
        let point_a = Point { x: 1, y: 2 }; //i32 version of a point
        let point_b = Point { x: 1.1f32, y: 2.2f32 }; //f32's provided

        //let f = point_a.x(); <-- method doesn't exist for point_a
        let f = point_b.x();
        assert_eq!(point_a.x, 1);
        assert_eq!(point_a.y, 2);
        assert_eq!(f, 1.1);
        assert_eq!(point_b.y, 2.2);
    }

    #[test]
    fn _1_mixing_generics_with_a_method() {
        struct Point<X1, Y1> {
            x: X1,
            y: Y1
        }
        impl<X1, Y1> Point<X1, Y1> {
            fn mix<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y //returns Point { x: X1, y: Y2 }, but is still a Point<T, U>, really
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "hi", y: '@' };
        let p3 = p1.mix(p2);
        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '@');
    }

    //----------------------------------------------- 10.2 defining shared behavior with traits

    #[test]
    fn _2_defining_and_implementing_a_trait() {
        pub trait Summary { //define trait, pub or private
            fn headline(&self) -> String; //fn signatures that must be implemented
        }

        pub struct Article { //an arbitrary struct
            pub headline: String
        }

        impl Summary for Article { //implement the trait on the struct
            fn headline(&self) -> String {
                format!("headline: {}", self.headline)
            }
        }
        let s = Article {
            headline: "meow".to_string()
        };
        let s = s.headline(); //can use the headline method now
        assert_eq!(s, "headline: meow".to_string());
    }

    #[test]
    fn _2_using_default_implementations() {
        pub trait Summary {
            fn default_text(&self) -> String {
                "default text...".to_string() //default implementation
            }
        }
        pub struct Article { } //declare empty Article Struct
        impl Summary for Article {} //declare summary trait on article
        let s = Article { }; //define an article
        let s = s.default_text(); //run the default method from the trait implementation
        assert_eq!(s, "default text...".to_string());
    }

    #[test]
    fn _2_overriding_a_default_trait_method() {
        pub trait ExampleTrait {
            fn example_method(&self) -> usize {
                10 //default method will return 10
            }
        }
        pub struct ExampleStruct { } //no data is fine
        impl ExampleTrait for ExampleStruct {
            fn example_method(&self) -> usize { //declared on implementation overrides default
                20
            }
        }
        let example = ExampleStruct {};
        assert_eq!(example.example_method(), 20);
    }

    #[test]
    fn _2_using_traits_as_function_parameters() {
        pub trait Summary {
            fn return_string(&self) -> String {
                "a string".to_string()
            }
        }
        pub struct Article {
            some_string: String
        }
        impl Summary for Article {
            fn return_string(&self) -> String {
                format!("{}", self.some_string)
            }
        }
        fn requires_trait_param(item: &impl Summary) -> String {
            item.return_string()
        }
        let a = Article { //is an article, and implements the summary trait
            some_string: "hello".to_string()
        };

        let s = requires_trait_param(&a);
        assert_eq!(s, "hello".to_string());
    }

    #[test]
    fn _2_using_trait_bound_syntax() {
        pub trait Summary { fn return_string(&self) -> String { "summary".to_string() } }
        pub struct Article { some_string: String }
        impl Summary for Article { fn return_string(&self) -> String { self.some_string.clone() } }

        //important bit - these lines mean the same thing vvv
        //this is called trait bound syntax, and lets you be more specific

        //fn requires_trait_param(item: &impl Summary) -> String { item.return_string() }
        fn requires_trait_param<T: Summary>(item: &T) -> String {
            item.return_string()
        }

        /*case:

        pub fn notify(item1: &impl Summary, item2: &imply Summary {     <--- fine, but won't enforce type
        pub fn notify<T: Summary>(item1: &T, item2: &T) {               <--- restricts to same type

        */

        //there are more ways to express these relationships ---

        /*case:

        pub fn notify(item: &(impl Summary + Display)) {                <--- fat
        pub fn notify<T: Summary + Display>(item: &T) {                 <--- trait bound ver

        */

        //using a where clause to help with legibility

        /*case:

        fn some_f<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {  <--- fatter

        fn some_f<T, U>(t: &T, u: &U) -> i32                                    <--- fattest
        where
            T: Display + Clone,
            U: Clone + Debug
        {

        */

        let a = Article { some_string: "hello".to_string() };
        let s = requires_trait_param(&a);
        assert_eq!(s, "hello".to_string());
    }

    #[test]
    fn return_a_traited_type() {
        trait Trait {
            fn get_s(&self) -> String;
        }
        struct Struct { s: String }
        impl Trait for Struct {
            fn get_s(&self) -> String {
                self.s.clone()
            }
        }
        fn returns_traited(item: &impl Trait) -> impl Trait {
            Struct {
                s: item.get_s()
            }
            //this won't work if returning multiple types
        }

        let s = Struct {
            s: "hello".to_string()
        };
        let str = returns_traited(&s); //incredible
        assert_eq!(str.get_s(), "hello".to_string());

    }

    #[test]
    fn conditional_method_implementation() {
        use std::fmt::Display;
        struct Pair<T> {
            x: T,
            y: T
        }
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }
        impl<T: Display + PartialOrd> Pair<T> { //generic implementation for Pairs with both traits
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("x: {}", self.x);
                } else {
                    println!("y: {}", self.y);
                }
            }
        }
        let p = Pair::new(10, 20);
        p.cmp_display();
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
        //can also conditionally implement a trait for any type that implements another trait
        //impl<T: Display> ToString for T {
        //T must have the Display trait to acquire ToString
    }

    //10.2 was a bit more complicated than the rest, but here it is

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}