use std::{sync::mpsc::*, thread, time::Duration};

use std::fs;
use std::{io, path};

use std::sync::*;
use std::sync::{Arc, Condvar};

pub fn channel_test_drop() {
    sender_dropped_and_receiver_response_test();
    receiver_dropped_and_sender_response_test();

    multiple_sender_single_receiver_test();

    cond_var_test();

    fn cond_var_test() {
        println!("Condition variable test..");
        let started = false;
        let tuple = (Condvar::new(), Mutex::new(started));

        let arc_condition = Arc::new(tuple);
        let arc_condition_clone = arc_condition.clone();

        thread::spawn(move || {
            let max_loop_count = 5;
            for i in 0..max_loop_count {
                println!("Loop Count: {}", i);
                thread::sleep(Duration::from_millis(500));
                let mut result = arc_condition_clone.1.lock().unwrap();

                // set condition variable as true
                if i == max_loop_count - 1 {
                    *result = true;
                }

                // notify waiting thread
                arc_condition_clone.0.notify_one();
            }
        });

        let mut guard = arc_condition.1.lock().unwrap();

        while !*guard {
            println!("Condition not satisfied..");
            let pair = arc_condition
                .0
                .wait_timeout(guard, Duration::from_millis(200))
                .unwrap();
            guard = pair.0;

            let wait_timeout_result = pair.1;
            if wait_timeout_result.timed_out() {
                println!("Timeouted out waiting for condition..");
            } else {
                println!("Received: {}", *guard);
            }
        }
    }

    fn multiple_sender_single_receiver_test() {
        let mut sender_threads = Vec::new();

        let (sender, receiver) = channel::<u32>();

        for i in 0..20 {
            let clone = sender.clone();
            sender_threads.push(thread::spawn(move || {
                println!("Sending : {}", i);
                if clone.send(i).is_err() {
                    println!("Error occured while sending");
                };
                thread::sleep(Duration::from_millis(500));
            }));
        }

        // drop extra sender to unblock receiving data
        drop(sender);

        let mut receiver_threads = Vec::new();

        let data = std::sync::Mutex::new(receiver);
        let arc = std::sync::Arc::new(data);

        for i in 0..4 {
            let arc_mutex_receiver_clone = arc.clone();

            receiver_threads.push(thread::spawn(move || loop {
                println!("Waiting to receive..");
                if let Ok(receiver_mutex_guard) = arc_mutex_receiver_clone.lock() {
                    let receiver_result = receiver_mutex_guard.recv();
                    if let Ok(x) = receiver_result {
                        println!("Received: {}", x);
                    } else if receiver_result.is_err() {
                        println!("Error occured while receiving: {:?}", receiver_result);
                        break;
                    }
                    thread::sleep(Duration::from_millis(200));
                }
            }));
        }

        for r in receiver_threads {
            r.join().unwrap();
        }

        // while let Ok(x) = receiver.recv() {
        //     println!("Received: {}", x);
        // }

        println!("End of multiple sender single receiver test");
    }

    fn sender_dropped_and_receiver_response_test() {
        println!("Sender drop test...");
        let (sender, receiver) = std::sync::mpsc::channel::<&str>();

        drop(sender);

        let receive_handle = thread::spawn(move || {
            let receive_result = receiver.recv();

            if let Ok(result) = receive_result {
                println!("Received: {}", result);
            } else if receive_result.is_err() {
                println!("Error occured : {:?}", receive_result);
            }
        });

        receive_handle.join().unwrap();
    }

    fn receiver_dropped_and_sender_response_test() {
        println!("Receiver drop test...");
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let msg = String::from("Hello World!!");

        drop(receiver);

        let send_handle = thread::spawn(move || {
            let clone = msg.clone();
            let send_result = sender.send(msg);

            if let Ok(_) = send_result {
                println!("Send success: {}", clone);
            } else if send_result.is_err() {
                println!("Error occured: {:?}", send_result);
            }
        });

        send_handle.join().unwrap();
    }
}

fn directory_scanner(root_path: &str) {
    let root_string = String::from(root_path);
    let root = path::Path::new(&root_string);

    let mut read_dir_result = fs::read_dir(root);

    if let Ok(ref mut x) = read_dir_result {
        while let Some(dir_entry) = x.next() {
            if let Ok(dir) = dir_entry {
                println!(
                    "Name: {:?}, File type: {:?}",
                    dir.file_name(),
                    dir.file_type()
                );
            }
        }
    } else {
        println!("Error occured while reading dir {:?}", read_dir_result);
    }
}

pub fn channel_test() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        for value in (0..10).filter(|x| x % 2 == 0).enumerate() {
            println!("Sending value: {:?}", value);

            if let Err(e) = tx.send(value) {
                println!("Error occured while sending..Error: {:?}", e);
            } else {
                thread::sleep(Duration::from_millis(20));
            }
        }
    });

    while let Ok(value) = rx.recv() {
        println!("Received: {:?}", value);
    }

    println!("End of Channel test");

    //directory_scanner("/home/santosh/Desktop");
}
