use std::thread;
use std::thread::{JoinHandle, Thread};

fn main() {
    println!("=== Demo of threads in Rust ===");

    demo_simple_thread();

    demo_nested_threads();
}

fn demo_simple_thread() {
    // Spawning threads
    // spawn func needs a closure which kicks off when the threads kicks off. It returns a JoinHandle
    let t1: JoinHandle<()> = thread::spawn(|| println!("Logging from thread 1"));

    t1.join().unwrap();
}

fn demo_nested_threads() {
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
