use serde::{Deserialize, Serialize};

use crate::database;

#[derive(Debug, Clone)]
pub enum QueryTable {
    PreOperativeDefault(Option<Vec<PreOperativeDefault>>),
    PreOperativeToolReady(Option<Vec<PreOperativeToolReady>>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreOperativeDefault {
    pub op_id: Option<i32>,
    pub op_label: String,
    pub patient_full_name: String,
    pub op_status: database::table::OperationStatus,
    pub room_name: String,
    pub total_tools: i64,
    pub on_site_tools: i64,
    pub on_site_ratio: f64,
    pub on_site_percentage: f64,
    pub start_time: String,
    pub end_time: String
}

