use std::net::{TcpListener, TcpStream};

use rpc_example_rs::{OperationResponse, OperationRequest, OperationStatus, PermittedOperations};



fn compute_response(ops: OperationRequest) -> OperationResponse {
    match ops.op {
        PermittedOperations::Sum => OperationResponse {
            status: OperationStatus::Success,
            value: ops.args.iter().sum()
        },
        PermittedOperations::Mean => OperationResponse {
            status: OperationStatus::Success,
            value: ops.args.iter().sum::<f32>() / ops.args.len() as f32
        }
    }
}   

fn handle_client(stream: TcpStream)  {
    // Lire le stream pour essayer de récupérer une requête connue
    let result = OperationRequest::from_stream(&stream);
    
    let response = match result {
        Ok(ops) => compute_response(ops),
        Err(_) => OperationResponse {
            status: OperationStatus::Failure(String::from("Error found")),
            value: 0.0
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
