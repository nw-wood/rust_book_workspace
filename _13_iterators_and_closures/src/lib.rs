use rust_book_utilities;

const CHAPTER_NAME: &str    = "13.0 iterators and closures";
const CHAPTER_SUMMARY: &str = "\
13.1 - closures: anon functions that capture their environment;     black mesa functions
13.2 - processing series of items with iterators;
13.3 - improving the io project;
13.4 - comparing performance: loops vs iterators;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _13_tests {
    use std::fmt::Debug;
    use std::thread;
    use std::time::Duration;

    use super::*;

    //---------------------------------------------- 13.1 capturing the environment with closures
    /*
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) //<--- closure shown in example
    }*/

    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }   function def
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };  fully annotated closure
    // let add_one_v3 = |x|             { x + 1 };  without type annotations
    // let add_one_v4 = |x|               x + 1;    without brackets for single expressions

    //  let add_one_v4 = |x| x + 1; is enough to express the add_one_v1 function
    #[test]
    fn _1_example_closure() {
        let slowlycute = |n: u32| -> u32 {
            println!("slowlycuting");
            thread::sleep(Duration::from_secs(2));
            n
        };
        let x = slowlycute(10);
        assert_eq!(10, x);
        //slowlycute contains a function instead of just some arbitrary type
        //this is an unusual way of saying:
        //  fn slowlycute(n: u32) -> u32 {
        //      n
        //  }
    }

    #[test]
    fn _1_capturing_refs_or_moving_ownership() {
        let list = vec![1, 2, 3];
        let only_borrow = || println!("closure! {list:?}"); //<-- will only borrow here
        only_borrow();
        assert_eq!(list, vec![1,2,3]);
    }

    #[test]
    fn _1_borrow_mutably() {
        let mut list = vec![1, 2, 3]; //<-- mut
        let mut borrow = || list.push(7); //<-- borrow push's the number 7 to the tail of list
        borrow();
        assert_eq!(list, vec![1, 2, 3, 7]);
    }

    #[test]
    #[ignore]
    fn _1_demanding_ownership_with_move() {
        let list = vec![1, 2, 3];
        thread::spawn(move || println!("thread: {list:?}")).join().unwrap();
        //passed ownership of list to the thread with move || println!
        //even though list isn't mutable this way it's passed anyway
        //thread was joined, so, should be able to reference list again

        //assert_eq!(list, vec![1, 2, 3]); <--- value moved! can't compare!
    }
    /*
                impl<T> Option<T> {
                pub fn unwrap_or_else<F>(self, f: F) -> T
                where
                    F: FnOnce() -> T
                {
                    match self {
                        Some(x) => x,
                        None => f(),
                    }
                }
     */

    #[test]
    fn _1_using_sort_by_key() {
        #[derive(Debug, PartialEq)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 }, //<-- just leave trailing commas for muscle memories sake
        ];

        list.sort_by_key(|r| r.height); //<-- sorts in order of r.height

        let val = Rectangle { width: 7, height: 12 };
        assert_eq!(val, list[2]); //since rectangles have the partialeq the custom en method is used
    }

    #[test]
    fn _1_an_fnmut_as_opposed_to_fnonce() {
        #[derive(Debug, PartialEq)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        //noinspection DuplicatedCode
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 }, //<-- just leave trailing commas for muscle memories sake
        ];

        let mut sort_operations = 0;
        list.sort_by_key(|r| {
            sort_operations += 1;
            r.width
        });
        assert_eq!(list[0].height, 5);
        assert_eq!(list[0], Rectangle { width: 3, height: 5});
        assert_eq!(sort_operations, 6);

    }

    // -------------------------------------------- 13.2 Processing a Series of Items with Iterators

    #[test]
    fn _2_lazy_ol_iterators() {
        let v1 = vec![1, 1, 1];
        let v1_iter = v1.iter();
        for index_value in v1_iter {
            assert_eq!(*index_value, 1);
        }
    }

    #[test]
    fn _2_iterator_next_method_demo() {
        let v = vec![1, 2, 3];
        let mut v_iter = v.iter();
        assert_eq!(v_iter.next(), Some(&1));
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&3));
        assert_eq!(v_iter.next(), None);
    }

    #[test]
    fn _2_consuming_an_iterator_with_sum() {
        let v = vec![1, 2, 3];
        let v_iter = v.iter();
        let total: i32 = v_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn _2_creating_an_iter_from_an_iter_and_collecting() {
        let v1 = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn _2_capturing_the_env_with_a_closure() {
        #[derive(Debug, PartialEq)]
        struct Shoe {
            size: u32,
            style: String,
        }
        fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect() //consumes shoes
        }
        let shoes = vec![
            Shoe { size: 10, style: "sneaker".to_string(), },
            Shoe { size: 13, style: "sandal".to_string(), },
            Shoe { size: 10, style: "boot".to_string(), }
        ];
        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(in_my_size, vec![Shoe { size: 10, style: "sneaker".to_string() }, Shoe { size: 10, style: "boot".to_string() }])
    }
    //13.3 skip - goes back to I/O program and implements direct usage of env::args().collect();
    //I feel I should write this program out separately, so I have a better grasp of it
    //13.4 audio decoder example
    /*
    let buffer: &mut [i32];                         <--- an empty mutable i32 array reference
    let coefficients: [i64; 12];                    <--- an array of 12 i64's
    let qlp_shift: i16;                             <--- an empty i16

    for i in 12..buffer.len() {                     <--- for i in index 12 through length of buffer
        let prediction = coefficients.iter()                        <--- iter for array
                                     .zip(&buffer[i - 12..i])       <--- zip method, i32 array provided
                                     .map(|(&c, &s)| c * s as i64)  <--- product of tuple params as i64
                                     .sum::<i64>() >> qlp_shift;    <--- collect by sum::i64 shifted by i16
        let delta = buffer[i];                      <--- delta is an i32 ref from index i
        buffer[i] = prediction as i32 + delta;      <--- pred as i32 + the value from buffer[i]
    }
     */
    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}