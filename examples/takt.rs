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
        let current = !*b1.lock().unwrap();
        thread::sleep(dur);
        *b1.lock().unwrap() = current;
        if *inp1.lock().unwrap() == "break" {
            break;
        }
    });

    let display = thread::spawn(move || loop {
        println!("{:?}", *b2.lock().unwrap());
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        if *inp2.lock().unwrap() == "break" {
            break;
        }
    });

    loop {
        let x = io::stdin().read_line(&mut *inp.lock().unwrap());
    }

    clock.join().unwrap();
    display.join().unwrap();
}
