
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub enum TableTarget {
    All,
    Equipment,
    Room,
    Tool,
    Staff,
    ToolReservation,
    ToolDesignatedRoom,
    ToolInspector,
    Patient,
    Operation,
    PatientWardRoom,
    PatientWardAssistant,
    OperationStaff,
    OperationTool
}
#[derive(Deserialize, Debug, Serialize)]
pub enum Operation {
    Initialize,
    Update
}
#[derive(Deserialize, Debug, Serialize)]
pub struct ReceiveMessage {
    pub table_name: TableTarget,
    pub operation: Operation,
    pub status_code: String,
    pub data: String,
}