use std::thread;

use diesel::connection::SimpleConnection;
use mini_x::api::api;
use mini_x::establish_connection;
use mini_x::frontend::frontend;

fn init_db(){
    const SCHEMA_SQL: &str = include_str!("schema.sql");
    let mut conn = establish_connection();
    let _ = conn.batch_execute(&SCHEMA_SQL);
}

fn main() {
    let _ = init_db();

    let handle1 = thread::spawn(|| frontend::start());
    let handle2 = thread::spawn(|| api::start());

    let _ = handle1.join().unwrap();
    let _ = handle2.join().unwrap();
}
