use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Number of threads open {}", i);
            println!("The value of v is {:?}", v.get(i));
            thread::sleep(Duration::from_secs(1))
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Delayed thread.. {}", i);
        thread::sleep(Duration::from_secs(2));
    }
}
