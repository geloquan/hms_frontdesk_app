mod table;
use table::{
    TableData
};

use eframe::{egui, App, Frame};
use egui::mutex::Mutex;
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
    Equipment
}
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum Operation {
    Inialize,
    Update
}
#[derive(Deserialize, Debug, Serialize)]
struct ReceiveMessage {
    table_name: DatabaseTable,
    operation: Operation,
    status_code: String,
    data: String,
}
struct FrontdeskApp {
    data: Option<TableData>,
    rx: tokio::sync::mpsc::Receiver<String>,
    tx: tokio::sync::mpsc::Sender<String>,
    messages: Vec<String>,
    sender: WsSender,
    receiver: WsReceiver,
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
            messages: vec![],
            sender,
            receiver
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
                                    match (message.table_name, message.operation) {
                                        (DatabaseTable::Equipment, Operation::Inialize) => {
                                            if let Some(data) = &self.data {
                                                println!("date.initialize()");
                                                data.initialize(message.data, DatabaseTable::Equipment);
                                            } else {
                                                
                                            }
                                        },
                                        (DatabaseTable::Equipment, Operation::Update) => {
                                            if let Some(data) = &self.data {
                                                println!("date.update()");
                                                data.update(message.data, DatabaseTable::Equipment)
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
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("WebSocket Messages:");
            for msg in &self.messages {
                ui.label(msg);
            }
            
            if ui.button("Send Message").clicked() {
                println!("button clicked");
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