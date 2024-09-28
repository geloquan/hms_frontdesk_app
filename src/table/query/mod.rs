use super::{query_return::{PreOperativeDefault, PreOperativeToolReady, WindowTable}, EquipmentStatus, OperationStatus, TableData};

pub trait Query {
    fn query(&mut self, window_table: &mut WindowTable, id: Option<i32>) -> WindowTable;
}
impl Query for TableData {
    fn query(&mut self, window_table: &mut WindowTable, id: Option<i32>) -> WindowTable {
        match window_table {
            WindowTable::PreOperativeDefault(_) => {
                let operations = self.operation.read().unwrap();
                let patients = self.patient.read().unwrap();
                let rooms = self.room.read().unwrap();
                let operation_tools = self.operation_tool.read().unwrap();
            
                let list: Vec<PreOperativeDefault> = operations.iter().map(|op| {
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

                println!("list {:?}", list);

                *window_table = WindowTable::PreOperativeDefault(Some(list));
                window_table.to_owned()
            },
            WindowTable::PreOperativeToolReady(_) => {
                if let Some(operation_id) = id {
                    let operation_tools = self.operation_tool.read().unwrap();
                    let operations = self.operation.read().unwrap();
                    let tools = self.tool.read().unwrap();
                    let equipment = self.equipment.read().unwrap();

                    let mut list: Vec<PreOperativeToolReady> = operation_tools.iter()
                    .filter(|op_tool| {
                        operations.iter().any(|op| op.id.unwrap_or_else(|| 0) == op_tool.operation_id.unwrap_or_else(|| -1) && op.id.unwrap_or_else(|| 0) == operation_id)
                    })
                    .map(|op_tool| {
                        let tool = tools.iter().find(|t| t.id == op_tool.tool_id);
                        let equipment_item = tool.and_then(|t| equipment.iter().find(|e| e.id == t.info_id));
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
                    list.sort_by(|a, b| {
                        a.on_site.cmp(&b.on_site)
                    });
                    *window_table = WindowTable::PreOperativeToolReady(Some(list));
                    window_table.to_owned()
                } else {
                    window_table.to_owned()
                }
            }
        }
    }
}