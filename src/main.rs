use std::env;
use std::net::TcpListener;
use std::sync::RwLock;
use std::time::Duration;
use std::{sync::Arc, thread};

use types::JobRequest;

use crate::client::handle_client;
use crate::db::DB;
use crate::queue::Queue;
use crate::utils::get_full_path;

mod client;
mod db;
mod queue;
mod snapshot;
mod types;
mod utils;

fn main() {
    // TODO: make args better
    let mut args = env::args();
    let addr: String = args.nth(1).unwrap_or("127.0.0.1:6379".to_string());
    let path: String = args.nth(2).unwrap_or("/data/cache".to_string());
    let snapshot_interval = match args.nth(3) {
        None => 30,
        Some(i) => u64::from_str_radix(&i, 10).unwrap_or(30)
    };

    // TCP listener
    let listener = TcpListener::bind(addr).unwrap();

    // tracks if the db has been updated
    let dirty: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));

    // create database
    let database = Arc::new(RwLock::new(DB::restore_or_new(&path)));

    // input queue
    let input_queue: Arc<Queue<JobRequest>> = Arc::new(Queue::new());

    // Worker Thread
    let iq = input_queue.clone();
    let uf = dirty.clone();
    let db = database.clone();
    thread::spawn(move || {
        loop {
            let job = iq.wait_pop();
            let response = db.write().unwrap().process(&job);
            job.respond(response.value);
            let mut flag = uf.write().unwrap();
            *flag = true;
        }
    });

    // IO Watcher Thread
    let uf = dirty.clone();
    let db = database.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(snapshot_interval));
            let flag = uf.read().unwrap();
            if *flag {
                snapshot::take_snapshot(uf.clone(), db.clone(), &get_full_path(&path))
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
