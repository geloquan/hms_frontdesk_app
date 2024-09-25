mod table;
mod database;
use database::table::OperationStatus;
use table::{
    query_return::{self, QueryTable}, TableData
};

use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};
use window::{InProgressScopeWindow, PreOperativeScopeWindow};
use std::fmt;
use eframe::{egui, App, Frame};
use egui::{mutex::Mutex, Color32, RichText};
use egui_extras::{TableBuilder, Column};
use futures::stream::SplitSink;
use tokio::{runtime::Runtime, sync::mpsc, task};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use std::sync::{mpsc::Sender, Arc};
use url::Url;
use ewebsock::{self, WsReceiver, WsSender};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Debug, Serialize)]
struct SendMessage {
    level: String,
    method: String,
    data: Option<serde_json::Value>,
}
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum DatabaseTable {
    Equipment,
    Room,
    Tool,
    Staff,
    ToolReservation,
    ToolDesignatedRoom,
    ToolInspector,
    Patient,
    Operation,
    PatientWardRoom,
    PatientWardAssistant,
    OperationStaff,
    OperationTool
}
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum Operation {
    Initialize,
    Update
}
#[derive(Deserialize, Debug, Serialize)]
struct ReceiveMessage {
    table_name: DatabaseTable,
    operation: Operation,
    status_code: String,
    data: String,
}
enum CentralWindowEnum {
    InProgress,
    PreOperative
}
mod window;
#[derive(Debug, Default)]
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
                    self.in_progress.scope.unwrap() 
                } else {
                    None
                }
            },
            CentralWindowEnum::PreOperative => {
                if self.pre_operative.enable_scope &&
                !self.pre_operative.id_reference.is_none() &&
                !self.pre_operative.scope.is_none() {
                    self.pre_operative.scope.unwrap() 
                } else {
                    None
                }
            }
        }
    }
}
#[derive(Default)]
struct CentralWindowInput {
    in_progress: String,
    pre_operative: String
}

struct FrontdeskApp {
    data: Option<TableData>,
    rx: tokio::sync::mpsc::Receiver<String>,
    tx: tokio::sync::mpsc::Sender<String>,
    sender: WsSender,
    receiver: WsReceiver,
    central_panel_window_show: CentralWindow,
    //central_window: OperationWindow,
    window_input_ctx: CentralWindowInput
}

fn format_date(input: &str) -> String {
    let naive_datetime = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse date");

    let month = naive_datetime.month();
    let day = naive_datetime.day();
    let year = naive_datetime.year();
    
    let hour = naive_datetime.hour();
    let minute = naive_datetime.minute();

    let month_str = match month {
        1 => "Jan.",
        2 => "Feb.",
        3 => "Mar.",
        4 => "Apr.",
        5 => "May",
        6 => "Jun.",
        7 => "Jul.",
        8 => "Aug.",
        9 => "Sept.",
        10 => "Oct.",
        11 => "Nov.",
        12 => "Dec.",
        _ => unreachable!(),
    };

    let (hour_display, period) = if hour >= 12 {
        (if hour > 12 { hour - 12 } else { 12 }, "PM")
    } else {
        (if hour == 0 { 12 } else { hour }, "AM")
    };

    let time_str = format!("{:02}:{:02}{}", hour_display, minute, period);

    format!("{} {}, {} {}", month_str, day, year, time_str)
}

fn date_code(start: &str, end: &str) -> Color32 {
    let current_time_utc8 = Utc::now() + chrono::Duration::hours(8);

    let start_time = NaiveDateTime::parse_from_str(start, "%Y-%m-%d %H:%M:%S").unwrap();
    let end_time = NaiveDateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S").unwrap();

    if current_time_utc8 < start_time.and_utc() {
        Color32::from_rgb(246, 140, 46)
    } else if current_time_utc8 >= start_time.and_utc() && current_time_utc8 <= end_time.and_utc() {
        Color32::from_rgb(0, 140, 26)
    } else {
        Color32::from_rgb(255, 46, 32)
    }
}
impl FrontdeskApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(32);
        
        let options = ewebsock::Options::default();
        let (mut sender, receiver) = ewebsock::connect("ws://127.0.0.15:8080", options).unwrap();

        let request_json = serde_json::to_string(&SendMessage {
            level: "frontdesk".to_string(),
            method: "initial".to_string(),
            data: Some(json!({"content": "Hello from button('Send Message')!"})),
        }).unwrap();
        sender.send(ewebsock::WsMessage::Text(request_json));

        FrontdeskApp {
            data: None,
            rx,
            tx,
            sender,
            receiver,
            central_panel_window_show: CentralWindow::default(),
            window_input_ctx: CentralWindowInput::default()
        }
    }
    fn toggle_window(&mut self, central_window: CentralWindowEnum) {
        match central_window {
            CentralWindowEnum::InProgress => self.central_panel_window_show.in_progress.show = !self.central_panel_window_show.in_progress.show,
            CentralWindowEnum::PreOperative => self.central_panel_window_show.pre_operative.show = !self.central_panel_window_show.pre_operative.show,

        }
    }
}

