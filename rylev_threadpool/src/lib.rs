use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    sender: mpsc::Sender<Box<dyn FnMut() + Send>>,
    _handles: Vec<std::thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = mpsc::channel::<Box<dyn FnMut() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut _handles = vec![];
        for _ in 0..num_threads {
            let receiver = receiver.clone();
            let handle = thread::spawn(move || loop {
                for mut work in receiver.lock().unwrap().try_iter() {
                    println!("Start");
                    work();
                    println!("Finish");
                }
            });
            _handles.push(handle);
        }
        Self { _handles, sender }
    }

    pub fn execute<T: FnMut() + Send + 'static>(&self, work: T) {
        println!("execute");
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_thread_sleep() {
        let pool = ThreadPool::new(10);
        for _ in 0..5 {
            pool.execute(|| {
                // println!("Hello from thread");
                thread::sleep(std::time::Duration::from_millis(100));
            });
        }

        thread::sleep(std::time::Duration::from_millis(1000));
    }

    #[test]
    fn add_num() {
        use std::sync::atomic;
        let n = atomic::AtomicU32::new(0);
        let nref = Arc::new(n);
        let foo = move || {
            thread::sleep(std::time::Duration::from_millis(500));
            nref.fetch_add(1, atomic::Ordering::SeqCst);
            println!("{:?}", nref);
            thread::sleep(std::time::Duration::from_millis(500));
        };

        let pool = ThreadPool::new(10);
        for _ in 0..5 {
            pool.execute(foo.clone());
        }

        thread::sleep(std::time::Duration::from_millis(2000));
    }
}
