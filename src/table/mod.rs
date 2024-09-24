use std::sync::{Arc, Mutex, RwLock};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{database, DatabaseTable};

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum TableRow {
    Equipment(EquipmentRow)
}
#[derive(Deserialize, Debug, Serialize)]
struct UpdateTableData {
    id: u32,
    new_row_data: database::table::Operation
}

#[derive(Deserialize, Debug, Serialize)]
pub(crate) struct EquipmentRow {
    id: u32,
    string: String
}
#[derive(Debug)]
pub(crate) struct TableData {
    pub equipment: Arc<RwLock<Vec<database::table::Operation>>>
}
impl TableData {
    pub fn new() -> Self {
        let initial_data: Vec<database::table::Operation> = Vec::new();
        TableData {
            equipment: Arc::new(RwLock::new(initial_data)),
        }
    }
    pub fn initialize(&mut self, raw_string: String, database_table: DatabaseTable) {
        match database_table {
            DatabaseTable::Equipment => {
                println!("raw_string: {:?}", raw_string);
                let equipment_rows: Vec<database::table::Operation> = serde_json::from_str(&raw_string).expect("parse error");
                println!("after equipment_rows");
                self.equipment = Arc::new(RwLock::new(equipment_rows));
            }
        }
    }
    pub fn update(&self, raw_string: String, database_table: DatabaseTable) {
        match serde_json::from_str::<UpdateTableData>(&raw_string) {
            Ok(update_table_data) => {
                let mut rows = self.equipment.write().unwrap();
                if let Some(row) = rows.iter_mut().find(|r| r.id.unwrap() == update_table_data.id as i32) {
                    *row = update_table_data.new_row_data;
                } else {
                }
            },
            Err(_) => todo!(),
        }
    }
}
