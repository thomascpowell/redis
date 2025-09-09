use std::{sync::Arc, thread};

use redis::{JobRequest, JobResponse};

use crate::db::DB;
use crate::queue::Queue;

mod db;
mod queue;
mod types;

fn main() {
    let input_queue: Arc<Queue<JobRequest>> = Arc::new(Queue::new());
    let output_queue: Arc<Queue<JobResponse>> = Arc::new(Queue::new());

    let iq = input_queue.clone();
    thread::spawn(move || {
        let test: JobRequest = JobRequest {
            client: 0,
            command: "SET a a".to_string(),
        };
        iq.push(test);
    });

    let iq = input_queue.clone();
    let oq = output_queue.clone();
    thread::spawn(move || {
        let mut db: DB = DB::new();
        loop {
            oq.push(db.process(iq.wait_pop()))
        }
    });

    let oq = output_queue.clone();
    let t = thread::spawn(move || {
        loop {
            println!("{}", oq.wait_pop().value)
        }
    });

    t.join().unwrap()
}
