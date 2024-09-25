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
#[derive(Default, Debug)]
pub struct PreOperativeScopeWindow {
    pub show: bool,
    pub enable_scope: bool,
    pub id_reference: Option<i32>,
    pub scope: Option<query_return::QueryTable>
}
impl PreOperativeScopeWindow {
    
}

pub enum InProgressScope {
}
#[derive(Default, Debug)]
pub struct InProgressScopeWindow {
    pub show: bool,
    pub enable_scope: bool,
    pub id_reference: Option<i32>,
    pub scope: Option<query_return::QueryTable>
}
