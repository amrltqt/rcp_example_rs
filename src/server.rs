use std::net::{TcpListener, TcpStream};
use log::{info, error};
use uuid::Uuid;

use rpc_example_rs::{
    OperationResponse, 
    OperationRequest, 
    OperationStatus,
    PermittedOperations
};

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
    let ip = stream.local_addr().unwrap().ip();
    let id_request = Uuid::new_v4();
    info!("{} - {} start a new connection ", id_request, ip);

    // Lire le stream pour essayer de récupérer une requête connue
    let result = OperationRequest::from_stream(&stream);
    let response = match result {
        Ok(ops) => {
            info!("{} - Operation requested: {:?}", id_request, ops);
            compute_response(ops)
        },
        Err(_) => OperationResponse {
            status: OperationStatus::Failure(String::from("Error found")),
            value: 0.0
        }
    };

    info!("{} - Close request with response: {:?}", id_request, response);
    let _ = serde_json::to_writer(stream, &response);
    
}

fn main() -> std::io::Result<()> {

    env_logger::init();

    // On lui envois un json avec {"op": "+", "args": [1, 2, 3]}
    // Il renvoit {"status": "ok", "value": 6}
    let listener = TcpListener::bind("127.0.0.1:8087")?;
    info!("Server started");

    for potential_stream in listener.incoming() {
        match potential_stream {
            Ok(stream) => {
                handle_client(stream);
            },
            Err(_) => {
                error!("Impossible to handle client");
            }
        }
    }

    Ok(())
}
