use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// Do not communicate by sharing memory
// INSTEAD
// Share memory by communicating
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi, number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi, number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("here's a vector: {v:?}");
    });

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hiya");
        tx.send(msg).unwrap(); // send takes ownership of the message
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let msgs = vec![
            String::from("hey"),
            String::from("from"),
            String::from("da"),
            String::from("thread"),
        ];

        for msg in msgs {
            tx.send(msg).unwrap(); // send takes ownership of the message
            thread::sleep(Duration::from_millis(1)); // from_secs
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for msg in msgs {
            tx2.send(msg).unwrap(); // send takes ownership of the message
            thread::sleep(Duration::from_millis(1)); // from_secs
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 7;
    }

    println!("m = {:?}", m);

    // * The RefCell smart pointer allows mutation inside the Rc smart pointer
    // ! It also comes whith the risks of creating circular dependencies
    // * The Mutex smart pointer allows mutation inside the Arc smart pointer
    // ! It also comes whith the risks of creating deadlocks

    // let counter = Rc::new(Mutex::new(0));
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
