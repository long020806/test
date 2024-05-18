use std::sync::mpsc::channel;
use std::thread;
pub fn main() {
    let (tx, rx) = channel();
    let join_handle = thread::spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("Received {}", n);
        }
    });
    for i in 0..10 {
        tx.send(i).unwrap();
    }
    join_handle.join().unwrap();
}
