use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Run {
    pub program: String,
    pub args: Vec<String>
}