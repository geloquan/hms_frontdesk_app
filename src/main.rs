use eframe::{egui, App, Frame};
use futures::stream::SplitSink;
use tokio::{runtime::Runtime, sync::mpsc, task};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use std::sync::mpsc::Sender;
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
struct ReceiveMessage {
    status: String,
    message: String,
}
struct MyApp {
    rx: tokio::sync::mpsc::Receiver<String>,
    tx: tokio::sync::mpsc::Sender<String>,
    messages: Vec<String>,
    sender: WsSender,
    receiver: WsReceiver,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        let (tx, rx) = tokio::sync::mpsc::channel(32);
        
        //std::thread::spawn(move || {
        //    let runtime = Runtime::new().unwrap();
        //    runtime.block_on(async {
        //        if let Err(e) = connect_to_websocket("".to_string//()).await {
        //            eprintln!("WebSocket connection error: {}", e);
        //        }
        //    });
        //});
        
        let options = ewebsock::Options::default();
        let (sender, receiver) = ewebsock::connect("ws://127.0.0.15:8080", options).unwrap();

        MyApp {
            rx,
            tx,
            messages: vec![],
            sender,
            receiver
        }
    }
    
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(msg) = self.rx.try_recv() {
            println!("Ok(msg): {:?}", msg);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("WebSocket Messages:");
            for msg in &self.messages {
                ui.label(msg);
            }

            if ui.button("Send Message").clicked() {
                println!("button clicked");
                let tx = self.tx.clone();
                let request = SendMessage {
                    level: "frontdesk".to_string(),
                    method: "initial".to_string(),
                    data: Some(json!({"content": "Hello from button('Send Message')!"})),
                };
            
                let request_json = serde_json::to_string(&request).unwrap();
                self.sender.send(ewebsock::WsMessage::Text(request_json));
            }
        });
    }
}

async fn connect_to_websocket(msg: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("connect_to_websocket()");
    let url = "ws://127.0.0.15:8080";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    
    let (mut write, mut read) = ws_stream.split();

    let request: SendMessage = SendMessage {
        level: "frontdesk".to_string(),
        method: "initial".to_string(),
        data: Some(json!({"content": "Hello from connect_to_websocket()!"})),
    }; 

    let request_json = serde_json::to_string(&request).unwrap();

    write.send(Message::Text(request_json)).await?;

    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if let Ok(text) = msg.to_text() {
                    match serde_json::from_str::<ReceiveMessage>(text) {
                        Ok(parsed_message) => {
                            println!("Parsed message: {:?}", parsed_message);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse message: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("FRONT DESK", native_options, Box::new(|cc| {
        let app = MyApp::new(cc);
        Ok(Box::new(app))
    }));
}