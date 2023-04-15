use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;


fn spawn_thread(duration: u64) {
    thread::spawn(move || {
        for i in 1..100 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(duration));  // sleep 1 ms
        }
    });
}

fn main() {
    println!("Testing thread behavior...");

    // time to sleep for, in milliseconds
    let sleep_spawned = 1;
    let sleep_main = 50;
    
    spawn_thread(sleep_spawned);

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(sleep_main));
    };
}