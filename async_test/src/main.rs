use std::{sync::mpsc, thread, time::Duration};

fn main() {
    threads_vs_sync();
    thread_result_handling();
    sender_receiver_example();
}

fn threads_vs_sync() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn thread_result_handling() {
    let val = match thread::spawn(|| String::from("Hello world!")).join() {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    println!("{}", val);
}

fn sender_receiver_example() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send(String::from("Hi!")).unwrap());
    println!("Got: {}", rx.recv().unwrap());
}
