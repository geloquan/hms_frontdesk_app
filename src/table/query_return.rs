
use serde::{Deserialize, Serialize};

use crate::database;

use super::EquipmentStatus;

#[derive(Debug, Clone)]
pub struct TableTree {
    pub data: WindowTable,
    pub child: Option<Box<TableTree>>
}

#[derive(Debug, Clone)]
pub enum WindowTable {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreOperativeToolReady {
    pub equipment_name: String,
    pub tool_status: crate::database::table::EquipmentStatus,
    pub on_site: bool,
}

