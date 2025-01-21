use std::sync::{Mutex};

fn main() {
    println!("=== Demo of a simple mutex === ");
    simple_mutex_demo();
}

fn simple_mutex_demo(){
    let simple_mutex = Mutex::new(10);

    // Display the mutex
    println!("{:?}", simple_mutex);

    // Display the data inside the mutex
    let num1 = simple_mutex.lock().unwrap();
    println!("Data inside mutex: {:?}", num1);

    // We drop the mutex guard to if release the lock explicitly 
    // Without this the program will get stuck next time we try to acquire a lock
    drop(num1);

    // we will create a new scope to represent a thread context
    {
        // acquire the lock 
        let mut num = simple_mutex.lock().unwrap(); 

        // modify the data 
        *num = 20;

    } // lock is automatically released here

    // Display the mutex
    println!("{:?}", simple_mutex);

    let num2 = simple_mutex.lock().unwrap();
    println!("Data inside mutex: {:?}", num2);
    drop(num2);
}
