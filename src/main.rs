use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::{sync::{mpsc, Arc}, thread};

use types::{JobRequest, JobResponse};

use crate::client::Client;
use crate::db::DB;
use crate::queue::Queue;

mod db;
mod queue;
mod client;
mod types;

fn main() {
    let input_queue: Arc<Queue<JobRequest>> = Arc::new(Queue::new());
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    // worker
    let iq = input_queue.clone();
    let t = thread::spawn(move || {
        let mut db: DB = DB::new();
        loop {
            let job = iq.wait_pop();
            let response = db.process(&job);
            job.respond(response.value);
        }
    });

    for stream in listener.incoming() {
        let iq = input_queue.clone();
        let stream = stream.unwrap();
        thread::spawn(move || handle_client(stream, iq));
    }

    t.join().unwrap();
}

fn handle_client(stream: TcpStream, input_queue: Arc<Queue<JobRequest>>) {
    let (tx, rx) = mpsc::channel::<JobResponse>();
    let reader = BufReader::new(stream.try_clone().unwrap());
    let mut client = Client {
        stream,
        reader,
        input_queue,
        tx,
        rx,
    };
    client.run();
}

