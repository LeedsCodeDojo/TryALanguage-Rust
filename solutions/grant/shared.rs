use std::thread;
use std::time::Duration;

fn main() {
    let mut val = 0;
    for _ in 1..10 {
        thread::spawn(move || {
            val = val + 1;
            print!("Value now: {}\n", val);
        });
        
        thread::sleep(Duration::from_millis(10));
    }
    
    thread::sleep(Duration::from_millis(10));
    
    print!("Final value: {}", val);
}