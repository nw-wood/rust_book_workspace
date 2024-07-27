use rust_book_utilities;

const CHAPTER_NAME: &str    = "8.0 common collections";
const CHAPTER_SUMMARY: &str = "\
8.1 - storing lists of values with vectors;         iterating over them and modifying them as well
8.2 - storing utf-8 encoded text with strings;      playing with strings and slices again
8.3 - storing keys with values in hash maps;        hashmaps, the k, v pairs of rust";

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

    //--------------------------------------------- 8.3 Hash Maps
    #[test]
    fn _3_creating_accessing_hashmaps() { //key value pair system
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("blue".to_string(), 10);
        let k = "blue".to_string();
        let v = map.get(&k).copied().unwrap_or(0); //would return &i32, but ref copied
        assert_eq!(v, 10);
    }

    #[test]
    fn _3_hashmaps_with_ownership() {
        use std::collections::HashMap;
        let k = "favorite color".to_string();
        let v = "blue".to_string();
        let mut map = HashMap::new();
        //map.insert(k, v); will take ownership
        map.insert(&k, &v);  //cool
        assert_eq!(k, "favorite color".to_string());
    }

    #[test]
    fn _3_hashmaps_iteration() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(1, 1);
        map.insert(2, 2);
        map.insert(3, 3);
        for (k, v) in &map { assert_eq!(k, v); }
    }

    #[test]
    fn _3_insert_or_and_hashmaps() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("a",1);
        map.entry("a").or_insert(1); //None if no "a" key, so, .or_insert
        //map.entry("a").unwrap(); would panic
        let result = map.get("a").unwrap();
        assert_eq!(*result, 1);
        println!("cool");
    }

    #[test]
    fn _3_updating_a_hashmap_value() {
        use std::collections::HashMap;
        let s = "hi ho he ha hu hy";
        let mut map = HashMap::new();
        for w in s.split_whitespace() {
            map.entry(w).or_insert(0);
        }
        for (k, v) in map { //way better
            println!("k: {k}, v: {v}");
            assert_eq!(v, 0); //all v should be 0
        }
        /*assert_eq!(*map.get("hi").unwrap(), 0); //getting better with dereferencing
        assert_eq!(*map.get("ho").unwrap(), 0);
        assert_eq!(*map.get("he").unwrap(), 0);
        assert_eq!(*map.get("ha").unwrap(), 0);
        assert_eq!(*map.get("hu").unwrap(), 0);
        assert_eq!(*map.get("hy").unwrap(), 0);*/
    }

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }


}