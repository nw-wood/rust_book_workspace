use rust_book_utilities;

const CHAPTER_NAME: &str    = "17.0 object oriented functionality in rust";
const CHAPTER_SUMMARY: &str = "\
17.1 - characteristics of object oriented languages;
17.2 - using trait objects that allow for values of different types;
17.3 - implementing an object-oriented design pattern;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _17_tests {
    use super::*;
    //17.1 skip, explaining pubs usage again
    //--------------------------------------------------- 17.2 traits that allow for vals of dif types
    /*#[test]
    fn a_trait_with_common_behavior() {
        pub trait Draw {
            fn draw(&self); //<-- requires the draw method signature be implemented for types
        }
        pub struct Screen {
            pub components: Vec<Box<dyn Draw>>, //a vec draw traited types
        }
        impl Screen {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw(); //run the draw method on the contained items
                }
            }
        }
    }*/

    #[test]
    fn _2_common_behavior() {
        pub trait Draw {
            fn draw(&self);
        }

        pub struct Screen<T: Draw> {
            pub components: Vec<T>
        }

        impl<T> Screen<T>
        where
            T: Draw,
        {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
        pub struct Button {}

        impl Draw for Button {
            fn draw(&self) {
                println!("button");
            }
        }

        pub struct SelectBox {}

        impl Draw for SelectBox {
            fn draw(&self) {
                println!("select box");
            }
        }

        let screen = Screen {
            components: vec![
                SelectBox {},
                SelectBox {},
            ],
        }; //example is bunkis
        screen.run();
        assert_eq!(1,1);
    }

    //----------------------------------------------------------- 17.3 OOP design pattern
    #[test]

    fn oop_design_pattern() {
        pub struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }

        impl Post {
            pub fn new() -> Post {
                Post {
                    state: Some(Box::new(Draft {})),
                    content: String::new(),
                }
            }

            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }

            pub fn content(&self) -> &str {
                ""
            }

            pub fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }

            pub fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                }
            }
        }

        trait State {
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            /*fn content<'a>(&self, post: &'a Post) -> &'a str {
                ""
            }*/
        }

        struct Draft {}

        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview {})
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }

        struct PendingReview {}

        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                Box::new(Published {})
            }
        }

        struct Published {}

        impl State for Published {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }

            /*fn content<'a>(&self, post: &'a Post) -> &'a str {
                &post.content
            }*/
        }

        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

    }

    #[test]
    fn show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}