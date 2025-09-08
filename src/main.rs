use std::{sync::Arc, thread};

use crate::{threads::process, types::Queue};

mod command;
mod db;
mod threads;
mod types;

fn main() {
    let input_queue: Arc<Queue<String>> = Arc::new(Queue::new());
    let output_queue: Arc<Queue<String>> = Arc::new(Queue::new());

    let iq = input_queue.clone();
    let input_thread = thread::spawn(move || {
        // TODO: input loop
        let test: String = "SET test test".to_string();
        iq.push(test);
    });

    let iq = input_queue.clone();
    let oq = output_queue.clone();
    let process_thread = thread::spawn(move || {
        loop {
            let string_command = iq.wait_pop();
            oq.push(process(string_command))
        }
    });

    let oq = output_queue.clone();
    let output_thread = thread::spawn(move || {
        loop {
            println!("{}", oq.wait_pop())
        }
    });

    input_thread.join().unwrap();
    process_thread.join().unwrap();
    output_thread.join().unwrap();
}
