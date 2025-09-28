use crate::{db::DB, types::Value};
use std::{
    fs::{self, File},
    io::{BufReader, Bytes, Read},
    iter::Peekable,
    str::from_utf8,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

// format:
// 4 byte total length
// 4 byte key len | key | 4 byte value length | value | 4 byte ttl

pub fn take_snapshot(flag: Arc<RwLock<bool>>, db: Arc<RwLock<DB>>, path: &str) {
    let full_path = env!("CARGO_MANIFEST_DIR").to_string() + path;
    println!("caching database...");
    thread::spawn(move || {
        let snapshot = db.read().unwrap();
        let mut buf: Vec<u8> = Vec::new();
        buf.extend(&(snapshot.store.len() as u32).to_le_bytes());
        for (k, v) in &snapshot.store {
            serialize_string(&mut buf, k);
            serialize_value(&mut buf, v);
        }
        let res = fs::write(&full_path, buf);
        if let Err(e) = res {
            eprintln!("snapshot error: {:?}", e);
        }
        *flag.write().unwrap() = false;
        println!("finished caching.")
    });
}

pub fn serialize_string(buf: &mut Vec<u8>, s: &str) {
    buf.extend(&(s.len() as u32).to_le_bytes()); // 4-byte length prefix
    buf.extend(s.as_bytes());
}

pub fn serialize_value(buf: &mut Vec<u8>, v: &Value) {
    // ttl can only store like 136 years before overflow
    // do not use this database if you operate beyond mortal timescales
    // also it can be off by a max of however long you configure it to wait
    // e.g. snapshot every 30 seconds -> max error 30 seconds
    let now = Instant::now();
    let ttl = match v.expires_at {
        None => 0,
        Some(time) if time > now => {
        time.saturating_duration_since(now)
            .as_secs()
            .min(u32::MAX as u64) as u32
        }
        _ => return
    };
    serialize_string(buf, &v.value);
    buf.extend(ttl.to_le_bytes());
}

pub fn deserialize(path: &str) -> Option<DB> {

    let file = File::open(path).ok()?;
    let mut source_buf = BufReader::new(file).bytes().peekable();
    let mut res = std::collections::HashMap::new();
    let mut read_buf = Vec::new();

    // get length
    read_bytes(&mut read_buf, &mut source_buf, 4).ok()?;
    let total_length = interpret_u32(&mut read_buf)?;
    read_buf.clear();

    for _ in 0..total_length {
        match read_iteration(&mut read_buf, &mut source_buf) {
            Ok(Some(x)) => res.insert(x.key, x.val),
            Err(e) => {
                eprintln!("{:?}: error deserializing", e);
                return None;
            }
            Ok(None) => {
                eprintln!("read_iteration returned None (what)");
                return None;
            }
        };
    }
    Some(DB { store: res })
}

#[derive(Debug)]
pub enum ReadErr {
    InterpretU32Failure,
    InterpretStringFailure,
    ReadBytesError,
    EOFError,
}

struct ReadEntry {
    key: String,
    val: Value,
}

// TODO: wrap everything here in helper functions
// e.g. read_u32
fn read_iteration(
    read_buf: &mut Vec<u8>,
    bytes: &mut Peekable<Bytes<BufReader<File>>>,
) -> Result<Option<ReadEntry>, ReadErr> {
    if bytes.peek().is_none() {
        return Ok(None);
    }

    read_bytes(read_buf, bytes, 4)?;
    let key_len = match interpret_u32(read_buf) {
        Some(x) => x,
        None => return Err(ReadErr::InterpretU32Failure),
    };
    read_buf.clear();
    read_bytes(read_buf, bytes, key_len)?;
    let key = match interpret_string(read_buf) {
        Some(x) => x,
        None => return Err(ReadErr::InterpretStringFailure),
    };
    read_buf.clear();
    read_bytes(read_buf, bytes, 4)?;
    let value_len = match interpret_u32(read_buf) {
        Some(x) => x,
        None => return Err(ReadErr::InterpretU32Failure),
    };
    read_buf.clear();
    read_bytes(read_buf, bytes, value_len)?;
    let value = match interpret_string(read_buf) {
        Some(x) => x,
        None => return Err(ReadErr::InterpretStringFailure),
    };
    read_buf.clear();
    read_bytes(read_buf, bytes, 4)?;
    let ttl = match interpret_u32(read_buf) {
        Some(x) => x,
        None => return Err(ReadErr::InterpretU32Failure),
    };
    read_buf.clear();

    let expires_at = if ttl == 0 {
        None
    } else {
        Some(Instant::now() + Duration::from_secs(ttl as u64))
    };
    let res = ReadEntry {
        key: key,
        val: Value {
            value: value,
            expires_at: expires_at,
        },
    };
    Ok(Some(res))
}

fn interpret_string(read_buf: &mut Vec<u8>) -> Option<String> {
    Some(from_utf8(read_buf).ok()?.to_owned())
}

fn interpret_u32(read_buf: &mut Vec<u8>) -> Option<u32> {
    let bytes: [u8; 4] = read_buf.get(0..4)?.try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}

pub fn read_bytes<'a>(
    read_buf: &'a mut Vec<u8>,
    bytes: &mut Peekable<Bytes<BufReader<File>>>,
    n: u32,
) -> Result<(), ReadErr> {
    for _ in 0..n as usize {
        match bytes.next() {
            Some(Ok(byte)) => {
                read_buf.push(byte);
            }
            Some(Err(_)) => return Err(ReadErr::ReadBytesError),
            None => return Err(ReadErr::EOFError),
        };
    }
    Ok(())
}
