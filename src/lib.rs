
use serde::{Deserialize, Serialize};
use std::net::{TcpStream};


#[derive(Deserialize, Serialize, Debug)]
pub enum PermittedOperations {
    Sum,
    Mean
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OperationStatus {
    Success,
    Failure(String)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OperationRequest {
    pub op: PermittedOperations,
    pub args: Vec<f32>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OperationResponse {
    pub status: OperationStatus,
    pub value: f32
}

impl OperationRequest {
    
    pub fn from_stream(stream: &TcpStream) -> Result<OperationRequest, Box<dyn std::error::Error>> {
        let mut de = serde_json::Deserializer::from_reader(stream);
        let ops = OperationRequest::deserialize(&mut de)?;
        Ok(ops)
    } 
}

impl OperationResponse {
    pub fn try_from_stream(stream: &TcpStream) -> Result<OperationResponse, Box<dyn std::error::Error>> {
        let mut de = serde_json::Deserializer::from_reader(stream);
        let resp = OperationResponse::deserialize(&mut de)?;
        Ok(resp)
    }
}


pub fn compute_response(ops: OperationRequest) -> OperationResponse {
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