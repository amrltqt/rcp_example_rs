
use serde::{Deserialize, Serialize};
use std::net::{TcpStream};


#[derive(Deserialize, Debug)]
pub enum PermittedOperations {
    Sum,
    Mean
}

#[derive(Serialize, Debug)]
pub enum OperationStatus {
    Success,
    Failure(String)
}

#[derive(Deserialize, Debug)]
pub struct OperationRequest {
    pub op: PermittedOperations,
    pub args: Vec<f32>
}

#[derive(Serialize, Debug)]
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
