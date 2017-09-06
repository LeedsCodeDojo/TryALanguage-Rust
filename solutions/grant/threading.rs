use std::thread;
use std::time::Duration;

fn main() {
    for n in 1..101 {
        thread::spawn(move || 
            for i in 1..3 { 
            println!("Thread {}!", n);
            thread::sleep(Duration::from_millis(2));
        });
    }
}