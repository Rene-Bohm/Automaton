use core::time;
use std::fmt::Error;
use std::io::{self, Error as OtherError, *};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let dur = time::Duration::from_millis(3000);

    let b = Arc::new(Mutex::new(false));
    let b1 = Arc::clone(&b); // This clone goes into Thread 1
    let b2 = Arc::clone(&b); // This clone goes into Thread 2

    let inp = Arc::new(Mutex::new(String::new()));
    let inp1 = Arc::clone(&inp); // This clone goes into Thread 1
    let inp2 = Arc::clone(&inp); // This clone goes into Thread 2

    let clock = thread::spawn(move || loop {
        let lock = b1.lock().unwrap();
        let current = !*lock;
        drop(lock);

        thread::sleep(dur);
        let mut lock = b1.lock().unwrap();
        *lock = current;
        drop(lock);

        if *inp1.lock().unwrap() == "break\n" {
            break;
        }
    });

    let display = thread::spawn(move || loop {
        
        thread::sleep(Duration::from_millis(1000));
        println!("{:?}", *b2.lock().unwrap());
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let val = inp2.lock().unwrap().clone();
        if val == "break\n" {
            break;
        }
    });

    loop {
        println!("> ");
        // std::io::stdout().flush().unwrap();
        let mut s = String::new();
        let _ = io::stdin().lock().read_line(&mut s).expect("");
        *inp.lock().unwrap() = s.clone();
        if s == "break\n" {
            break;
        }
    }

    clock.join().unwrap();
    display.join().unwrap();
}