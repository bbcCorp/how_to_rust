use std::sync::mpsc::channel;
use std::thread;
use std::thread::{JoinHandle, Thread};
use std::time::Duration;

fn main() {
    println!("=== Demo of threads in Rust ===");

    demo_simple_thread();

    demo_nested_threads();

    demo_thread_communication();
}

fn demo_simple_thread() {
    println!("\n\n===Simple thread===");

    // Spawning threads
    // spawn func needs a closure which kicks off when the threads kicks off. It returns a JoinHandle
    let t1: JoinHandle<()> = thread::spawn(|| println!("Logging from thread 1"));

    t1.join().unwrap();
}

fn demo_nested_threads() {
    println!("\n\n===Nested threads===");

    let t2: JoinHandle<()> = thread::spawn(|| println!("Logging from thread 2"));

    let t3: JoinHandle<()> = thread::spawn(|| {
        println!("Starting thread 3");

        // Capture t2
        let two: &Thread = t2.thread();
        println!("Thread2 name:{:?} and id:{:?}", two.name(), two.id());
        t2.join().unwrap();

        // In this way we can be sure that t2 finishes before t3
        println!("Finishing thread 3");
    });

    t3.join().unwrap();
}

fn demo_thread_communication() {
    println!("\n\n===Communication between threads===");

    let (transmitter, receiver) = channel();

    let sender_thread: JoinHandle<()> = thread::spawn(move || {
        println!("Starting sender_thread");

        // generate messages here
        for i in 0..20 {
            let msg: String = format!("Message:{}", i + 1);
            println!("sender_thread: sending msg:{}", msg);
            transmitter.send(msg).unwrap();
            thread::sleep(Duration::from_millis(500));
        }

        println!("Finished sender_thread");
    });

    let receiver_thread: JoinHandle<()> = thread::spawn(move || {
        println!("Starting receiver_thread");

        // use receiver here
        for _ in 0..20 {
            let msg: String = receiver.recv().unwrap();
            println!("receiver_thread Received msg:{}", msg);
            thread::sleep(Duration::from_millis(750));
        }
        println!("Finishing receiver_thread");
    });

    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();
}
