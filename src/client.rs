use serde_json::json;

use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8087")?;
        
    let buf = json!({
        "op": "Mean",
        "args": [
            1, 2, 3
        ]
    });
    let _ = stream.write(buf.to_string().as_bytes());
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;

    println!("{}", buffer);
    Ok(())
}