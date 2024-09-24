use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateEquipmentRow {
    pub id: u32,
    pub new_row_data: crate::database::table::Equipment,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateRoomRow {
    pub id: u32,
    pub new_row_data: crate::database::table::Room,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateToolRow {
    pub id: u32,
    pub new_row_data: crate::database::table::Tool,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateStaffRow {
    pub id: u32,
    pub new_row_data: crate::database::table::Staff,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateToolReservationRow {
    pub id: u32,
    pub new_row_data: crate::database::table::ToolReservation,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateToolDesignatedRoomRow {
    pub id: u32,
    pub new_row_data: crate::database::table::ToolDesignatedRoom,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateToolInspectorRow {
    pub id: u32,
    pub new_row_data: crate::database::table::ToolInspector,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdatePatientRow {
    pub id: u32,
    pub new_row_data: crate::database::table::Patient,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateOperationRow {
    pub id: u32,
    pub new_row_data: crate::database::table::Operation,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdatePatientWardRoomRow {
    pub id: u32,
    pub new_row_data: crate::database::table::PatientWardRoom,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdatePatientWardAssistantRow {
    pub id: u32,
    pub new_row_data: crate::database::table::PatientWardAssistant,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateOperationStaffRow {
    pub id: u32,
    pub new_row_data: crate::database::table::OperationStaff,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateOperationToolRow {
    pub id: u32,
    pub new_row_data: crate::database::table::OperationTool,
}
