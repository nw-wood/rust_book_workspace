use rust_book_utilities;
//use crate::_7_tests::garden::vegetables::Asparagus;

const CHAPTER_NAME: &str    = "7.0 packages and crate modules";
const CHAPTER_SUMMARY: &str = "\
7.1 - packages and crates;
7.2 - defining modules to control scope and privacy;
7.3 - paths of referring to items on module tree;
7.4 - bringing paths into scope with the use keyword;
7.5 - separating modules into different files;";

pub fn run_summary() {
    rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
    /*let mut a = Asparagus { species: String::from("Batman") };
    a.species = String::from("Robin");
    drop(a); // <--- dropped after usage, compiler warning skip*/
}

#[cfg(test)]
#[path = "_7_tests"]
mod _7_tests { // <-------------------------------- 7.1 welcome to modules
    use super::*;


    //--------------------------------------------- 7.2 defining modules
    //#[test]
    use crate::_7_tests::garden::vegetables::Asparagus;
    pub mod garden;
    fn _2_usage_of_declared_submodule() {
        let plant = Asparagus { species: String::from("full auto") };
        assert_eq!(plant.species, String::from("full auto"));
    }


    #[test]
    fn _2_grouping_related_code_in_mods() {
        pub mod front_of_house {
            pub mod hosting {
                pub fn add_to_waitlist() -> bool { true } //<--- public func
                //fn seat_at_table() -> bool { false } //<--- implicitly private
            }
        }
        //mod serving {
            //fn take_order() {}
            //fn server_order() {}
            //fn take_payment() {}
        //}
        let a = front_of_house::hosting::add_to_waitlist(); //accessible from here
        //let b = front_of_house::hosting::seat_at_table(); <--- not a public function of hosting
        assert_eq!(a, true);
    }


    #[test]
    fn referring_to_declared_module_items() { //I've done this twice already
        assert_eq!(1,1);
    }

    //--------------------------------------------- 7.3 paths for referring to items
    fn _3_refer_to_an_item_in_the_module_tree() {
        //use crate::_7_tests::garden::vegetables::Asparagus; <---- bring path into scope
        //let plant = Asparagus { species: String::from("full auto") }; <--- usage of item in scope
        //again
        //pub mod front of house { pub mod hosting { pub fn add_to_waitlist() -> bool { true } } }
        //let a = front_of_house::hosting::add_to_waitlist();
        assert_eq!(1,1);
    }


    #[test]
    fn _3_exposing_paths_with_pub() {
        //I've done this multiple times above
        //pub mod front_of_house {
        //             pub mod hosting {
        //                 pub fn add_to_waitlist()
        //pub mod garden;
        //
        //as well as the items and modules contained within garden and vegetable.rs
        assert_eq!(1,1);
    }


    //--------------------------------------------- 7.4 the use keyword
    #[test]
    // skipping a large amount of fluff, new stuff in 7.4
    fn _4_use_as_another_name() {
        let a = String::from("beep");
        use std::string::String as Straaaang;
        let b = Straaaang::from("beep");
        assert_eq!(a, b); //String and Straaaang are interchangeable
    }


    #[test]
    fn _4_pub_use_example() {
        //pub use crate::_7_tests::garden::vegetables::Asparagus;
        //I believe if another file uses _7_tests module then it will have access to asparagus
        //writing this is conflicting with the above use
        assert_eq!(1,1);
    }

    /* some shorthand for the use statement -
    |
    |   use::std::io::{self, Write};    // <--- use std::io; + use std::io::Write
    |   use std::collection::*;         // <--- glob operator, use all cases
    |
     */

    //--------------------------------------------- 7.5 mods into different files
    //like in this file, file modules can be brought into scope
    //reminder: the first test in here, 7.2 is example

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}