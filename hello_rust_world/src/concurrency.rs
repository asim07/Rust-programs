use std::thread;
use std::time;
pub fn spawning_and_joining_thread() {
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_millis(500));
        }
    });
    for _ in 1..10 {
        print!("_");
        thread::sleep(time::Duration::from_millis(300));
    }
    let a = handle.join(); //wait for the thread to finish
}
