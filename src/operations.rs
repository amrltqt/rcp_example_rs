pub fn sum(items: Vec<f32>) -> f32 {
    items.iter().sum()
}

pub fn mean(items: Vec<f32>) -> f32 {
    items.iter().sum() / items.len() as f32
}