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


    //--------------------------------------------- 5.2 example struct program
    #[test]
    fn _2_struct_program() {
        fn area(w: u32, h: u32) -> u32 {
            w * h
        }
        let w = 30;
        let h = 50;
        let a = area(w,h);
        assert_eq!(a, 30u32 * 50u32);
    }


    #[test]
    fn _2_struct_program_with_tuples() {
        fn area(wh: (u32, u32)) -> u32 {
            wh.0 * wh.1
        }
        let r = (30, 50);
        let a = area(r);
        assert_eq!(a, 30u32 * 50u32);
    }


    #[test]
    fn _2_factoring_in_structs_in_program_and_derive() {
        #[derive(Debug)]
        struct Rectangle {
            w: u32,
            h: u32
        }
        fn area(r: &Rectangle) -> u32 {
            r.w * r.h
        }
        let rect = Rectangle {
            w: 30,
            h: 50
        };
        let a = area(&rect); //takes a rectangle ref
        print!("{rect:?}");
        dbg!(&rect); //usage of debug macro for types that derive debug
        assert_eq!(a, 30u32 * 50u32);
    }


    //--------------------------------------------- 5.3 method syntax
    #[test]
    fn _3_rectangle_method_examples() {
        #[derive(Debug)]
        struct Rectangle {
            w: u32,
            h: u32
        }
        impl Rectangle {
            fn area(&self) -> u32 {
                self.w * self.h
            }
            fn non_zero_w(&self) -> bool {
                self.w > 0
            }
            fn can_hold(&self, r: &Rectangle) -> bool {
                self.w > r.w && self.h > r.h
            }
        }
        fn build_rect(w: u32, h: u32) -> Rectangle { Rectangle { w, h } }
        let rect1 = build_rect(30, 50);
        let rect2 = build_rect(20,40);
        assert_eq!(rect1.area(), 30u32*50u32);
        assert_eq!(rect1.non_zero_w(), true);
        assert_eq!(rect1.can_hold(&rect2), true);
    }


    #[test]
    fn _3_making_a_rectangle_with_associated_function() {
        struct Rect {
            w: u32,
            h: u32
        }
        impl Rect {
            fn new_square(w: u32) -> Self {
                Self {
                    w,
                    h: w //neat, field and not field init of w
                }
            }
        }
        let r = Rect::new_square(10);
        assert_eq!(r.h, 10);
        assert_eq!(r.w, 10);
    }


    #[test]
    fn _3_multiple_impl_blocks_and_mutable_ref_methods() {
        struct Rect {
            p: bool
        }
        impl Rect {
            fn new() -> Self {
                Self {
                    p: true
                }
            }
            /*fn set_p(tf: bool) -> Self { <--- nope
                Self {
                    p: tf
                }
            }*/
        }
        impl Rect {
            fn p_val(&self) -> bool {
                self.p
            }
            fn set_p(&mut self, tf: bool) { //  <---- yup
                self.p = tf
            }
        }
        let mut r = Rect::new();
        assert_eq!(r.p_val(), true);
        r.set_p(false);
        assert_eq!(r.p_val(), false);

    }


    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}