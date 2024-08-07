use rust_book_utilities;

const CHAPTER_NAME: &str    = "16.0 fearless concurrency";
const CHAPTER_SUMMARY: &str = "\
16.1 - using threads to run code simultaneously;
16.2 - using message passing to transfer data between threads;
16.3 - shared-state concurrency;
16.4 - extensible concurrency with the sync and send traits;";

pub fn run_summary() { rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY); }

#[cfg(test)]
mod _16_tests {
    use std::thread;
    use super::*;
    //-------------------------------------------------- 16.1 using threads for concurrent code exec
    #[test]
    fn _1_spawning_a_thread() {
        use std::thread;
        use std::time::Duration;

        thread::spawn(|| {
            for i in 1..10 {
                println!("{i}");
                thread::sleep(Duration::from_millis(1));
            }
        });
        assert_eq!(1,1); //<-- will finish before spawned thread and kill it
    }

    #[test]
    fn _1_waiting_for_a_thread() {
        use::std::thread;
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("{i}");
            }
        });
        handle.join().unwrap();
        assert_eq!(1,1); //<--- thread is going to finish before assert_eq because of join()
    }

    #[test]
    fn _1_moving_values_into_threads() {
        let v = vec![1,2,3];
        let handle = thread::spawn(move || {
            println!("{v:?}");
        });
        handle.join().unwrap();
    }

    //-------------------------------------------------- 16.2 passing between threads
    #[test]
    fn _2_using_channels_with_threads() {
        use std::sync::mpsc;
        use std::thread;

        let (sender, receiver) = mpsc::channel();

        let handle = thread::spawn(move || {
            let val = "hi".to_string();
            sender.send(val).unwrap(); //<-- send method gets processed on Sender with val
            //println!("{}", val); <-- attempt to use val after move to send method
        });

        handle.join().unwrap(); //<-- wait for thread to finish first!
        assert_eq!(receiver.recv().unwrap(), "hi".to_string()); //<-- Receiver holds "hi" now
    }

    #[test]
    fn _2_sending_multiple_values_down_a_channel() {
        use std::sync::mpsc;
        use std::thread;
        let (s, r) = mpsc::channel();
        let h = thread::spawn(move || {
            let v = vec![1,2,3,4];
            for i in v { s.send(i).unwrap(); }
        });

        h.join().unwrap(); //<-- thread finished what it was doing - process for r
        for received in r {
            assert!(received <= 4 && received >= 1);
        }
    }

    #[test]
    fn _2_cloning_the_transmitter() {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();

        let mut handles_list = vec![];
        handles_list.push(
            thread::spawn(move || {
                let values = vec![
                    "hi".to_string(),
                    "from".to_string(),
                    "the".to_string(),
                    "thread".to_string()
                ];
                for s in values {
                    tx.send(s).unwrap();
                }
            }));

        handles_list.push(
            thread::spawn(move || {
                let values = vec![
                    String::from("more"),
                    String::from("messages"),
                ];

                for s in values {
                    tx1.send(s).unwrap(); //<-- tx1, the clone
                }
            }));

        for handle in handles_list {
            handle.join().unwrap();
        }

        for s in rx {
            println!("{s}");
        }
        assert_eq!(1, 1);
    }

    //-------------------------------------------------- 16.3 sharing a mutex<t>
    #[test]
    fn _3_using_a_mutex() {
        use std::sync::Mutex;
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        assert_eq!(*m.lock().unwrap(), 6); //<- got it c:

        //derefing the lock result, not the mutex itself, is what is important
    }
    #[test]
    fn _3_sharing_a_mutex_with_arc() {
        use std::sync::{Arc, Mutex};
        use std::thread;
        //think of an arc as a Rc, but for threads*
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..10 {
            let counter = Arc::clone(&counter); //<-- arc clone
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap(); //<-- clone sent in
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(10, *counter.lock().unwrap()); //deref the lock result of mutex in arc
    }

    #[test]
    fn _0_show_summary() {
        rust_book_utilities::chapter_summary(CHAPTER_NAME, CHAPTER_SUMMARY);
        assert_eq!(1,1)
    }
}