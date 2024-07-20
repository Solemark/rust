mod tasks;
use tasks::{w1::w1, w2::w2, w3::w3, w4::w4, w5::w5};

fn main() {
    println!("{}", w1());
    println!("{:?}", w2());
    println!("{:?}", w3());
    println!("{:?}", w4());
    let _ = w5();
}
