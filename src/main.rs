use std::thread;
use mini_x::api::api;
use mini_x::frontend::frontend;

fn main() {
    let handle1 = thread::spawn(|| frontend::start());
    let handle2 = thread::spawn(|| api::start());

    let _ = handle1.join().unwrap();
    let _ = handle2.join().unwrap();
}
