use {
    serde::{Serialize, Deserialize},
    std::{
        time::Duration,
        collections::HashMap
    }
};

#[derive(Serialize, Deserialize)]
pub struct StatusReport {
    pub programs: HashMap<String, Status>
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Running(Duration),
    Inactive,
    Failed
}