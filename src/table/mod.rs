use std::sync::{Arc, Mutex, RwLock};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::DatabaseTable;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum TableRow {
    Equipment(EquipmentRow)
}
#[derive(Deserialize, Debug, Serialize)]
struct UpdateTableData {
    id: u32,
    new_row_data: TableRow
}

#[derive(Deserialize, Debug, Serialize)]
struct EquipmentRow {
    id: u32,
    string: String
}
pub(crate) struct TableData {
    pub equipment: Arc<RwLock<Vec<EquipmentRow>>>
}
impl TableData {
    pub fn initialize(&self, raw_string: String, database_table: DatabaseTable) {
        match database_table {
            DatabaseTable::Equipment => {
                
            }
        }
    }
    pub fn update(&self, raw_string: String, database_table: DatabaseTable) {
        match serde_json::from_str::<UpdateTableData>(&raw_string) {
            Ok(update_table_data) => {
                println!("update_table_data: {:?}", update_table_data);
            },
            Err(_) => todo!(),
        }
        //match database_table {
        //    DatabaseTable::Equipment => {
        //        match serde_json::from_str::<EquipmentRow>(&new_row_value) {
        //            Ok(_) => {
        //            
        //            },
        //            Err(_) => {
        //                
        //            },
        //        }
        //    }
        //}
    }

    pub fn modify_data(&self, id: i32, new_value: String) {
        let mut rows = self.equipment.write().unwrap(); // Only one writer allowed
        if let Some(row) = rows.iter_mut().find(|r| r.id == id as u32) {
            row.string = new_value;
        }
    }
}
