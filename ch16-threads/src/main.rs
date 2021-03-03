use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn part2() {
    let (tx, rx) = mpsc::channel();

    for j in 1..9 {
        let tx1 = tx.clone();
        thread::spawn(move || {
            for i in 1..10 {
                tx1.send(format!("{} {}", j, i)).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    // Must drop the original tx otherwise the refcount is never 0
    // the rx iterator never finishes and the program hangs
    drop(tx);

    for received in rx {
        println!("Got: {}", received);
    }
}

fn part1() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap()
}

fn main() {
    // part1();
    part2();
    // part2_demo();
}
