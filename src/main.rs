use std::{sync::Arc, thread};

use crate::{
    threads::process,
    types::Queue,
};

mod command;
mod db;
mod threads;
mod types;

fn main() {
    let input_queue: Arc<Queue<String>> = Arc::new(Queue::new());
    let output_queue: Arc<Queue<String>> = Arc::new(Queue::new());

    // input thread
    let iq = input_queue.clone();
    thread::spawn(move || {
        // TODO: input loop
        let test: String = "SET test test".to_string();
        // use owned strings so that input_queue owns them
        iq.push(test);
    });

    // processing thread
    let iq = input_queue.clone();
    let oq = output_queue.clone();
    thread::spawn(move || {
        loop {
            let string_command = iq.wait_pop();
            oq.push(process(string_command))
        }
    });

    //output thread
    thread::spawn(move || {});
}
