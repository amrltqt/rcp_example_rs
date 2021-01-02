use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::{TcpListener, TcpStream};

#[derive(Deserialize, Debug)]
enum PermittedOperations {
    Sum,
    Mean
}

#[derive(Deserialize, Debug)]
struct OperationRequest {
    op: PermittedOperations,
    args: Vec<i32>
}

#[derive(Serialize, Debug)]
struct OperationResponse {
    status: String,
    value: i32
}

fn read_operation_request_from_stream(stream: &TcpStream) -> Result<OperationRequest, Box<dyn Error>> {
    let mut de = serde_json::Deserializer::from_reader(stream);
    let ops = OperationRequest::deserialize(&mut de)?;
    Ok(ops)
} 

fn compute_response(ops: OperationRequest) -> OperationResponse {
    match ops.op {
        PermittedOperations::Sum => OperationResponse {
            status: String::from("Ok"),
            value: ops.args.iter().sum()
        },
        PermittedOperations::Mean => OperationResponse {
            status: String::from("Not supported"),
            value: 0
        }
    }
}

fn handle_client(stream: TcpStream)  {

    // Lire le stream pour essayer de récupérer une requête connue
    let result = read_operation_request_from_stream(&stream);
    
    let response = match result {
        Ok(ops) => compute_response(ops),
        Err(_) => OperationResponse {
            status: String::from("Error"),
            value: 0
        }
    };

    let _ = serde_json::to_writer(stream, &response);
}

fn main() -> std::io::Result<()> {
    println!("Rusties - Calculette en ligne");

    // On lui envois un json avec {"op": "+", "args": [1, 2, 3]}
    // Il renvoit {"status": "ok", "value": 6}
    let listener = TcpListener::bind("127.0.0.1:8087")?;
    
    for stream in listener.incoming() {
        handle_client(stream?);        
    }

    Ok(())
}
