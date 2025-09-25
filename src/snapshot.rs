use crate::{db::DB, types::Value};
use std::{
    fs::{self, File},
    io::{BufReader, Bytes, Read},
    sync::{Arc, RwLock},
    thread,
    time::Instant,
};

// serializes in format:
// 4 byte key length
// key (string)
//
// 4 byte value length
// Value.value (string)
//
// value of length l
// 4 byte ttl (in seconds)

// 4 byte key len | key | 4 byte value length | value | 4 byte ttl

pub fn take_snapshot(flag: Arc<RwLock<bool>>, db: Arc<RwLock<DB>>, path: String) {
    thread::spawn(move || {
        let snapshot = db.read().unwrap();
        let mut buf: Vec<u8> = Vec::new();
        buf.extend(&(snapshot.store.len() as u32).to_le_bytes());
        for (k, v) in &snapshot.store {
            serialize_string(&mut buf, k);
            serialize_value(&mut buf, v);
        }
        let res = fs::write(&path, buf);
        if let Err(e) = res {
            eprintln!("snapshot error: {:?}", e);
        }
        *flag.write().unwrap() = false;
    });
}

pub fn serialize_string(buf: &mut Vec<u8>, s: &str) {
    buf.extend(&(s.len() as u32).to_le_bytes()); // 4-byte length prefix
    buf.extend(s.as_bytes());
}

pub fn serialize_value(buf: &mut Vec<u8>, v: &Value) {
    // can only store 132 years before overflow
    // do not use this database if you operate beyond mortal timescales
    let ttl = match v.expires_at {
        None => 0,
        Some(time) => time
            .saturating_duration_since(Instant::now())
            .as_secs()
            .min(u32::MAX as u64) as u32,
    };
    serialize_string(buf, &v.value);
    buf.extend(ttl.to_le_bytes());
}

#[derive(Debug)]
enum ReadBuf {
    // define the bufs than can be read into
    // resize them after reading the length bytes
    // key buf
    // value buf
}
impl ReadBuf {
    // define method to read stuff in
    //   read len -> resize
    //   read data

    // define methods to interpret the bufs when full
    // e.g. Key -> Option<String>
    
    // get a mutable reference to the current buf
}

// pattern: 4 byte key len | key | 4 byte value length | value | 4 byte ttl

pub fn deserialize(path: &str) -> Option<DB> {
    let file = File::open(path).ok()?;
    let source_buf = BufReader::new(file);

    let res = std::collections::HashMap::new();
    
    {
        // use methods from ReadBuf
        // 1 loop = one full pattern
        //
        // define a buf variable, call methods, define variables
        // e.g. buf.read_len
        //      buf.read_data
        //
        // once the loop is complete (key, value are defined)
        // update res
        // None anywhere -> break
    }
    
    Some(DB { store: res })
}

pub fn read_bytes<'a>(read_buf: &'a mut Vec<u8>, bytes: &mut Bytes<BufReader<File>>, number: u8) -> Option<&'a Vec<u8>> {
    for _ in 0..number {
        // read bytes into the buf 
        // read_buf borrowed from ReadBuf
        // 
        let byte = bytes.next()?;
    }

    return Some(read_buf);
}
