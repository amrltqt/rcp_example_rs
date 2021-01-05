use crate::models::{OperationRequest, PermittedOperations, OperationResponse, OperationStatus};

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

pub fn sum(items: Vec<f32>) -> f32 {
    items.iter().sum()
}

pub fn mean(items: Vec<f32>) -> f32 {
    items.iter().sum::<f32>() / items.len() as f32
}