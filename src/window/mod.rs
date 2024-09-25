use crate::table::query_return::{self, *};

pub enum CentralWindowEnum {
    InProgress,
    PreOperative
}
#[derive(Debug, Default, Clone)]
pub struct CentralWindow {
    pub in_progress: InProgressScopeWindow,
    pub pre_operative: PreOperativeScopeWindow
}
impl CentralWindow {
    pub fn supports_scope(&self, database_table: CentralWindowEnum) -> Option<query_return::QueryTable> {
        match database_table {
            CentralWindowEnum::InProgress => {
                if self.in_progress.enable_scope &&
                !self.in_progress.id_reference.is_none() &&
                !self.in_progress.scope.is_none() {
                    Some(self.in_progress.scope.clone().unwrap())
                } else {
                    None
                }
            },
            CentralWindowEnum::PreOperative => {
                if self.pre_operative.enable_scope &&
                !self.pre_operative.id_reference.is_none() &&
                !self.pre_operative.scope.is_none() {
                    Some(self.pre_operative.scope.clone().unwrap())
                } else {
                    None
                }
            }
        }
    }
}

pub enum WindowPropertyScope {
    PreOperative(Option<PreOperativeScope>),
    InProgress(Option<InProgressScope>)
}

pub enum PreOperativeScope {
    Default,
    PatientData,
    RoomProperty,
    ToolReady,
    CalendarVisualization
}
#[derive(Default, Debug, Clone)]
pub struct PreOperativeScopeWindow {
    pub show: bool,
    pub search_input: String,
    pub enable_scope: bool,
    pub id_reference: Option<i32>,
    pub scope: Option<query_return::QueryTable>
}
impl PreOperativeScopeWindow {
    
}

pub enum InProgressScope {
}
#[derive(Default, Debug, Clone)]
pub struct InProgressScopeWindow {
    pub show: bool,
    pub search_input: String,
    pub enable_scope: bool,
    pub id_reference: Option<i32>,
    pub scope: Option<query_return::QueryTable>
}
