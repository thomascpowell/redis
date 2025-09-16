use std::env;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::{
    sync::{Arc, mpsc},
    thread,
};

use types::{JobRequest, JobResponse};

use crate::client::Client;
use crate::db::DB;
use crate::queue::Queue;

mod client;
mod db;
mod queue;
mod types;

fn main() {
    let addr: String = env::args().nth(1).unwrap_or("127.0.0.1:6379".to_string());
    let input_queue: Arc<Queue<JobRequest>> = Arc::new(Queue::new());
    let listener = TcpListener::bind(addr).unwrap();

    // Worker Thread
    let iq = input_queue.clone();
    let t = thread::spawn(move || {
        let mut db: DB = DB::new();
        loop {
            let job = iq.wait_pop();
            let response = db.process(&job);
            job.respond(response.value);
        }
    });

    // IO Threads
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
