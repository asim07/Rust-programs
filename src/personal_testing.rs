#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

pub fn test() {
    
    let code = String::from("abcd");
    let mut entry = String::new();
    let mut state = State::Locked;

    loop {
        match state{
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                    entry.push_str(&input.trim_end())
                    }
                    Err(_) => {
                        continue;
                    }
                   
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }

                if code==entry {
                    state = State::Unlocked;
                }
            },       
            State::Unlocked => {
                println!("U win..");
                return;
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }

        }
    }
}
