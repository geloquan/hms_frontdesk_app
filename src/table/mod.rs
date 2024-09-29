use std::sync::{Arc, Mutex, RwLock};
use query_return::{PreOperativeDefault, PreOperativeToolReady, WindowTable};
use serde::{Deserialize, Serialize};
use serde_json::json;
use update::UpdateEquipmentRow;

use egui_extras::{TableBuilder, Column};

mod update;
pub mod build;
pub mod query;
pub mod query_return;

pub use build::BuildTable;
pub use query::Query;

use crate::{database::{self, table::{self, *}}, date_code, format_date, window::{self, *}, TableTarget};

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TableRow {
    Equipment(Equipment), 
    Room(Room),           
    Tool(Tool),           
    Staff(Staff),         
    ToolReservation(ToolReservation), 
    ToolDesignatedRoom(ToolDesignatedRoom),
    ToolInspector(ToolInspector),  
    Patient(Patient),              
    Operation(Operation),          
    PatientWardRoom(PatientWardRoom), 
    PatientWardAssistant(PatientWardAssistant), 
    OperationStaff(OperationStaff),  
    OperationTool(OperationTool), 
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RawTable {
    pub equipment: Vec<Equipment>,
    pub room: Vec<Room>,
    pub tool: Vec<Tool>,
    pub staff: Vec<Staff>,                          
    pub tool_reservation: Vec<ToolReservation>,     
    pub tool_designated_room: Vec<ToolDesignatedRoom>, 
    pub tool_inspector: Vec<ToolInspector>,        
    pub patient: Vec<Patient>,                     
    pub operation: Vec<Operation>,                 
    pub patient_ward_room: Vec<PatientWardRoom>,   
    pub patient_ward_assistant: Vec<PatientWardAssistant>, 
    pub operation_staff: Vec<OperationStaff>,       
    pub operation_tool: Vec<OperationTool>,
    pub alert: Vec<Alert>,
    pub frontdesk: Vec<OperationStaff>,
    pub alert_frontdesk: Vec<OperationStaff>,
    pub alert_staff: Vec<OperationStaff>   
}
#[derive(Debug, Clone)]
pub struct TableData {
    pub equipment: Arc<RwLock<Vec<database::table::Equipment>>>,
    pub room: Arc<RwLock<Vec<database::table::Room>>>,
    pub tool: Arc<RwLock<Vec<database::table::Tool>>>,
    pub staff: Arc<RwLock<Vec<database::table::Staff>>>,
    pub tool_reservation: Arc<RwLock<Vec<database::table::ToolReservation>>>,
    pub tool_designated_room: Arc<RwLock<Vec<database::table::ToolDesignatedRoom>>>,
    pub tool_inspector: Arc<RwLock<Vec<database::table::ToolInspector>>>,
    pub patient: Arc<RwLock<Vec<database::table::Patient>>>,
    pub operation: Arc<RwLock<Vec<database::table::Operation>>>,
    pub patient_ward_room: Arc<RwLock<Vec<database::table::PatientWardRoom>>>,
    pub patient_ward_assistant: Arc<RwLock<Vec<database::table::PatientWardAssistant>>>,
    pub operation_staff: Arc<RwLock<Vec<database::table::OperationStaff>>>,
    pub operation_tool: Arc<RwLock<Vec<database::table::OperationTool>>>,
}
impl TableData {
    pub fn new() -> Self {
        TableData {
            equipment: Arc::new(RwLock::new(Vec::new())),
            room: Arc::new(RwLock::new(Vec::new())),
            tool: Arc::new(RwLock::new(Vec::new())),
            staff: Arc::new(RwLock::new(Vec::new())),
            tool_reservation: Arc::new(RwLock::new(Vec::new())),
            tool_designated_room: Arc::new(RwLock::new(Vec::new())),
            tool_inspector: Arc::new(RwLock::new(Vec::new())),
            patient: Arc::new(RwLock::new(Vec::new())),
            operation: Arc::new(RwLock::new(Vec::new())),
            patient_ward_room: Arc::new(RwLock::new(Vec::new())),
            patient_ward_assistant: Arc::new(RwLock::new(Vec::new())),
            operation_staff: Arc::new(RwLock::new(Vec::new())),
            operation_tool: Arc::new(RwLock::new(Vec::new())),
        }
    }
    pub fn initialize(&mut self, raw_string: String) {
        let raw_table: RawTable = serde_json::from_str(&raw_string).expect("parse error");
        self.equipment = Arc::new(RwLock::new(raw_table.equipment.clone()));
        self.room = Arc::new(RwLock::new(raw_table.room.clone()));
        self.tool = Arc::new(RwLock::new(raw_table.tool.clone()));
        self.staff = Arc::new(RwLock::new(raw_table.staff.clone()));
        self.tool_reservation = Arc::new(RwLock::new(raw_table.tool_reservation.clone()));
        self.tool_designated_room = Arc::new(RwLock::new(raw_table.tool_designated_room.clone()));
        self.tool_inspector = Arc::new(RwLock::new(raw_table.tool_inspector.clone()));
        self.patient = Arc::new(RwLock::new(raw_table.patient.clone()));
        self.operation = Arc::new(RwLock::new(raw_table.operation.clone()));
        self.patient_ward_room = Arc::new(RwLock::new(raw_table.patient_ward_room.clone()));
        self.patient_ward_assistant = Arc::new(RwLock::new(raw_table.patient_ward_assistant.clone()));
        self.operation_staff = Arc::new(RwLock::new(raw_table.operation_staff.clone()));
        self.operation_tool = Arc::new(RwLock::new(raw_table.operation_tool.clone()));
    }
    pub fn update(&self, raw_string: String, database_table: TableTarget) {
        match serde_json::from_str::<UpdateEquipmentRow>(&raw_string) {
            Ok(update_table_data) => {
                let mut rows = self.equipment.write().unwrap();
                //if let Some(row) = rows.iter_mut().find(|r| r.id.unwrap() == update_table_data.id as i32) {
                //    *row = update_table_data.new_row_data;
                //} else {
                //}
            },
            Err(_) => todo!(),
        }
    }
}

 