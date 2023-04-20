use std::sync::mpsc;
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

fn channel_test(duration: u64) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..10 {
            let to_send = i;
            tx.send(to_send).unwrap();
            let received = rx.recv().unwrap();
            println!("Received value: {}", received);
            thread::sleep(Duration::from_millis(duration));
        }
    });
}

fn main() {
    println!("Testing thread behavior...");

    // time to sleep for, in milliseconds
    let sleep_spawned = 1;
    let sleep_main = 50;
    let sleep_channel = 10;
    
    spawn_thread(sleep_spawned);
    channel_test(sleep_channel);

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(sleep_main));
    };
}