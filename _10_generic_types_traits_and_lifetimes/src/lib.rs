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
    fn _2_traits_and_how_to_use_them() {
        pub trait Summary {//<-- specifying the summary trait

            fn summarize(&self) -> String; //<-- if using Summary must handle signature

            fn summarize_harder(&self) -> String { //a default implementation
                format!("{}", self.summarize()) //kinda silly, works fine
            }
        }
        pub struct Article {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String
        }
        impl Summary for Article {
            fn summarize(&self) -> String { //<-- must implement this method to satisfy Summary
                println!("{}", self.content); //using up fields
                format!("{}{}{}", self.headline, self.author, self.location)
            }
            //note summarize_harder(&self) did not need to be specified, but we can
            /*fn summarize_harder(&self) -> String {
            }*/
        }
        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool
        }
        impl Summary for Tweet {
            //compiler insists we add the summarize trait
            fn summarize(&self) -> String {
                //the IDE implemented a
                println!("{}{}", self.reply, self.retweet); //using up fields
                format!("{}: {}", self.username, self.content)
            }
        }
        let article = Article {
            headline: "head-point".to_string(),
            location: "the bulkiness".to_string(),
            author: "wood".to_string(),
            content: "lorem ipsum".to_string()
        };
        let tweet = Tweet {
            username: "wood".to_string(),
            content: "c o n t e n t".to_string(),
            reply: true,
            retweet: true
        };

        let article_summary = article.summarize();
        let tweet_summary = tweet.summarize();
        let f = format!("{}{}{}", article.headline, article.author, article.location);
        assert_eq!(article_summary, f);
        let f =  format!("{}: {}", tweet.username, tweet.content);
        assert_eq!(f, tweet_summary);
        assert_eq!(f, tweet.summarize_harder());
    }

    #[test]
    fn _2_traits_as_parameters() {
        pub trait HelloTrait {
            fn hello(&self) -> String {
                "hello".to_string()
            }
        }
        /*pub fn notify(item: &impl HelloTrait) -> String {
            let s = item.hello();
            assert_eq!(s, "hello".to_string());
            item.hello()
        }*/
        struct Item {
        }

        impl HelloTrait for Item { }
        let i = Item {};
        let s = i.hello();
        assert_eq!(s, "hello".to_string())

        //notify(HelloTrait item?)... brain farting
    }

    //coming back later - need to focus on the material more

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}