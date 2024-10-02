mod table;
mod database;
use database::table::OperationStatus;
use table::{
    query_return::{self, WindowTable::{self, *}}, BuildTable, Query, TableData::{self}
};
mod window;
mod ws;
use ws::receive::*;

pub mod cipher;
use cipher::{decrypt_message, generate_fixed_key, EncryptedText};

use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};
use window::{*};
use eframe::{egui, App, Frame};
use egui::{mutex::Mutex, Color32, Label, RichText, Sense};
use egui_extras::{TableBuilder, Column};
use ewebsock::{self, WsReceiver, WsSender};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Debug, Serialize)]
struct SendMessage {
    level: String,
    method: String,
    data: Option<serde_json::Value>,
}

struct FrontdeskApp {
    data: Option<TableData>,
    rx: tokio::sync::mpsc::Receiver<String>,
    tx: tokio::sync::mpsc::Sender<String>,
    sender: WsSender,
    receiver: WsReceiver,
    central_panel_window_show: CentralWindow,
    //central_window: OperationWindow,
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
            level: "Frontdesk".to_string(),
            method: "Initial".to_string(),
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
            match msg {
                ewebsock::WsEvent::Opened => {
                    
                },
                ewebsock::WsEvent::Message(text) => {
                    println!("text: {:?}", text);
                    match text {
                        ewebsock::WsMessage::Binary(vec) => todo!(),
                        ewebsock::WsMessage::Text(text) => {
                            println!("text: {:?}", text);
                            match serde_json::from_str::<EncryptedText>(&text) {
                                Ok(encrypted_text) => {
                                    if let Ok(key) = &generate_fixed_key() {
                                        if let Ok(decrypted_text) = decrypt_message(key, &encrypted_text.nonce, &encrypted_text.cipher_text) {
                                            match serde_json::from_str::<ReceiveMessage>(&decrypted_text) {
                                                Ok(message) => {
                                                    println!("message: {:?}", message);
                                                    match message.operation {
                                                        Operation::Initialize => {
                                                            if let Some(data) = &mut self.data {
                                                                data.initialize(message.data);
                                                            } else {
                                                                let mut new_table_data = TableData::new();
                                                                new_table_data.initialize(message.data);
                                                                self.data = Some(new_table_data);
                                                                println!("self.data: {:?}", self.data);
                                                            }
                                                        },
                                                        Operation::Update => {},
                                                    }
                                                },
                                                Err(_) => {
                                                    println!("err parsing: ReceiveMessage");
                                                },
                                            }
                                        }
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
                        level: "Frontdesk".to_string(),
                        method: "Initial".to_string(),
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
                let request_json = serde_json::to_string(&SendMessage {
                    level: "Frontdesk".to_string(),
                    method: "Alert".to_string(),
                    data: None,
                }).unwrap();
                self.sender.send(ewebsock::WsMessage::Text(request_json));
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
                let mut window_table = WindowTable::PreOperativeDefault(None);
                if self.central_panel_window_show.pre_operative.tree.is_none() {
                    if let Some(data) = &mut self.data {
                        self.central_panel_window_show.initial_tree(CentralWindowEnum::PreOperative, data.query(&mut WindowTable::PreOperativeDefault(None), None));
                    }
                }
                egui::Window::new("〰 Pre-Operative")
                .id(egui::Id::new("pre_operative")) 
                .resizable(true)
                .constrain(true)
                .collapsible(true)
                .title_bar(true)
                .scroll(false)
                .enabled(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("🔎");
                        ui.text_edit_singleline(&mut self.central_panel_window_show.pre_operative.search_input);
                        if ui.button("help").clicked() {
                            
                        }
                        if !self.central_panel_window_show.is_root_state(CentralWindowEnum::PreOperative) {
                            if ui.button("back").clicked() {
                                println!("backedd");
                                self.central_panel_window_show.remove_last(CentralWindowEnum::PreOperative);
                            }
                        }    
                    });
                    let mut option_data = self.central_panel_window_show.display_last(CentralWindowEnum::PreOperative);
                    if let (Some(option_data), Some(data)) = (&mut option_data, &mut self.data) {
                        TableData::build_table(
                            ui, 
                            option_data.to_owned(),
                            &mut self.central_panel_window_show,
                            data
                        );

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