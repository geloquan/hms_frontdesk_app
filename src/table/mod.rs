use std::sync::{Arc, Mutex, RwLock};
use query_return::{PreOperativeDefault, PreOperativeToolReady, WindowTable};
use serde::{Deserialize, Serialize};
use serde_json::json;
use update::UpdateEquipmentRow;

use egui::{Color32, Label, RichText, Sense, Separator, Ui};
use egui_extras::{TableBuilder, Column};

mod update;
pub mod query_return;

use crate::{database::{self, table::{self, *}}, date_code, format_date, window::{self, *}, DatabaseTable};

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
}
#[derive(Debug, Clone)]
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
impl TableData {
    pub fn query(&mut self, window_table: &mut WindowTable, id: Option<i32>) -> WindowTable {
        match window_table {
            WindowTable::PreOperativeDefault(_) => {
                let operations = self.operation.read().unwrap();
                let patients = self.patient.read().unwrap();
                let rooms = self.room.read().unwrap();
                let operation_tools = self.operation_tool.read().unwrap();
            
                let listt: Vec<PreOperativeDefault> = operations.iter().map(|op| {
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
            
                    let bruh = PreOperativeDefault {
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
                    bruh
                }).collect::<Vec<crate::query_return::PreOperativeDefault>>();

                *window_table = WindowTable::PreOperativeDefault(Some(listt));
                window_table.to_owned()
            },
            WindowTable::PreOperativeToolReady(_) => {
                if let Some(operation_id) = id {
                    let operation_tools = self.operation_tool.read().unwrap();
                    let operations = self.operation.read().unwrap();
                    let tools = self.tool.read().unwrap();
                    let equipment = self.equipment.read().unwrap();
                    let patients = self.patient.read().unwrap();
                    

                    let list: Vec<PreOperativeToolReady> = operation_tools.iter()
                        .filter(|op_tool| {
                            operations.iter().any(|op| op.id.unwrap_or_else(|| 0) == op_tool.operation_id.unwrap_or_else(|| -1) && op.id.unwrap_or_else(|| 0) == operation_id)
                        })
                        .map(|op_tool| {
                            println!("op_tool {:?}", op_tool);
                            let tool = tools.iter().find(|t| t.id == op_tool.tool_id);
                            let equipment_item = tool.and_then(|t| equipment.iter().find(|e| e.id == t.info_id));
                            println!("equipment_item {:?}", equipment_item);
                            println!("tools {:?}", tools);
                            println!("tool {:?}", tool);
                            let tool_name = equipment_item.map_or(
                                "Unknown Tool".to_string(), 
                                |e| e.name.clone().unwrap_or_else(|| "N/A".to_string())
                            );
                            let tool_status = tool.map_or(EquipmentStatus::ForInspection, |t| t.status.clone().unwrap_or_else(|| EquipmentStatus::ForInspection));
                            
                            PreOperativeToolReady {
                                equipment_name: tool_name,
                                on_site: op_tool.on_site.map_or(false, |value| value == 1), // Assuming `on_site` is an Option<bool>
                                tool_status,
                            }
                        })
                        .collect();
                    *window_table = WindowTable::PreOperativeToolReady(Some(list));
                    window_table.to_owned()
                } else {
                    window_table.to_owned()
                }
            }
        }
    }
    pub fn build_table<'a>(ui: &'a mut Ui, window_table: WindowTable, central_window: &mut CentralWindow, data: &mut TableData) -> () {
        let table_return = if let WindowTable::PreOperativeDefault(Some(s)) = &window_table {
            let tbl = TableBuilder::new(ui)
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .header(20.0, |mut header| {
                let headings = ["label", "patient full name", "room name", "tools ready", "starting operation", "ending operation"];
                for title in headings {
                    header.col(|ui| {
                        ui.horizontal(|ui|{
                            ui.heading(title);
                        });
                    });
                }
            })
            .body(|mut body| {
                for content in s {
                    if content.op_status.clone() != OperationStatus::PreOperative {
                        continue;
                    }
                    let date_color = date_code(
                        &content.start_time.clone(),
                        &content.end_time.clone()
                    );
                    body.row(30.0, |mut row| {
                
                        row.col(|ui| {
                            if ui.add(Label::new(content.op_label.clone()).sense(Sense::click())).clicked() {
                        
                            }
                        });
                        row.col(|ui| {
                            if ui.add(Label::new(content.patient_full_name.clone()).sense(Sense::click())).clicked() {
                        
                            }
                        });
                        row.col(|ui| {
                            if ui.add(Label::new(content.room_name.clone()).sense(Sense::click())).clicked() {
                        
                            }
                        });
                        row.col(|ui| {
                            if ui.add(Label::new(content.on_site_percentage.clone().to_string()).sense(Sense::click())).clicked() {
                                central_window.push_last(CentralWindowEnum::PreOperative, data.query(&mut WindowTable::PreOperativeToolReady(None), content.op_id.clone()));
                            }
                        });
                        row.col(|ui| {
                            let text = RichText::new(format_date(&content.start_time.clone())).color(date_color);
                            if ui.add(Label::new(text).sense(Sense::click())).clicked() {
                        
                            }
                        });
                        row.col(|ui| {
                            let text = RichText::new(format_date(&content.end_time.clone())).color(date_color);
                            if ui.add(Label::new(text).sense(Sense::click())).clicked() {
                        
                            }
                        });
                    });
                    body.row(0.0, |mut row| {
                        for _ in 0..6 {
                            row.col(|ui| {
                                ui.separator();
                            });
                        }
                    });
                }
            });
            tbl
        } else if let WindowTable::PreOperativeToolReady(Some(s)) = &window_table { 
            let tbl = TableBuilder::new(ui)
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .header(20.0, |mut header| {
                let headings = ["equipment name", "equipment on site", "tool status"];
                for title in headings {
                    header.col(|ui| {
                        ui.horizontal(|ui|{
                            ui.heading(title);
                        });
                    });
                }
            })
            .body(|mut body| {
                for content in s {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            if ui.add(Label::new(content.equipment_name.clone()).sense(Sense::click())).clicked() {
                        
                            }
                        });
                        row.col(|ui| {
                            let text = RichText::new(if content.on_site { "Yes" } else { "No" }).color(Color32::from_rgb(246, 140, 46));
                            if ui.add(Label::new(text).sense(Sense::click())).clicked() {
                        
                            }
                        });
                        row.col(|ui| {
                            if ui.add(Label::new(content.tool_status.clone().to_string()).sense(Sense::click())).clicked() {
                        
                            }
                        });
                    });
                    body.row(0.0, |mut row| {
                        for _ in 0..3 {
                            row.col(|ui| {
                                ui.separator();
                            });
                        }
                    });
                }
            });
            tbl
        } else {
            let tbl = TableBuilder::new(ui)
            .column(Column::auto().resizable(false))
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.horizontal(|ui|{
                        ui.heading("N/A");
                    });
                });
            })
            .body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        if ui.add(Label::new("N/A").sense(Sense::click())).clicked() {
                            
                        }
                    });
                });
            });
            tbl
        };
        table_return
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
