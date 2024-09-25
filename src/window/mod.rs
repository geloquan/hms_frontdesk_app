
pub enum WindowPropertyScope {
    PreOperative(Option<PreOperativeScope>)
    InProgress(Option<InProgressScope>)
}

pub enum PreOperativeScope {
    Default,
    PatientData,
    RoomProperty,
    ToolReady,
    CalendarVisualization
}
#[derive(Default)]
pub struct PreOperativeScopeWindow {
    show: bool,
    enable_scope: bool,
    id_reference: Option<i32>,
    scope: Option<query_return::QueryTable>
}
impl PreOperativeScopeWindow {
    
}

pub enum InProgressScope {
}
#[derive(Default)]
pub struct InProgressScopeWindow {
    show: bool,
    enable_scope: bool,
    id_reference: Option<i32>,
    scope: Option<query_return::QueryTable>
}
