use std::sync;
use std::thread;
use std::time::Duration;

pub fn synchronization_test() {
    simple_fork_join_test();
    arc_test();

    fn simple_fork_join_test() {
        let msg = "Hello World";
        let join_handle = thread::spawn(move || {
            println!("Inside the thread..");
            let duration = Duration::from_millis(500);
            thread::sleep(duration);
            println!("Msg: {}", msg);
            "Hello World"
        });
        let x = join_handle.join().unwrap();
        println!("Received from thread: {:?}", x);
    }

    fn arc_test() {
        let numbers: Vec<i32> = (1..=10).collect();
        let numbers = vec!["One", "Two", "Three", "Four", "Five"];
        let arc = sync::Arc::new(numbers);

        simple_vector_test(arc);

        fn simple_vector_test(arc: sync::Arc<Vec<&'static str>>) {
            let mut join_handle_collection = Vec::new();

            for n in 0..arc.len() {
                let cl = arc.clone();
                let element = cl[n];
                join_handle_collection.push(thread::spawn(move || {
                    println!("Element: {:?}, Shared data: {:?}", element, cl);
                    thread::sleep(std::time::Duration::from_millis(200));
                }));
            }

            for handle in join_handle_collection {
                let result = handle.join().unwrap();
                println!("Received from thread: {:?}", result);
            }
        }
    }
}
