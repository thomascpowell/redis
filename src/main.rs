use std::env;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::time::Duration;
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
mod snapshot;
mod types;

fn main() {
    let addr: String = env::args().nth(1).unwrap_or("127.0.0.1:6379".to_string());
    let input_queue: Arc<Queue<JobRequest>> = Arc::new(Queue::new());
    let listener = TcpListener::bind(addr).unwrap();
    let updated_flag: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let database: Arc<Mutex<DB>> = Arc::new(Mutex::new(DB::new()));

    // Worker Thread
    let iq = input_queue.clone();
    let uf = updated_flag.clone();
    let db = database.clone();
    thread::spawn(move || {
        loop {
            let job = iq.wait_pop();
            let response = db.lock().unwrap().process(&job);
            job.respond(response.value);
            let mut flag = uf.lock().unwrap();
            *flag = true;
        }
    });

    // IO Watcher Thread
    let uf = updated_flag.clone();
    let db = database.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(30));
            let flag = uf.lock().unwrap();
            if *flag {
                snapshot::take_snapshot(uf.clone(), db.clone());
            }
        }
    });

    // TCP Threads
    for stream in listener.incoming() {
        let iq = input_queue.clone();
        let stream = stream.unwrap();
        thread::spawn(move || handle_client(stream, iq));
    }
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
