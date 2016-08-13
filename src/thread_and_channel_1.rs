/// This is first try to cope with threads and channel messages from threads
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn run() {
    let (tx,rx) = channel();

    thread::spawn(move || {
        let mut count = 0u32;

        while count < 5 {
            count += 1;
            tx.send(count.clone()).expect("Failed to send message");
            thread::sleep(Duration::new(1, 0));
        }
    });

    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("count tick: {}", msg);
            },
            Err(_) => {
                continue;
            }
        }
    }
}

