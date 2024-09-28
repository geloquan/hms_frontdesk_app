use crate::{database::{self, table}, table::query_return::{self, *}};
#[derive(Debug)]
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
    pub fn display_last(&self, central_window_enum: CentralWindowEnum) -> Option<&WindowTable> {
        let mut window_table: Option<&WindowTable> = None;
        match central_window_enum {
            CentralWindowEnum::InProgress => {
                
            },
            CentralWindowEnum::PreOperative => {
                if let Some(current) = &self.pre_operative.tree {
                    let mut current_child = &current.child;
                    let current_data = &current.data;

                    if current_child.is_none() {
                        return Some(&current_data);
                    }

                    while let Some(ref next) = current_child {
                        window_table = Some(&next.data); 
                        current_child = &next.child;
                    }
                }
            },
        }
        window_table
    }
    fn remove_innermost_child(child: &mut Option<Box<TableTree>>) {
        if let Some(ref mut child_boxed) = child {
            if let Some(ref mut next_child) = child_boxed.child {
                CentralWindow::remove_innermost_child(&mut child_boxed.child);
            } else {
                *child = None;
            }
        }
    }
    
    pub fn remove_last(&mut self, central_window_enum: CentralWindowEnum) {
        match central_window_enum {
            CentralWindowEnum::InProgress => todo!(),
            CentralWindowEnum::PreOperative => {
                if let Some(holder) = &mut self.pre_operative.tree {
                    CentralWindow::remove_innermost_child(&mut holder.child); 
                }
            },
        }
    }
    fn push_data_innermost_child(child: &mut Box<TableTree>, window_table: WindowTable) {
        if let Some(ref mut child) = child.child {
            if child.child.is_some() {
                CentralWindow::push_data_innermost_child(child, window_table);
            } else {
                child.child = Some(Box::new(TableTree {
                    data: window_table,
                    child: None
                }));
            }
        }
    }
    pub fn push_last(&mut self, central_window_enum: CentralWindowEnum, window_table: WindowTable) {

        match central_window_enum {
            CentralWindowEnum::InProgress => {
            },
            CentralWindowEnum::PreOperative => {
                if let Some(holder) = &mut self.pre_operative.tree {
                    if let Some(ref mut child) = holder.child {
                        if child.child.is_some() {
                            CentralWindow::push_data_innermost_child(child, window_table);
                        } else {
                            child.child = Some(Box::new(TableTree {
                                data: window_table,
                                child: None
                            }));
                        }
                    } else {
                        holder.child = Some(Box::new(TableTree {
                            data: window_table,
                            child: None
                        }));
                    }
                }
            },
        }
    }
    pub fn initial_tree(&mut self, central_window_enum: CentralWindowEnum, window_table: WindowTable) {
        match central_window_enum {
            CentralWindowEnum::InProgress => {
            },
            CentralWindowEnum::PreOperative => {
                self.pre_operative.tree = Some(TableTree {
                    data: window_table,
                    child: None
                });
            },
        }
    }
    pub fn is_root_state(&self, central_window_enum: CentralWindowEnum) -> bool {
        match central_window_enum {
            CentralWindowEnum::InProgress => {
                false
            },
            CentralWindowEnum::PreOperative => {
                if let Some(tree) = &self.pre_operative.tree {
                    if tree.child.is_none() {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
        }
    }
}

pub enum WindowPropertyScope {
    PreOperative(Option<PreOperativeScope>),
    InProgress(Option<InProgressScope>)
}

pub enum PreOperativeScope {
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
    pub tree: Option<query_return::TableTree>,
}
impl PreOperativeScopeWindow {
    pub fn initial_tree(&mut self) {

    }
}

pub enum InProgressScope {
}
#[derive(Default, Debug, Clone)]
pub struct InProgressScopeWindow {
    pub show: bool,
    pub search_input: String,
    pub enable_scope: bool,
    pub id_reference: Option<i32>,
    pub scope: Option<query_return::WindowTable>
}
