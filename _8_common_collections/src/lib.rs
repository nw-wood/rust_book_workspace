use rust_book_utilities;

const CHAPTER_NAME: &str    = "8.0 common collections";
const CHAPTER_SUMMARY: &str = "\
8.1 - storing lists of values with vectors;         iterating over them and modifying them as well
8.2 - storing utf-8 encoded text with strings;
8.3 - storing keys with values in hash maps;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _8_tests {
    use super::*;
    //--------------------------------------------- 8.1 storing values in vectors
    #[test]
    fn _1_creating_and_storing_in_a_vec() {
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        assert_eq!(v[0], 1);
    }

    #[test]
    fn _1_reading_elements_of_a_vector() {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        println!("third: {third}");
        let third: Option<&i32> = v.get(2); //<--- uses get which returns an Option<T>
        match third {
            Some(third) => println!("third: {third}"),
            None => ()
        }
        assert_eq!(v[2], 3);
    }

    #[test]
    fn _1_iterating_over_a_vec() {
        let v = vec![100, 100, 100];
        for i in &v {
            assert_eq!(*i, 100);
        }
    }

    #[test]
    fn _1_mutating_while_iterating() {
        let mut v = vec![10, 10, 10];
        for i in &mut v {
            *i += 10;
        }
        assert_eq!(v[0], 20);
    }

    #[test]
    fn _1_storing_enums_in_vec() {
        enum En {
            Int(i32),
            Float(f64),
            Text(String)
        }
        let v = vec![
            En::Int(3),
            En::Float(23.0),
            En::Text("hi".to_string())
        ];
        for i in &v {
            match i {
                En::Int(n) => assert_eq!(*n, 3),
                En::Float(f) => assert_eq!(*f, 23.0),
                En::Text(s) => assert_eq!(*s, "hi".to_string())
            }
        }
    }

    //--------------------------------------------- 8.2 Strings
    #[test]
    fn _2_to_string_method() {
        let data = "hi";
        let data = data.to_string();
        let data2 = "hi".to_string(); //works fine too
        assert_eq!(data, "hi");
        assert_eq!(data2, "hi");
    }

    #[test]
    fn _2_mutating_a_string() {
        let mut s = "hi".to_string();
        s.push_str("ho");                 //<-- str mut
        assert_eq!(s, "hiho".to_string());
        s.push('h');                        //<-- 1 char mut
        assert_eq!(s, "hihoh".to_string());
    }

    #[test]
    fn _2_concatenation_with_string_refs() {
        let a = "1".to_string();
        let b = "2".to_string();
        let c = "3".to_string();
        let a = a + &b + &c;
        assert_eq!(a, "123".to_string());
    }

    #[test]
    fn _2_concatenation_with_format() {
        let a = "1".to_string();
        let b = "2".to_string();
        let c = "3".to_string();
        let d = format!("{a}-{b}-{c}!!");
        println!("{d}");
        assert_eq!(d, "1-2-3!!".to_string()); //format! is like println!, but returns a string
    }

    #[test]
    fn _2_indexing_into_strings() {
        /*let a = "hi".to_string();
        let a = &a[0];
        let a = a[0];
        println!("{a}"); won't work!*/
        let a = "hi".to_string();
        let a = a[..1].to_string(); //<-- string slices!
        println!("{a}");
        assert_eq!(a, "h".to_string());
    }

    #[test]
    fn _2_iter_chars_instead_of_bytes() {
        for c in "@@@@@@@@@@@@@@@".chars() { //returns iterator for string as chars
            assert_eq!(c, '@');
        }
        for c in "@@@".bytes() { //as bytes instead
            assert_eq!(c, 64);
        }
    }

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}