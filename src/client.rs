use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, mpsc};

use crate::{
    queue::Queue,
    types::{JobRequest, JobResponse},
};

pub struct Client {
    pub stream: TcpStream,
    pub reader: BufReader<TcpStream>,
    pub input_queue: Arc<Queue<JobRequest>>,
    pub tx: mpsc::Sender<JobResponse>,
    pub rx: mpsc::Receiver<JobResponse>,
}

impl Client {
    pub fn run(&mut self) {
        loop {
            let should_continue = self.process_line();
            if !should_continue {
                break;
            }
        }
    }

    fn process_line(&mut self) -> bool {
        let mut buf = String::new();
        let _ = match self.reader.read_line(&mut buf) {
            Ok(0) => return false,
            Ok(n) => n,
            Err(e) => {
                eprintln!("read error: {}", e);
                return false;
            }
        };
        let command = buf.trim().to_string();
        if command.is_empty() {
            return true;
        }
        let job = JobRequest {
            command,
            respond_to: self.tx.clone(),
        };
        self.input_queue.push(job);
        match self.rx.recv() {
            Ok(response) => {
                if let Err(e) = writeln!(self.stream, "{}", response.value) {
                    eprintln!("write error: {}", e);
                    return false;
                }
                return true;
            }
            Err(_) => {
                eprintln!("response channel closed");
                return false;
            }
        }
    }
}
