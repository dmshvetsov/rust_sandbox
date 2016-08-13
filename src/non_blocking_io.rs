use std::thread;
use std::io;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let mut buf = String::new();

        loop {
            match io::stdin().read_line(&mut buf) {
                Ok(_) => {
                    tx.send(buf.clone()).expect("Failed to send message");
                },
                Err(error) => println!("error: {}", error),
            }

            buf.clear();
        }
    });

    let mut rx_loop_count = 0u32;
    let wait_for = Duration::from_millis(300);

    loop {
        rx_loop_count += 1;
        match rx.try_recv() {
            Ok(msg) => println!("[LOOP:{}] Input: {}", rx_loop_count, msg),
            Err(_) => {}
        }
        thread::sleep(wait_for);
    }
}
