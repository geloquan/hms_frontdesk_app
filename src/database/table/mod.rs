use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: Option<i32>,
    pub label: Option<String>,
    pub status: Option<String>,
    pub patient_full_name: Option<String>,
    pub room_name: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>
}

