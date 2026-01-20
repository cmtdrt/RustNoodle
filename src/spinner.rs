use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub fn run_with_spinner<T, F>(message: &str, f: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = mpsc::channel::<T>();
    thread::spawn(move || {
        let res = f();
        let _ = tx.send(res);
    });

    let spinner = ['|', '/', '-', '\\'];
    let mut i = 0usize;

    loop {
        match rx.recv_timeout(Duration::from_millis(80)) {
            Ok(v) => {
                print!("\r{}  \n", " ".repeat(message.len() + 2));
                let _ = io::stdout().flush();
                return v;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let ch = spinner[i % spinner.len()];
                i += 1;
                print!("\r{} {}", message, ch);
                let _ = io::stdout().flush();
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("\r{} (interrompu)", message);
                panic!("Thread spinner déconnecté");
            }
        }
    }
}

pub fn run_with_spinner_timeout<T, F>(message: &str, timeout: Duration, f: F) -> Option<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = mpsc::channel::<T>();
    thread::spawn(move || {
        let res = f();
        let _ = tx.send(res);
    });

    let spinner = ['|', '/', '-', '\\'];
    let mut i = 0usize;
    let start = Instant::now();

    loop {
        if start.elapsed() >= timeout {
            print!("\r{}  \n", " ".repeat(message.len() + 2));
            let _ = io::stdout().flush();
            return None;
        }

        match rx.recv_timeout(Duration::from_millis(80)) {
            Ok(v) => {
                print!("\r{}  \n", " ".repeat(message.len() + 2));
                let _ = io::stdout().flush();
                return Some(v);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let ch = spinner[i % spinner.len()];
                i += 1;
                print!("\r{} {}", message, ch);
                let _ = io::stdout().flush();
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("\r{} (interrompu)", message);
                return None;
            }
        }
    }
}

