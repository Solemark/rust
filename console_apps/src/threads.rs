use std::{sync::mpsc, thread, time::Duration};

#[allow(dead_code)]
pub fn threads_and_sync() -> String {
    let mut output = String::new();
    let handle = thread::spawn(|| {
        let mut output = String::new();
        for i in 1..=10 {
            output += &i.to_string();
        }
        output
    });
    for i in 1..=5 {
        output += &number_to_letter(i);
    }
    output += &handle.join().unwrap();
    output
}

fn number_to_letter(i: i32) -> String {
    match i {
        1 => "a".to_string(),
        2 => "b".to_string(),
        3 => "c".to_string(),
        4 => "d".to_string(),
        5 => "e".to_string(),
        _ => panic!(),
    }
}

#[allow(dead_code)]
pub fn thread_result_handling(msg: String) -> String {
    match thread::spawn(move || format!("message: {}", msg)).join() {
        Ok(s) => s,
        Err(_) => "Error! Thread died!".to_string(),
    }
}

#[allow(dead_code)]
pub fn sender_reciever(msg: String) -> String {
    let (sender, reciever) = mpsc::channel();
    thread::spawn(move || sender.send(msg).unwrap());
    format!("message: {}", reciever.recv().unwrap())
}

#[allow(dead_code)]
pub fn wait_a_bit(i: u64) -> String {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(i));
        format!("waited {} seconds!", i)
    })
    .join()
    .unwrap()
}
