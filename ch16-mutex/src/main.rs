use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(5));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            println!("{} before lock", i);
            let mut num = counter.lock().unwrap();
            println!("{} after lock", i);
            thread::sleep(Duration::from_millis(1000));
            *num += 1;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
