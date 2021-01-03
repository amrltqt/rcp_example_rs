use std::net::TcpStream;

use rpc_example_rs::{OperationRequest, PermittedOperations, OperationResponse};


fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8087")?;
        
    let ops = OperationRequest {
        op: PermittedOperations::Sum,
        args: vec![1.0, 2.0, 3.0]
    };

    let _ = serde_json::to_writer(&stream, &ops);
    
    let result = OperationResponse::try_from_stream(&stream);
    match result {
        Ok(response) => {
            println!("{:?}", response);
        },
        Err(err) => {
            println!("{:?}", err);
        } 
    }

    Ok(())
}