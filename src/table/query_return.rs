pub enum QueryTable {
    PreOperativeDefault(Option<Vec<PreOperativeDefault>>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreOperativeDefault {
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

