use std::thread;

use env_logger::Env;
use mini_x::api::api_server;
use mini_x::frontend::client;

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let handle1 = thread::spawn(client::start);
    let handle2 = thread::spawn(api_server::start);

    let _ = handle1.join().unwrap();
    let _ = handle2.join().unwrap();
}
