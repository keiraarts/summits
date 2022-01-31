use std::sync::mpsc;
use std::thread;

use std::time::Duration;

fn main() {
    // multiple producer, single consumer.
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let val = String::from("Rust");
        tx.send(val).unwrap();

        let vals = vec![String::from("hi"), String::from("from")];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(5))
        }
    });

    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("from")];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(5))
        }
    });

    for received in rx {
        println!("Got {}", received);
    }
}
