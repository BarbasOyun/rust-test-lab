use std::time::Duration;
use std::thread;
use std::sync::mpsc;
// use std::rc::Rc; // doesn't work with multiple threads
use std::sync::{Arc, Mutex}; // Arc = Atomic RC -> Work with multiple threads

pub fn test_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn data_from_main_to_spawned() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // drop(v); // oh no!

    handle.join().unwrap();
}

pub fn test_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {val}"); // Loose ownership on sending in a transmitter
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

pub fn multiple_data_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

// mpsc
pub fn multiple_producer_single_consumer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

pub fn test_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // Get the lock
        *num = 6;
        // Drop and Lock release happen automatically when out of scope
    }

    println!("m = {m:?}");
}

pub fn threads_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn dead_lock(a: i16, b: i16, c: i16) {
    // e.g. calculating (a * b) + (a * b * c)
    // Single Thread -> 1) a * b = x 2) x * c = y 3) x + y
    // Double Thread -> 1) a * b = x && a * b * c = y 2) x + y
    // Dead Lock = Thread1 own a, wait for b && Thread2 own b, wait for a
    // Thread1 Take a, Wait 1s and try to take b && Thread2 Take b and try to take a

    // Mutex + Smart Pointer
    let ma = Arc::new(Mutex::new(a));
    let mb = Arc::new(Mutex::new(b));
    let mc = Arc::new(Mutex::new(c));
    let mr = Arc::new(Mutex::new(0));

    // Thread1 : a * b
    let cma = Arc::clone(&ma);
    let cmb = Arc::clone(&mb);
    let cmr = Arc::clone(&mr);

    let handle1 = thread::spawn(move || {
        let access_a = cma.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let access_b = cmb.lock().unwrap();
        let mut access_result = cmr.lock().unwrap();

        *access_result += *access_a * *access_b;
    });

    // Thread2 : a * b * c
    let cma = Arc::clone(&ma);
    let cmb = Arc::clone(&mb);
    let cmc = Arc::clone(&mc);
    let cmr = Arc::clone(&mr);

    let handle2 = thread::spawn(move || {
        let access_b = cmb.lock().unwrap(); // Take b first
        let access_a = cma.lock().unwrap();
        let access_c = cmc.lock().unwrap();
        let mut access_result = cmr.lock().unwrap();

        *access_result += *access_a * *access_b * *access_c;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Result : {}", *mr.lock().unwrap())
}