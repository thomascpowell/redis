use std::{sync::Arc, thread};

use crate::{
    command::{execute, parse},
    types::Queue,
};

mod command;
mod db;
mod types;

fn main() {
    let input_queue: Arc<Queue<String>> = Arc::new(Queue::new());
    let output_queue: Arc<Queue<String>> = Arc::new(Queue::new());

    // input thread
    let q = input_queue.clone();
    thread::spawn(move || {
        // TODO: input loop
        let test: String = "SET test test".to_string();
        // use owned strings so that input_queue owns them
        q.push(test);
    });

    // processing thread
    let q = input_queue.clone();
    thread::spawn(move || {
        let string_command = q.wait_pop();
        let result: String = match parse(&string_command) {
            Some(cmd) => {
                let execute_result = execute(cmd);
                execute_result.unwrap_or_else(|err| format!("error: {:?}", err))
            }
            _ => format!("error: (invalid command) {}", string_command),
        };
        // TODO: send string to output_queue
    });

    // TODO: output thread using output_queue
}
