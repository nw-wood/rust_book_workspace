use rust_book_utilities;

const CHAPTER_NAME: &str    = "5.0 using structs to structure related data";
const CHAPTER_SUMMARY: &str = "\
5.1 - defining and instantiating structs;
5.2 - an example program using structs;
5.3 - method syntax;

learned some stuff here - am sure of it";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _5_tests {
    use super::*;
    //--------------------------------------------- 5.1 defining and instantiating structs
    #[test]
    fn _1_first_struct() {
        struct User {
            active: bool
        }
        let user = User {
            active: true
        };
        assert_eq!(user.active, true);
    }
    #[test]
    fn _1_first_mutable_struct() {
        struct User {
            active: bool
        }
        let mut user = User {
            active: true
        };
        user.active = false;
        assert_eq!(user.active, false);
    }
    #[test]
    fn _1_build_struct_with_func() {
        struct User {
            active: bool
        }
        fn build(tf: bool) -> User {
            User {
                active: tf
            }
        }
        let built = build(true);
        assert_eq!(built.active, true);
    }
    #[test]
    fn _1_build_with_field_init() {
        struct User {
            active: bool
        }
        fn build(active: bool) -> User {
            User {
                active //param must match field name from struct
            }
        }
        let built = build(true);
        assert_eq!(built.active, true);
    }
    #[test]
    fn _1_struct_update_syntax() {
        struct User {
            active: bool
        }
        fn build(active: bool) -> User {
            User {
                active //param must match field name from struct
            }
        }
        let built = build(true);
        let update = User {
            ..built
        };
        assert_eq!(update.active, true);
    }
    #[test]
    fn _1_tuple_struct_example() {
        struct Color(i32, i32, i32);
        let black = Color(0,0,0);
        assert_eq!(black.0, 0);
        assert_eq!(black.1, 0);
        assert_eq!(black.2, 0);
    }
    /*fn unit_like_struct() {
        struct Blank;
        let a = Blank;
        assert_eq!(a, ());
    }*/
    #[test]
    fn _1_ownership_of_struct_data() {
        struct User<'a> {
            name: &'a str //lifetimes later, but name is held for life of User<'a>
        }
        let s = String::from("wood");
        let u = User {
            name: &s[..] //a reference to s moved into User
        };
        assert_eq!(u.name, "wood"); //we can refer to s in u by reference
        assert_eq!(s, "wood"); //s still has ownership of its own data
        //if u goes out of scope (User<'a>) then the &s[..] in name gets dumped too
    }
    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}