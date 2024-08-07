use rust_book_utilities;

const CHAPTER_NAME: &str    = "15.0 smart pointers";
const CHAPTER_SUMMARY: &str = "\
15.1 - using Box<T> to point at heap data;
15.2 - treating smart pointers like references with the deref trait;
15.3 - running code on cleanup with the drop trait;
15.4 - Rc<t>, the reference counted smart pointer;
15.5 - RefCell<T> and the interior mutability pattern;
";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _15_tests {
    use super::*;

    // ----------------------------------------------- 15.1 smart pointers: Box<t>
    #[test]
    fn _1_using_a_box_t() {
        let b = Box::new(5); //<-- a box is a container type for data on the heap
        println!("b = {b}");
    }

    #[test]
    fn _1_using_boxes_for_recursion() {
        use rust_list::List::{Cons, Nil};
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        assert_eq!(list, Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    }

    // ----------------------------------------------- 15.2 treating smart pointers like derefs
    #[test]
    fn _2_using_a_box_like_a_ref() {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(*y, x);
    }

    #[test]
    fn _2_implementing_the_deref_trait() {
        struct MyBox<T>(T); //<--- single index tuple struct
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        use std::ops::Deref;
        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0 //<--- returned value when dereferenced
            }
        }
        //can deref the user defined box now
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(x, *y);
    }

    #[test]
    fn _2_deref_coercion_in_action() {
        struct MyBox<T>(T); //<--- single index tuple struct
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        use std::ops::Deref;
        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0 //<--- returned value when dereferenced
            }
        }

        fn hello(name: &str) { //<--- Strings dereference to an &str
            println!("hello, {name}");
        }

        let m = MyBox::new("Rust".to_string());
        hello(&m);                  //<- my box derefs to a string, then the string derefs to a str due to coercion
        hello(&(*m)[..]);   //equivalent without coercion
    }

    // ----------------------------------------------- 15.3 cleaning up with the drop trait
    #[test]
    fn _3_implementing_the_drop_trait() {
        struct CustomPointer {
            data: String,
        }
        impl Drop for CustomPointer { //<-- implementation for the drop method follows
            fn drop(&mut self) { println!("drop custom pointer with data: {}", self.data) }
        }
        let c = CustomPointer {
            data: "stuff".to_string(),
        };
        let d = CustomPointer {
            data: "other".to_string(),
        };
        println!("pointers created");
        assert_eq!(c.data, "stuff".to_string());
        assert_eq!(d.data, "other".to_string());
    }

    #[test]
    fn _3_dropping_values_early() {
        use std::mem::drop;
        struct SomeStruct {
            data: String,
        }
        impl Drop for SomeStruct {
            fn drop(&mut self) {
                println!("lahiem: {}", self.data);
            }
        }
        let var = SomeStruct {
            data: "hoy minoy".to_string(),
        };
        assert_eq!(var.data, "hoy minoy".to_string());
        drop(var);
    }

    // 15.4 skipped Cons(T, Nil) is kinda bunks
    //--------------------------------------------------------- 15.5 RefCell<T>, Rc<T>
    #[test]
    fn _5_refcell_and_testing_with_a_mock_object() {
        pub trait Messenger {
            fn send(&self, msg: &str);
        }

        pub struct LimitTracker<'a, T: Messenger> {
            messenger: &'a T,
            value: usize,
            max: usize,
        }

        impl<'a, T> LimitTracker<'a, T>
        where
            T: Messenger,
        {
            pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
                LimitTracker {
                    messenger,
                    value: 0,
                    max,
                }

            }
            pub fn set_value(&mut self, value: usize) {
                self.value = value;
                let     percentage_of_max= self.value as f64 / self.max as f64;
                if      percentage_of_max >= 1.0    { self.messenger.send("err: over quote"); }
                else if percentage_of_max >= 0.9    { self.messenger.send("warn: over 90&") }
                else if percentage_of_max >= 0.75   { self.messenger.send("warn: 0ver 75%") };

            }
        } //after first figure, the next two together

        use std::cell::RefCell;

        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>, //<-- vec contained in a refcell
        }
        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]), //<-- new refcell method
                }
            }
        }
        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                println!("MockMessenger: {}", message);
                self.sent_messages.borrow_mut().push(message.to_string()); //<-- borrow_mut() on it
            }
        }
        fn it_sends_an_over_75_warning() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
            limit_tracker.set_value(80);
            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); //<-- borrow() here, immutable
        }

        it_sends_an_over_75_warning();
    }

    //--------------------------------------------------------- 15.6 Reference cycles can leak mem
    #[test]
    fn _6_avoiding_a_cycle_with_weak() {

        use std::cell::RefCell;
        use std::rc::{Weak, Rc};


        #[derive(Debug)]
        struct Node {
            parent: RefCell<Weak<Node>>,
        }

        let leaf = Rc::new(Node {
            parent: RefCell::new(Weak::new()),
        });
        assert_eq!(Rc::strong_count(&leaf), 1);
        assert_eq!(Rc::weak_count(&leaf), 0);

        { // ' start
            let branch = Rc::new(Node {
                parent: RefCell::new(Weak::new()),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            assert_eq!(Rc::strong_count(&branch), 1);
            assert_eq!(Rc::weak_count(&branch), 1);

            assert_eq!(Rc::strong_count(&leaf), 1);
            assert_eq!(Rc::weak_count(&leaf), 0);
        } // ' end

        assert_eq!(Rc::strong_count(&leaf), 1);
        assert_eq!(Rc::weak_count(&leaf), 0);
    }

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}