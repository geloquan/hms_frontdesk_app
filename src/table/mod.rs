use std::sync::{Arc, Mutex, RwLock};
use serde::{Deserialize, Serialize};
use serde_json::json;
use update::UpdateEquipmentRow;

mod update;

use crate::{database::{self, table::*}, DatabaseTable};

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum TableRow {
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
}
#[derive(Debug)]
pub(crate) struct TableData {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySample {
    pub op_id: Option<i32>,
    pub op_label: String,
    pub patient_full_name: String,
    pub op_status: OperationStatus,
    pub room_name: String,
    pub total_tools: i64,
    pub on_site_tools: i64,
    pub on_site_ratio: f64,
    pub on_site_percentage: f64,
    pub start_time: String,
    pub end_time: String
}
impl TableData {
    pub fn query(&mut self) -> Vec<QuerySample> {
        let operation_summaries: Vec<QuerySample> = {
            let operations = self.operation.read().unwrap();
            println!("operation poisoned: {:?}", self.operation);
            let patients = self.patient.read().unwrap();
            println!("patients poisoned: {:?}", self.patient);
            let rooms = self.room.read().unwrap();
            println!("rooms poisoned: {:?}", self.room);
            let operation_tools = self.operation_tool.read().unwrap();
            println!("operation_tools poisoned: {:?}", self.operation_tool);
        
            operations.iter().map(|op| {
                let op_id = op.id;
                let op_label = op.label.clone().unwrap_or_else(|| "N/A".to_string());
                let op_status = op.status.clone().unwrap_or_else(|| OperationStatus::Discharge);
        
                let patient_full_name = patients.iter()
                    .find(|p| op.patient_id.map(|id| id == p.id.unwrap()).unwrap_or(false))
                    .map(|p| format!("{} {}", p.first_name.clone().unwrap_or_else(|| "N/A".to_string()), p.last_name.clone().unwrap_or_else(|| "N/A".to_string()))) // CONCAT operation
                    .unwrap_or_else(|| "N/A".to_string()); 
        
                let room_name = rooms.iter()
                    .find(|r| op.room_id.map(|id| id == r.id.unwrap()).unwrap_or(false))
                    .map(|r| r.name.clone().unwrap_or_else(|| "N/A".to_string()))
                    .unwrap_or_else(|| "N/A".to_string()); 
        
                let total_tools = operation_tools.iter()
                    .filter(|ot| op_id.map(|id| id == ot.operation_id.unwrap()).unwrap_or(false))
                    .count() as i64;
        
                let on_site_tools = operation_tools.iter()
                    .filter(|ot| op_id.map(|id| id == ot.operation_id.unwrap() && match ot.on_site { Some(1) => true, _ => false }).unwrap_or(false))
                    .count() as i64;
        
                let on_site_ratio = if total_tools > 0 {
                    on_site_tools as f64 / total_tools as f64
                } else {
                    0.0
                };
        
                let on_site_percentage = on_site_ratio * 100.0;
        
                let bruh = QuerySample {
                    op_id,
                    op_label,
                    patient_full_name,
                    op_status,
                    room_name,
                    total_tools,
                    on_site_tools,
                    on_site_ratio,
                    on_site_percentage,
                    start_time: op.start_time.clone().unwrap_or_else(|| "N/A".to_string()), 
                    end_time: op.end_time.clone().unwrap_or_else(|| "N/A".to_string()),   
                };
                println!("bruh: {:?}", bruh);
                bruh
            }).collect::<Vec<QuerySample>>()
        };
        operation_summaries
    }
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
        println!("raw_table: {:?}", raw_table);
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
    pub fn update(&self, raw_string: String, database_table: DatabaseTable) {
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
