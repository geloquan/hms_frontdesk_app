
pub struct UpdateRow {
    pub id: i32,
    pub new_row_data: table::TableRow
} 
impl UpdateRow {
    pub fn local(&self, central_window: CentralWindow) {
        match update_row.new_row_data {
            Equipment(equipment) => {
                
            }, 
            Room(room) => {
            
            },           
            Tool(tool) => {
            
            },           
            Staff(staff) => {
            
            },         
            ToolReservation(tool_reservation) => {
            
            }, 
            ToolDesignatedRoom(tool_designated_room) => {
            
            },
            ToolInspector(tool_inspector) => {
            
            },  
            Patient(patient) => {
            
            },              
            Operation(operation) => {
            
            },          
            PatientWardRoom(patient_ward_room) => {
            
            }, 
            PatientWardAssistant(patient_ward_assistant) => {
            
            }, 
            OperationStaff(operation_staff) => {
                
            },  
            OperationTool(operation_tool) => {
                    
            }, 
        }
    }
    pub fn websocket(&self) {
        
    }
}
