use std::{sync::Arc, thread};

use crate::db::DB;
use crate::queue::Queue;

mod db;
mod queue;
mod types;

fn main() {
    let input_queue: Arc<Queue<String>> = Arc::new(Queue::new());
    let output_queue: Arc<Queue<String>> = Arc::new(Queue::new());

    let iq = input_queue.clone();
    thread::spawn(move || {
        let test: String = "SET test test".to_string();
        iq.push(test);
        let test: String = "GET test".to_string();
        iq.push(test);

    });

    let iq = input_queue.clone();
    let oq = output_queue.clone();
    thread::spawn(move || {
        let mut db: DB = DB::new();
        loop {
            let string_command = iq.wait_pop();
            oq.push(db.process(string_command))
        }
    });

    let oq = output_queue.clone();
    let t = thread::spawn(move || {
        loop {
            println!("{}", oq.wait_pop())
        }
    });

    t.join().unwrap()
}