impl App for FrontdeskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(msg) = self.receiver.try_recv() {
            println!("Ok(msg): {:?}", msg);
            match msg {
                ewebsock::WsEvent::Opened => {
                    
                },
                ewebsock::WsEvent::Message(text) => {
                    println!("text: {:?}", text);
                    match text {
                        ewebsock::WsMessage::Binary(vec) => todo!(),
                        ewebsock::WsMessage::Text(text) => {
                            match serde_json::from_str::<ReceiveMessage>(&text) {
                                Ok(message) => {
                                    println!("message: {:?}", message);
                                    match message.operation {
                                        Operation::Initialize => {
                                            if let Some(data) = &mut self.data {
                                                println!("message.data: {:?}", message.data);
                                                data.initialize(message.data);
                                            } else {
                                                println!("message.data: {:?}", message.data);
                                                let mut new_table_data = TableData::new();
                                                new_table_data.initialize(message.data);
                                                self.data = Some(new_table_data);
                                            }
                                        },
                                        Operation::Update => {
                                            if let Some(data) = &self.data {
                                                println!("date.update()");
                                                data.update(message.data, DatabaseTable::Equipment)
                                            } else {
                                                let new_table_data = TableData::new();
                                                new_table_data.update(message.data, DatabaseTable::Equipment);
                                                self.data = Some(new_table_data);
                                            }
                                        },
                                    }
                                },
                                Err(_) => {
                                    
                                },
                            }
                        },
                        ewebsock::WsMessage::Unknown(_) => todo!(),
                        ewebsock::WsMessage::Ping(vec) => todo!(),
                        ewebsock::WsMessage::Pong(vec) => todo!(),
                    }
                },
                ewebsock::WsEvent::Error(_) => {
                    let options = ewebsock::Options::default();
                    let (mut sender, receiver) = ewebsock::connect("ws://127.0.0.15:8080", options).unwrap();
                    
                    let request_json = serde_json::to_string(&SendMessage {
                        level: "frontdesk".to_string(),
                        method: "initial".to_string(),
                        data: Some(json!({"content": "Hello from button('Send Message')!"})),
                    }).unwrap();
                    sender.send(ewebsock::WsMessage::Text(request_json));

                    self.sender = sender;
                    self.receiver = receiver;
                },
                ewebsock::WsEvent::Closed => {

                },
            }
        }

        egui::SidePanel::left("left").show(ctx, |ui| {
            ctx.set_pixels_per_point(1.0);
            ui.label("FrontdeskDashboard:");
            ui.collapsing(
                "⚙ Operation", 
                |ui| { 
                    if ui.button("❕ In-progress").clicked() {
                        self.toggle_window(CentralWindowEnum::InProgress);
                    }; 
                    ui.collapsing("☰ Others", |ui| {
                        if ui.button("〰 Pre-Operative").clicked() {
                            self.toggle_window(CentralWindowEnum::PreOperative);
                        }; 
                        let _ = ui.button("⛔ post-operative");
                        let _ = ui.button("✚ recovery");
                        let _ = ui.button("✅ discharge");
                    });
                }
            );

            if ui.button("Send Message").clicked() {
                println!("button clicked");
            }
        });
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.label("Hello World!");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.central_panel_window_show.in_progress.show {
                egui::Window::new("❕ In-progress")
                    .id(egui::Id::new("in_progress")) // unique id for the window
                    .resizable(true)
                    .constrain(true)
                    .collapsible(true)
                    .title_bar(true)
                    .scroll(false)
                    .enabled(true)
                    .show(ctx, |ui| {
                        TableBuilder::new(ui)
                            .column(Column::auto().resizable(false))
                            .column(Column::auto().resizable(false))
                            .column(Column::auto().resizable(false))
                            .column(Column::auto().resizable(false))
                            .column(Column::auto().resizable(false))
                            .header(20.0, |mut header| {
                                let headings = ["label", "patient full name", "room name", "start time", "end time"];
                                for title in headings {
                                    header.col(|ui| {
                                        ui.horizontal(|ui|{
                                            ui.heading(title);
                                            ui.button("🔁");
                                        });
                                    });
                                }
                            })

                            .body(|mut body| {
                                if let Some(table_data) = &self.data {
                                    let rows = table_data.equipment.read().unwrap();
                                    //for content in &*rows {
                                    //    if content.status.clone().unwrap() != "in-progress" {
                                    //        continue;
                                    //    }
                                    //    body.row(30.0, |mut row| {
                                    //        row.col(|ui| {
                                    //            ui.label(content.label.clone().unwrap_or_default());
                                    //        });
                                    //        row.col(|ui| {
                                    //            ui.label(content.patient_full_name.clone().unwrap_or_default());
                                    //        });
                                    //        row.col(|ui| {
                                    //            ui.label(content.room_name.clone().unwrap_or_default());
                                    //        });
                                    //        row.col(|ui| {
                                    //            ui.label(format_date(&content.start_time.clone().unwrap_or_default()));
                                    //        });
                                    //        row.col(|ui| {
                                    //            ui.label(format_date(&content.end_time.clone().unwrap_or_default()));
                                    //        });
                                    //    });
                                    //}
                                }
                            });
                    });
            }
            if self.central_panel_window_show.pre_operative.show {
                egui::Window::new("〰 Pre-Operative")
                .id(egui::Id::new("pre_operative")) // unique id for the window
                .resizable(true)
                .constrain(true)
                .collapsible(true)
                .title_bar(true)
                .scroll(false)
                .enabled(true)
                .show(ctx, |ui| {
                    if let Some(query_table_return) = self.central_panel_window_show.supports_scope(CentralWindowEnum::PreOperative) {
                        if let Some(data) = &self.data {
                            match self.data.unwrap().query(query_table_return) {
                                QueryTable::PreOperativeDefault(Some(s)) => {
                                    
                                }
                                //query_return::PreOperativeDefault(Some(s)) => {
                                //    
                                //},
                                //PreOperativeDefault(None) => {
                                //    
                                //}
                            }
                        }
                    } else {
                        ui.horizontal(|ui| {
                            ui.label("🔎");
                            ui.text_edit_singleline(&mut self.window_input_ctx.pre_operative);
                            if ui.button("help").clicked() {
    
                            }
                        });
                        TableBuilder::new(ui)
                        .column(Column::auto().resizable(false))
                        .column(Column::auto().resizable(false))
                        .column(Column::auto().resizable(false))
                        .column(Column::auto().resizable(false))
                        .column(Column::auto().resizable(false))
                        .column(Column::auto().resizable(false))
                        .header(20.0, |mut header| {
                            let headings = ["label", "patient full name", "room name", "tools ready", "starting operation", "ending operation"];
                            for title in headings {
                                header.col(|ui| {
                                    ui.horizontal(|ui|{
                                        ui.heading(title);
                                    });
                                });
                            }
                        })
                        .body(|mut body| {
                            if let Some(table_data) = &mut self.data {
                                let sample_query = table_data.query();
    
                                for content in sample_query {
                                    if content.op_status.clone() != OperationStatus::PreOperative {
                                        continue;
                                    }
                                    let date_color = date_code(
                                        &content.start_time.clone(),
                                        &content.end_time.clone()
                                    );
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| {
                                            if ui.add(Label::new(content.op_label.clone()).sense(Sense::click())).clicked() {
                                                
                                            }
                                        });
                                        row.col(|ui| {
                                            if ui.add(Label::new(content.patient_full_name.clone()).sense(Sense::click())).clicked() {
                                                
                                            }
                                        });
                                        row.col(|ui| {
                                            if ui.add(Label::new(content.room_name.clone()).sense(Sense::click())).clicked() {
                                                
                                            }
                                        });
                                        row.col(|ui| {
                                            if ui.add(Label::new(content.on_site_percentage.clone()).sense(Sense::click())).clicked() {
    
                                            }
                                        });
                                        row.col(|ui| {
                                            let text = RichText::new(format_date(&content.start_time.clone())).color(date_color);
                                            if ui.add(Label::new(text).sense(Sense::click())).clicked() {
                                                
                                            }
                                        });
                                        row.col(|ui| {
                                            let text = RichText::new(format_date(&content.end_time.clone())).color(date_color);
                                            if ui.add(Label::new(text).sense(Sense::click())).clicked() {
                                                
                                            }
                                        });
                                    });
                                }
                            }
                        });
                    }
                });
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("FRONT DESK", native_options, Box::new(|cc| {
        let app = FrontdeskApp::new(cc);
        Ok(Box::new(app))
    }));
}