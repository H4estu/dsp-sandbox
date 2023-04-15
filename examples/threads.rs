use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;


fn spawn_thread(duration: u64) {
    thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(duration));  // sleep 1 ms
        }
    });
}

fn main() {
    println!("Testing thread behavior...");

    let thread_sleep = 1;  // Threads sleep for 1 millisecond
    
    let spawned_thread = spawn_thread(thread_sleep);
    spawned_thread;

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(thread_sleep));
    };
}