use std::{sync::mpsc, thread, time::Duration};

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from spawned thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     handle.join().unwrap();
//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// fn main() {
//     let val = match thread::spawn(hello_world).join() {
//         Ok(s) => s,
//         Err(_) => panic!(),
//     };
//     println!("{}", val);
// }

// fn hello_world() -> String {
//     String::from("Hello world!")
// }

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let recieved = rx.recv().unwrap();
    println!("Got: {}", recieved);
}
