#![allow(dead_code)]

use std::io::stdin;

//enum construct that have a list of different possible values
enum State {
    Locked,
    Failed,
    Unlocked,
}
pub fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();
    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end()) //before it's for only a character but now it appends a number of characters!
                    }
                    Err(_) => continue,
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    //if the string not matches to "1234"
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("Failed");
                entry.clear(); //"", means if you have something in the string then it would be the empty string
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("Unlocked!");
                return;
            }
        }
    }
}
