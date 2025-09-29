use std::io::{BufRead, BufReader, Read, Write};
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

#[derive(Debug)]
pub enum IOError {
    MissingCRLF,
    // Default,
    InvalidData,
}

pub fn handle_client(stream: TcpStream, input_queue: Arc<Queue<JobRequest>>) {
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

impl Client {
    pub fn run(&mut self) {
        loop {
            let should_continue = self.handle_input();
            if !should_continue {
                break;
            }
        }
    }

    // return type corresponds to "should continue?"
    fn handle_input(&mut self) -> bool {
        match self.get_valid_io() {
            Ok(tokens) => {
                let job = JobRequest {
                    tokens,
                    respond_to: self.tx.clone(),
                };
                self.input_queue.push(job);
            }
            Err(e) => {
                eprintln!("client error: {:?}", e);
                return false;
            }
        }
        match self.rx.recv() {
            Ok(response) => {
                if let Err(e) = write!(self.stream, "{}", response.value) {
                    eprintln!("write error: {}", e);
                }
            }
            Err(_) => {
                eprintln!("response channel closed");
                return false;
            }
        }
        return true;
    }

    fn get_valid_io(&mut self) -> Result<Vec<String>, IOError> {
        let mut tokens: Vec<String> = Vec::new();
        let mut line = String::new();
        // read overall length
        self.get_line(&mut line)?;
        let command_len: usize = line
            .trim_end()
            .strip_prefix('*')
            .ok_or(IOError::InvalidData)?
            .parse()
            .map_err(|_| IOError::InvalidData)?;
        // read tokens
        for _ in 0..command_len {
            line.clear();
            // read next token length
            self.get_line(&mut line)?;
            let token_len: usize = line
                .trim_end()
                .strip_prefix('$')
                .ok_or(IOError::InvalidData)?
                .parse()
                .map_err(|_| IOError::InvalidData)?;
            let mut token_buf = vec![0; token_len];
            self.get_exact(&mut token_buf)?;
            // read crlf
            let mut crlf = [0; 2];
            self.get_exact(&mut crlf)?;
            if &crlf != b"\r\n" {
                return Err(IOError::MissingCRLF);
            }
            // store the token
            let token = String::from_utf8(token_buf).map_err(|_| IOError::InvalidData)?;
            tokens.push(token);
        }
        println!("{:?}", tokens);
        Ok(tokens)
    }

    fn get_line(&mut self, buf: &mut String) -> Result<usize, IOError> {
        let res = self.reader.read_line(buf).map_err(|_| IOError::InvalidData);
        res
    }

    fn get_exact(&mut self, buf: &mut [u8]) -> Result<(), IOError> {
        let res = self
            .reader
            .read_exact(buf)
            .map_err(|_| IOError::MissingCRLF);
        res
    }
}
