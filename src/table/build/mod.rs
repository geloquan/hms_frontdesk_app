use egui::{Button, Color32, Label, RichText, Sense, Ui};
use egui_extras::{TableBuilder, Column};

use crate::{date_code, format_date, table::TableData};

use super::{query_return::WindowTable, CentralWindow, CentralWindowEnum, OperationStatus, Query};

pub trait BuildTable {
    fn build_table<'a>(ui: &'a mut Ui, window_table: WindowTable, central_window: &mut CentralWindow, data: &mut TableData);
}

impl BuildTable for TableData {
    fn build_table<'a>(ui: &'a mut Ui, window_table: WindowTable, central_window: &mut CentralWindow, data: &mut TableData) -> () {
        let table_return = if let WindowTable::PreOperativeDefault(Some(s)) = &window_table {
            let tbl = TableBuilder::new(ui)
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .header(20.0, |mut header| {
                let headings = [
                    "LABEL",
                    "PATIENT FULL NAME",
                    "ROOM NAME",
                    "TOOLS READY",
                    "STARTING OPERATION",
                    "ENDING OPERATION",
                ];                
                for title in headings {
                    header.col(|ui| {
                        ui.horizontal(|ui|{
                            ui.heading(title);
                        });
                    });
                }
            })
            .body(|mut body| {
                println!("content: {:?}", s);
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
                            if ui.add(Button::new(content.op_label.clone()).sense(Sense::click()).fill(Color32::TRANSPARENT)).clicked() {
                        
                            }
                        });
                        row.col(|ui| {
                            if ui.add(Button::new(content.patient_full_name.clone()).sense(Sense::click()).fill(Color32::TRANSPARENT)).clicked() {
                        
                            }
                        });
                        row.col(|ui| {
                            if ui.add(Button::new(content.room_name.clone()).sense(Sense::click()).fill(Color32::TRANSPARENT)).clicked() {
                        
                            }
                        });
                        row.col(|ui| {
                            if ui.add(Button::new(content.on_site_percentage.clone().to_string()).sense(Sense::click()).fill(Color32::TRANSPARENT)).clicked() {
                                central_window.push_last(CentralWindowEnum::PreOperative, data.query(&mut WindowTable::PreOperativeToolReady(None), content.op_id.clone()));
                            }
                        });
                        row.col(|ui| {
                            let text = RichText::new(format_date(&content.start_time.clone())).color(date_color);
                            if ui.add(Button::new(text).sense(Sense::click()).fill(Color32::TRANSPARENT)).clicked() {
                        
                            }
                        });
                        row.col(|ui| {
                            let text = RichText::new(format_date(&content.end_time.clone())).color(date_color);
                            if ui.add(Button::new(text).sense(Sense::click()).fill(Color32::TRANSPARENT)).clicked() {
                        
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
                            ui.add(Label::new(content.equipment_name.clone()));
                        });
                        row.col(|ui| {
                            let text = RichText::new(if content.on_site { "Yes" } else { "No" })
                            .color(Color32::from_rgb(246, 140, 46))
                            .underline();
                            if ui.add(Button::new(text).sense(Sense::click()).fill(Color32::TRANSPARENT)).clicked() {
                                
                            }
                        });
                        row.col(|ui| {
                            ui.add(Label::new(content.tool_status.clone().to_string()));
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
                        if ui.add(Button::new("N/A").sense(Sense::click()).fill(Color32::TRANSPARENT)).clicked() {
                            
                        }
                    });
                });
            });
            tbl
        };
        table_return
    }
}